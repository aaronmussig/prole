use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ProleError, ProleResult};

lazy_static! {
    static ref RE_PFAM_LINE: Regex = Regex::new(r"^([^\s]+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+(\d+)\s+(\d+)\s+(\d+)\s+([+-.e\d]+)\s+([+-.e\d]+)\s+([+-.e\d]+)\s+([^\s]+)\s*$").unwrap();
}


/// Methods for loading a PyPfam HMMER output file.
///
/// ## Example
/// An example of the file would appear in the format as follows:
///
/// ```text
/// CAKWUX010000001.1_1       1    263      1    265 PF02896.19  PEP-utilizers_C   Domain    72   292   294    252.7   5.5e-76   1 CL0151
/// CAKWUX010000001.1_10     34    157     33    160 PF14622.7   Ribonucleas_3_3   Family     2   124   128     82.4     4e-24   1 CL0539
/// ```
pub struct PyPfamFile(pub Vec<PyPfamHit>);


impl PyPfamFile {
    /// Read the content from a [BufReader] and parse it into a [PyPfamFile].
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use prole::hmm::pypfam_file::PyPfamFile;
    ///
    /// let reader = BufReader::new(File::open("/path/to/file").unwrap());
    /// let pfam_file = PyPfamFile::from_bufreader(reader).unwrap();
    /// ```
    pub fn from_bufreader<T: std::io::Read>(buf: BufReader<T>) -> ProleResult<Self> {
        let mut out = vec![];
        for line in buf.lines() {
            let line = line.map_err(ProleError::IoError)?;
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            out.push(PyPfamHit::from_string(&line)?);
        }
        Ok(Self(out))
    }

    /// Read the content from a [Path] and parse it into a [PyPfamFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::hmm::pypfam_file::PyPfamFile;
    ///
    /// let path = Path::new("/path/to/file");
    /// let pfam_file = PyPfamFile::from_path(&path).unwrap();
    /// ```
    pub fn from_path(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let reader = BufReader::new(file);
        Self::from_bufreader(reader)
    }

    /// Read the content from a gz compressed file at [Path] and parse it into a [PyPfamFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::hmm::pypfam_file::PyPfamFile;
    ///
    /// let path = Path::new("/path/to/file.gz");
    /// let pfam_file = PyPfamFile::from_path(&path).unwrap();
    /// ```
    pub fn from_path_gz(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let gz = GzDecoder::new(file);
        let reader = BufReader::new(gz);
        Self::from_bufreader(reader)
    }
}


/// A hit within the [PyPfamFile] struct.
pub struct PyPfamHit {
    /// The gene identifier of the target sequence.
    pub seq_id: String,
    /// The position in the target sequence at which the hit starts.
    pub align_start: u32,
    /// The position in the target sequence at which the hit ends.
    pub align_end: u32,
    /// The position in the target sequence at which the surrounding envelope starts.
    pub envelope_start: u32,
    /// The position in the target sequence at which the surrounding envelope ends.
    pub envelope_end: u32,
    /// The accession of the HMM (e.g. `PF02896.19`).
    pub hmm_acc: String,
    /// The name of the HMM (e.g. `PEP-utilizers_C`).
    pub hmm_name: String,
    /// The type of the HMM (e.g. `Domain`).
    pub hmm_type: String,
    /// The position in the hmm at which the hit starts.
    pub hmm_start: u32,
    /// The position in the hmm at which the hit ends.
    pub hmm_end: u32,
    /// The length of the target sequence.
    pub hmm_length: u32,
    /// The score (in bits) for this hit.
    pub bit_score: f64,
    /// The expectation value (statistical significance) of the target.
    pub e_value: f64,
    /// The significance value is true if the bit score for a hit is greater than or equal to the curated gathering threshold for the matching family, false otherwise.
    /// Pfam-B hits are always assigned a significance value of "NA", since Pfam-B families do not have curated thresholds and the value is therefore meaningless.
    pub significance: Option<bool>,
    /// Overlapping hits within clan member families (applies to Pfam-A families only)
    pub clan: String,
}


/// A hit within the [PyPfamFile] struct.
impl PyPfamHit {
    /// Creates a new [PyPfamHit] from an input string.
    ///
    /// ```no_run
    /// use prole::hmm::pypfam_file::PyPfamHit;
    ///
    /// let input_string = "...";
    /// let hit = PyPfamHit::from_string(input_string).unwrap();
    /// ```
    pub fn from_string(string: &str) -> ProleResult<Self> {
        let hits = RE_PFAM_LINE.captures(string)
            .ok_or_else(|| ProleError::Exit(format!("Error parsing: {}", string)))?;
        let significance: Option<bool> = if &hits[14] == "NA" {
            None
        } else {
            Some(&hits[14] == "1")
        };
        Ok(Self {
            seq_id: hits[1].to_string(),
            align_start: hits[2].parse().map_err(ProleError::ParseIntError)?,
            align_end: hits[3].parse().map_err(ProleError::ParseIntError)?,
            envelope_start: hits[4].parse().map_err(ProleError::ParseIntError)?,
            envelope_end: hits[5].parse().map_err(ProleError::ParseIntError)?,
            hmm_acc: hits[6].to_string(),
            hmm_name: hits[7].to_string(),
            hmm_type: hits[8].to_string(),
            hmm_start: hits[9].parse().map_err(ProleError::ParseIntError)?,
            hmm_end: hits[10].parse().map_err(ProleError::ParseIntError)?,
            hmm_length: hits[11].parse().map_err(ProleError::ParseIntError)?,
            bit_score: hits[12].parse().map_err(ProleError::ParseFloatError)?,
            e_value: hits[13].parse().map_err(ProleError::ParseFloatError)?,
            significance,
            clan: hits[15].to_string(),
        })
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
        writeln!(file, "#to be ignored").unwrap();
        writeln!(file, "CAKWUX010000001.1_1       1    263      1    265 PF02896.19  PEP-utilizers_C   Domain    72   292   294    252.7   5.5e-76   1 CL0151 ").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "CAKWUX010000001.1_10     34    157     33    160 PF14622.7   Ribonucleas_3_3   Family     2   124   128     82.4     4e-24   1 CL0539  ").unwrap();

        let result = PyPfamFile::from_path(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(&result.0[0].seq_id, "CAKWUX010000001.1_1");
        assert_eq!(&result.0[1].seq_id, "CAKWUX010000001.1_10");
    }

    #[test]
    fn test_from_path_gz_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(b"#to be ignored\n").unwrap();
        e.write_all(b"CAKWUX010000001.1_1       1    263      1    265 PF02896.19  PEP-utilizers_C   Domain    72   292   294    252.7   5.5e-76   1 CL0151 \n").unwrap();
        e.write_all(b"\n").unwrap();
        e.write_all(b"CAKWUX010000001.1_10     34    157     33    160 PF14622.7   Ribonucleas_3_3   Family     2   124   128     82.4     4e-24   1 CL0539  \n").unwrap();
        let compressed_bytes = e.finish().unwrap();
        file.write_all(&compressed_bytes).unwrap();

        let result = PyPfamFile::from_path_gz(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(&result.0[0].seq_id, "CAKWUX010000001.1_1");
        assert_eq!(&result.0[1].seq_id, "CAKWUX010000001.1_10");
    }

    #[test]
    fn test_from_path_invalid_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "CAKWUX010000001.1_1       1    263      1    265 ").unwrap();
        let result = PyPfamFile::from_path(&file.into_temp_path());
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string_valid() {
        let string = "CAKWUX010000001.1_1       1    263      2    265 PF02896.19  PEP-utilizers_C   Domain    72   292   294    252.7   5.5e-76   1 CL0151 ";
        let result = PyPfamHit::from_string(string);
        assert!(result.is_ok());
        let hit = result.unwrap();
        assert_eq!(hit.seq_id, "CAKWUX010000001.1_1");
        assert_eq!(hit.align_start, 1);
        assert_eq!(hit.align_end, 263);
        assert_eq!(hit.envelope_start, 2);
        assert_eq!(hit.envelope_end, 265);
        assert_eq!(hit.hmm_acc, "PF02896.19");
        assert_eq!(hit.hmm_name, "PEP-utilizers_C");
        assert_eq!(hit.hmm_type, "Domain");
        assert_eq!(hit.hmm_start, 72);
        assert_eq!(hit.hmm_end, 292);
        assert_eq!(hit.hmm_length, 294);
        assert_eq!(hit.bit_score, 252.7);
        assert_eq!(hit.e_value, 5.5e-76);
        assert_eq!(hit.significance, Some(true));
        assert_eq!(hit.clan, "CL0151");
    }

    #[test]
    fn test_from_string_invalid_format() {
        let string = "CAKWUX010000001.1_1       1    263      1    ";
        let result = PyPfamHit::from_string(string);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string_empty() {
        let string = "";
        let result = PyPfamHit::from_string(string);
        assert!(result.is_err());
    }
}
