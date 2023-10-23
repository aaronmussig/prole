use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ProleError, ProleResult};

/// Methods for loading a HMMER output file formatted using `--tblout`.
///
/// ## Example
/// An example of the file would appear in the format as follows:
///
/// ```text
/// CAKWUX010000027.1_18 -          TIGR00001            TIGR00001    1.9e-26   89.3   7.9   2.1e-26   89.2   7.9   1.0   1   0   0   1   1   1   1 # 15227 # 15421 # -1 # ID=27_18;partial=00;start_type=ATG;rbs_motif=None;rbs_spacer=None;gc_cont=0.492
/// CAKWUX010000058.1_8  -          TIGR00002            TIGR00002    4.7e-30  100.6   0.0   8.2e-30   99.8   0.0   1.4   1   0   0   1   1   1   1 # 6333 # 6881 # -1 # ID=58_8;partial=00;start_type=ATG;rbs_motif=AAAA;rbs_spacer=11bp;gc_cont=0.599
/// ```
pub struct HmmSearchFile(pub Vec<HmmSearchHit>);


impl HmmSearchFile {
    /// Read the content from a [BufReader] and parse it into a [HmmSearchFile].
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use prole::hmm::hmmsearch_file::HmmSearchFile;
    ///
    /// let reader = BufReader::new(File::open("/path/to/file").unwrap());
    /// let pfam_file = HmmSearchFile::from_bufreader(reader).unwrap();
    /// ```
    pub fn from_bufreader<T: std::io::Read>(buf: BufReader<T>) -> ProleResult<Self> {
        let mut out = vec![];
        for line in buf.lines() {
            let line = line.map_err(ProleError::IoError)?;
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            out.push(HmmSearchHit::from_string(&line)?);
        }
        Ok(Self(out))
    }

    /// Read the content from a [Path] and parse it into a [HmmSearchFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::hmm::hmmsearch_file::HmmSearchFile;
    ///
    /// let path = Path::new("/path/to/file");
    /// let pfam_file = HmmSearchFile::from_path(&path).unwrap();
    /// ```
    pub fn from_path(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let reader = BufReader::new(file);
        Self::from_bufreader(reader)
    }

    /// Read the content from a gz compressed file at [Path] and parse it into a [HmmSearchFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::hmm::hmmsearch_file::HmmSearchFile;
    ///
    /// let path = Path::new("/path/to/file.gz");
    /// let pfam_file = HmmSearchFile::from_path(&path).unwrap();
    /// ```
    pub fn from_path_gz(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let gz = GzDecoder::new(file);
        let reader = BufReader::new(gz);
        Self::from_bufreader(reader)
    }
}


/// A hit within the [HmmSearchFile] struct.
pub struct HmmSearchHit {
    /// The name of the target sequence or profile.
    pub target_name: String,
    /// The accession of the target sequence or profile.
    pub target_accession: Option<String>,
    /// The name of the query sequence or profile.
    pub query_name: String,
    /// The accession of the query sequence or profile.
    pub query_accession: Option<String>,
    /// The expectation value (statistical significance) of the target.
    pub full_seq_evalue: f64,
    /// The score (in bits) for this hit.
    pub full_seq_score: f64,
    ///  The biased-composition correction: the bit score difference contributed by the null2 model.
    pub full_seq_bias: f64,
    /// The E-value if only the single best-scoring domain envelope were found in the sequence,
    /// and none of the others.
    pub best_domain_evalue: f64,
    /// The bit score if only the single best-scoring domain envelope were found in the sequence,
    /// and none of the others.
    pub best_domain_score: f64,
    /// The null2 bias correction that was applied to the bit score of the single best-scoring domain.
    pub best_domain_bias: f64,
    /// Expected number of domains, as calculated by posterior decoding on the mean number of begin
    /// states used in the alignment ensemble.
    pub exp: f64,
    /// Number of discrete regions defined, as calculated by heuristics applied to posterior
    /// decoding of begin/end state positions in the alignment ensemble.
    pub reg: u32,
    /// Number of regions that appeared to be multidomain, and therefore were passed to stochastic
    /// traceback clustering for further resolution down to one or more envelopes.
    pub clu: u32,
    /// For envelopes that were defined by stochastic traceback clustering, how many of them
    /// overlap other envelopes.
    pub ov: u32,
    /// The total number of envelopes defined, both by single envelope regions and by stochastic
    /// traceback clustering into one or more envelopes per region.
    pub env: u32,
    /// Number of domains defined.
    pub dom: u32,
    /// Number of domains satisfying reporting thresholds.
    pub rep: u32,
    /// Number of domains satisfying inclusion thresholds.
    pub inc: u32,
    /// Target’s description line, as free text
    pub description: String,
}


lazy_static! {
    static ref RE_HMM_SEARCH_LINE: Regex = Regex::new(r"^([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+([\d.e+-]+)\s+([\d.e+-]+)\s+([\d.e+-]+)\s+([\d.e+-]+)\s+([\d.e+-]+)\s+([\d.e+-]+)\s+([\d.e+-]+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(.+)$").unwrap();
}

/// A hit within the [HmmSearchHit] struct.
impl HmmSearchHit {
    /// Creates a new [HmmSearchHit] from an input string.
    ///
    /// ```no_run
    /// use prole::hmm::hmmsearch_file::HmmSearchHit;
    ///
    /// let input_string = "...";
    /// let hit = HmmSearchHit::from_string(input_string).unwrap();
    /// ```
    pub fn from_string(string: &str) -> ProleResult<Self> {
        let hits = RE_HMM_SEARCH_LINE.captures(string)
            .ok_or_else(|| ProleError::Exit(format!("Error parsing: {}", string)))?;

        let target_accession = if &hits[2] == "-" {
            None
        } else {
            Some(hits[2].to_string())
        };
        let query_accession = if &hits[4] == "-" {
            None
        } else {
            Some(hits[4].to_string())
        };

        // target_accession, query_accession = '-' if none
        Ok(Self {
            target_name: hits[1].to_string(),
            target_accession,
            query_name: hits[3].to_string(),
            query_accession,
            full_seq_evalue: hits[5].parse().map_err(ProleError::ParseFloatError)?,
            full_seq_score: hits[6].parse().map_err(ProleError::ParseFloatError)?,
            full_seq_bias: hits[7].parse().map_err(ProleError::ParseFloatError)?,
            best_domain_evalue: hits[8].parse().map_err(ProleError::ParseFloatError)?,
            best_domain_score: hits[9].parse().map_err(ProleError::ParseFloatError)?,
            best_domain_bias: hits[10].parse().map_err(ProleError::ParseFloatError)?,
            exp: hits[11].parse().map_err(ProleError::ParseFloatError)?,
            reg: hits[12].parse().map_err(ProleError::ParseIntError)?,
            clu: hits[13].parse().map_err(ProleError::ParseIntError)?,
            ov: hits[14].parse().map_err(ProleError::ParseIntError)?,
            env: hits[15].parse().map_err(ProleError::ParseIntError)?,
            dom: hits[16].parse().map_err(ProleError::ParseIntError)?,
            rep: hits[17].parse().map_err(ProleError::ParseIntError)?,
            inc: hits[18].parse().map_err(ProleError::ParseIntError)?,
            description: hits[19].to_string(),
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
        writeln!(file, "CAKWUX010000001.1_73 -          TIGR00046            TIGR00046    7.9e-36  120.7   0.0   9.6e-36  120.4   0.0   1.0   1   0   0   1   1   1   1 # 101713 # 102426 # 1 # ID=1_73;partial=00;start_type=ATG;rbs_motif=AATAA;rbs_spacer=13bp;gc_cont=0.651").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "CAKWUX010000041.1_17 -          TIGR00054            TIGR00054    8.9e-62  206.4   0.0   1.1e-61  206.0   0.0   1.0   1   0   0   1   1   1   1 # 20284 # 21807 # 1 # ID=41_17;partial=01;start_type=GTG;rbs_motif=AAA;rbs_spacer=11bp;gc_cont=0.583").unwrap();

        let result = HmmSearchFile::from_path(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(&result.0[0].target_name, "CAKWUX010000001.1_73");
        assert_eq!(&result.0[1].target_name, "CAKWUX010000041.1_17");
    }

    #[test]
    fn test_from_path_gz_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(b"#to be ignored\n").unwrap();
        e.write_all(b"CAKWUX010000001.1_73 -          TIGR00046            TIGR00046    7.9e-36  120.7   0.0   9.6e-36  120.4   0.0   1.0   1   0   0   1   1   1   1 # 101713 # 102426 # 1 # ID=1_73;partial=00;start_type=ATG;rbs_motif=AATAA;rbs_spacer=13bp;gc_cont=0.651\n").unwrap();
        e.write_all(b"\n").unwrap();
        e.write_all(b"CAKWUX010000041.1_17 -          TIGR00054            TIGR00054    8.9e-62  206.4   0.0   1.1e-61  206.0   0.0   1.0   1   0   0   1   1   1   1 # 20284 # 21807 # 1 # ID=41_17;partial=01;start_type=GTG;rbs_motif=AAA;rbs_spacer=11bp;gc_cont=0.583\n").unwrap();
        let compressed_bytes = e.finish().unwrap();
        file.write_all(&compressed_bytes).unwrap();

        let result = HmmSearchFile::from_path_gz(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(&result.0[0].target_name, "CAKWUX010000001.1_73");
        assert_eq!(&result.0[1].target_name, "CAKWUX010000041.1_17");
    }

    #[test]
    fn test_from_path_invalid_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "CAKWUX010000027.1_18 -          TIGR00001            TIGR00001    1.9e-26").unwrap();
        let result = HmmSearchFile::from_path(&file.into_temp_path());
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string_valid() {
        let string = "CAKWUX010000027.1_18 -          TIGR00001            TIGR00001    1.9e-26   89.3   7.9   2.1e-26   89.2   7.9   1.0   1   0   0   1   1   1   1 # 15227 # 15421 # -1 # ID=27_18;partial=00;start_type=ATG;rbs_motif=None;rbs_spacer=None;gc_cont=0.492";
        let result = HmmSearchHit::from_string(string);
        assert!(result.is_ok());
        let hit = result.unwrap();
        assert_eq!(hit.target_name, "CAKWUX010000027.1_18");
        assert_eq!(hit.target_accession, None);
        assert_eq!(hit.query_name, "TIGR00001");
        assert_eq!(hit.query_accession, Some("TIGR00001".to_string()));
        assert_eq!(hit.full_seq_evalue, 1.90e-26);
        assert_eq!(hit.full_seq_score, 89.3);
        assert_eq!(hit.full_seq_bias, 7.9);
        assert_eq!(hit.best_domain_evalue, 2.1e-26);
        assert_eq!(hit.best_domain_score, 89.2);
        assert_eq!(hit.best_domain_bias, 7.9);
        assert_eq!(hit.exp, 1.0);
        assert_eq!(hit.reg, 1);
        assert_eq!(hit.clu, 0);
        assert_eq!(hit.ov, 0);
        assert_eq!(hit.env, 1);
        assert_eq!(hit.dom, 1);
        assert_eq!(hit.rep, 1);
        assert_eq!(hit.inc, 1);
        assert_eq!(hit.description, "# 15227 # 15421 # -1 # ID=27_18;partial=00;start_type=ATG;rbs_motif=None;rbs_spacer=None;gc_cont=0.492");
    }

    #[test]
    fn test_from_string_valid_2() {
        let string = "DEJT01000119.1_4     -          TIGR04114            TIGR04114    3.7e-05   20.9  53.4     2e+03  -17.7  53.4   3.2   1   1   0   1   1   0   0 # 2754 # 3044 # 1 # ID=58_4;partial=00;start_type=ATG;rbs_motif=TAAAAA;rbs_spacer=4bp;gc_cont=0.471";
        let result = HmmSearchHit::from_string(string);
        assert!(result.is_ok());
        let hit = result.unwrap();
        assert_eq!(hit.target_name, "DEJT01000119.1_4");
        assert_eq!(hit.target_accession, None);
        assert_eq!(hit.query_name, "TIGR04114");
        assert_eq!(hit.query_accession, Some("TIGR04114".to_string()));
        assert_eq!(hit.full_seq_evalue, 3.7e-05);
        assert_eq!(hit.full_seq_score, 20.9);
        assert_eq!(hit.full_seq_bias, 53.4);
        assert_eq!(hit.best_domain_evalue, 2e+03);
        assert_eq!(hit.best_domain_score, -17.7);
        assert_eq!(hit.best_domain_bias, 53.4);
        assert_eq!(hit.exp, 3.2);
        assert_eq!(hit.reg, 1);
        assert_eq!(hit.clu, 1);
        assert_eq!(hit.ov, 0);
        assert_eq!(hit.env, 1);
        assert_eq!(hit.dom, 1);
        assert_eq!(hit.rep, 0);
        assert_eq!(hit.inc, 0);
        assert_eq!(hit.description, "# 2754 # 3044 # 1 # ID=58_4;partial=00;start_type=ATG;rbs_motif=TAAAAA;rbs_spacer=4bp;gc_cont=0.471");
    }

    #[test]
    fn test_from_string_invalid_format() {
        let string = "CAKWUX010000001.1_1       1    263      1    ";
        let result = HmmSearchHit::from_string(string);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string_empty() {
        let string = "";
        let result = HmmSearchHit::from_string(string);
        assert!(result.is_err());
    }
}
