use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::error::{ProleError, ProleResult};
use crate::genome::genome_id::GenomeId;

/// This struct wraps the GTDB genome_dirs.tsv file.
///
/// An example of the content is as follows:
/// ```text
/// GCA_934854595.1	/srv/db/gtdb/genomes/ncbi/release214/genbank/GCA/934/854/595/GCA_934854595.1_MTG237_bin.38.fa	G934854595
/// GCA_934854545.1	/srv/db/gtdb/genomes/ncbi/release214/genbank/GCA/934/854/545/GCA_934854545.1_MTG236_bin.31.fa	G934854545
/// GCA_934854535.1	/srv/db/gtdb/genomes/ncbi/release214/genbank/GCA/934/854/535/GCA_934854535.1_MTG234_bin.48.fa	G934854535
/// ```
pub struct GenomeDirsFile(pub HashMap<GenomeId, PathBuf>);

impl GenomeDirsFile {
    /// Load the [GenomeDirsFile] from the specified [Path].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::gtdb::genome_dirs_file::GenomeDirsFile;
    ///
    /// let path = Path::new("/path/to/file");
    /// let out = GenomeDirsFile::load(&path).unwrap();
    /// ```
    pub fn load(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let reader = io::BufReader::new(file);

        let mut out = HashMap::new();
        for line in reader.lines() {
            let line = line.map_err(ProleError::IoError)?;
            let line_split = line.split('\t').collect::<Vec<&str>>();
            let genome_id = GenomeId(line_split[0].to_string());
            let genome_dir = PathBuf::from(line_split[1]);
            out.insert(genome_id, genome_dir);
        }
        Ok(Self(out))
    }

    /// Returns the path for the specified [GenomeId].
    pub fn get_path(&self, genome_id: &GenomeId) -> Option<&PathBuf> {
        self.0.get(genome_id)
    }
}


#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_from_path_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "GCA_934854595.1	/tmp/a	G934854595").unwrap();
        writeln!(file, "GCA_934854545.1	/tmp/b/b	G934854545").unwrap();
        writeln!(file, "GCA_934854535.1	/c	G934854535").unwrap();

        let result = GenomeDirsFile::load(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get_path(&GenomeId("GCA_934854595.1".to_string())), Some(PathBuf::from("/tmp/a")).as_ref());
        assert_eq!(result.get_path(&GenomeId("GCA_934854545.1".to_string())), Some(PathBuf::from("/tmp/b/b")).as_ref());
        assert_eq!(result.get_path(&GenomeId("GCA_934854535.1".to_string())), Some(PathBuf::from("/c")).as_ref());
    }
}
