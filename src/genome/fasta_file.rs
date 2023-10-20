use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use bio::io::fasta;
use flate2::read::GzDecoder;

use crate::error::{ProleError, ProleResult};

/// A genome fasta formatted file. Sequence IDs must be unique.
pub struct FastaFile(pub HashMap<String, (Option<String>, String)>);

impl FastaFile {
    /// Read the content from a [BufReader] and parse it into a [FastaFile].
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use prole::genome::fasta_file::FastaFile;
    ///
    /// let reader = BufReader::new(File::open("/path/to/file").unwrap());
    /// let file = FastaFile::from_bufreader(reader).unwrap();
    /// ```
    pub fn from_bufreader<T: std::io::Read>(buf: BufReader<T>) -> ProleResult<Self> {
        let mut reader = fasta::Reader::new(buf).records();
        let mut out = HashMap::new();
        while let Some(Ok(record)) = reader.next() {
            let id = record.id().to_string();
            if out.contains_key(&id) {
                return Err(ProleError::Exit("Duplicate ID found in fasta output: ".to_string()));
            }
            let desc = record.desc().map(|x| x.to_string());
            let mut seq = String::from_utf8(record.seq().to_vec()).map_err(ProleError::Utf8Error)?;
            if seq.ends_with("*") {
                seq = seq[..seq.len() - 1].to_string();
            }
            out.insert(id, (desc, seq));
        }

        Ok(Self(out))
    }

    /// Read the content from a [Path] and parse it into a [FastaFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::genome::fasta_file::FastaFile;
    ///
    /// let path = Path::new("tests/data/contigs.fa");
    /// let fasta = FastaFile::from_path(path).unwrap();
    /// ```
    pub fn from_path(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let reader = BufReader::new(file);
        Self::from_bufreader(reader)
    }

    /// Read the content from a gz compressed file at [Path] and parse it into a [FastaFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::genome::fasta_file::FastaFile;
    ///
    /// let path = Path::new("tests/data/contigs.fa.gz");
    /// let fasta = FastaFile::from_path_gz(path).unwrap();
    /// ```
    pub fn from_path_gz(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let gz = GzDecoder::new(file);
        let reader = BufReader::new(gz);
        Self::from_bufreader(reader)
    }

    /// Return the length of the sequence stored at `contig`.
    pub fn get_length(&self, contig: &str) -> Option<usize> {
        if let Some(seq) = self.get_sequence(contig) {
            return Some(seq.len());
        }
        None
    }

    /// Return the sequence stored at `contig`.
    pub fn get_sequence(&self, contig: &str) -> Option<&str> {
        let hit = self.0.get(contig);
        if let Some(hit) = hit {
            return Some(hit.1.as_str());
        }
        None
    }

    /// Return the description stored at `contig`.
    pub fn get_description(&self, config: &str) -> Option<&str> {
        if let Some((desc, _seq)) = self.0.get(config) {
            return desc.as_ref().map(|x| x.as_str());
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::Compression;
    use flate2::write::GzEncoder;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_from_path_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, ">foo desc1").unwrap();
        writeln!(file, "ATGATG").unwrap();
        writeln!(file, ">bar desc2").unwrap();
        writeln!(file, "CCGGTTAA").unwrap();

        let result = FastaFile::from_path(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(result.get_sequence("foo").unwrap(), "ATGATG");
        assert_eq!(result.get_description("foo").unwrap(), "desc1");
        assert_eq!(result.get_sequence("bar").unwrap(), "CCGGTTAA");
        assert_eq!(result.get_description("bar").unwrap(), "desc2");
    }

    #[test]
    fn test_from_path_gz_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(b">foo desc1\nATGATG\n").unwrap();
        e.write_all(b">bar desc2\nCCGGTTAA\n").unwrap();
        let compressed_bytes = e.finish().unwrap();
        file.write_all(&compressed_bytes).unwrap();

        let result = FastaFile::from_path_gz(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(result.get_sequence("foo").unwrap(), "ATGATG");
        assert_eq!(result.get_description("foo").unwrap(), "desc1");
        assert_eq!(result.get_sequence("bar").unwrap(), "CCGGTTAA");
        assert_eq!(result.get_description("bar").unwrap(), "desc2");
    }
}
