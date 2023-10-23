use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ProleError, ProleResult};

lazy_static! {
    static ref RE_GR: Regex = Regex::new(r"^#=GR ([^\s]+)\s+PP\s+([^\s]+)$").unwrap();
    static ref RE_PP_CONS: Regex = Regex::new(r"^#=GC PP_cons\s+([^\s]+)$").unwrap();
    static ref RE_GC_RF: Regex = Regex::new(r"^#=GC RF\s+([^\s]+)$").unwrap();
    static ref RE_ALIGN: Regex = Regex::new(r"^([^\s]+)\s+([^\s]+)$").unwrap();
}

/// Wraps the output of a HMMER alignment file.
pub struct HmmAlignFile {
    pub seq: HashMap<String, String>,
    pub pp: HashMap<String, String>,
    pub pp_cons: String,
    pub mask: Vec<bool>,
    pub mask_idx: Vec<usize>,
}

impl HmmAlignFile {
    /// Read the content from a [BufReader] and parse it into a [HmmAlignFile].
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use prole::hmm::hmmalign_file::HmmAlignFile;
    ///
    /// let reader = BufReader::new(File::open("/path/to/file").unwrap());
    /// let _ = HmmAlignFile::from_bufreader(reader).unwrap();
    /// ```
    pub fn from_bufreader<T: std::io::Read>(buf: BufReader<T>) -> ProleResult<Self> {
        let mut seq = HashMap::new();
        let mut pp = HashMap::new();
        let mut pp_cons = String::new();
        let mut mask = vec![];
        let mut mask_idx = vec![];

        for line in buf.lines() {
            let line = line.map_err(ProleError::IoError)?;
            if line.is_empty() || line.starts_with("# STOCKHOLM") || line.starts_with("//") {
                continue;
            } else if line.starts_with("#=GR ") {
                // Within pp for genome
                let hits = RE_GR.captures(&line)
                    .ok_or_else(|| ProleError::Exit(format!("Error parsing: {}", line)))?;
                let gene_id = hits[1].to_string();
                if pp.contains_key(&gene_id) {
                    return Err(ProleError::Exit(format!("Duplicate: {}", line)));
                }
                pp.insert(gene_id, hits[2].to_string());
            } else if line.starts_with("#=GC PP_cons") {
                // Within conserved
                let hits = RE_PP_CONS.captures(&line)
                    .ok_or_else(|| ProleError::Exit(format!("Error parsing: {}", line)))?;
                if !pp_cons.is_empty() {
                    return Err(ProleError::Exit(format!("Duplicate: {}", line)));
                }
                pp_cons = hits[1].to_string();
            } else if line.starts_with("#=GC RF") {
                // Within mask
                let hits = RE_GC_RF.captures(&line)
                    .ok_or_else(|| ProleError::Exit(format!("Error parsing: {}", line)))?;
                if !mask.is_empty() {
                    return Err(ProleError::Exit(format!("Duplicate: {}", line)));
                }
                for (idx, char) in hits[1].chars().enumerate() {
                    if char == 'x' {
                        mask.push(true);
                        mask_idx.push(idx);
                    } else {
                        mask.push(false);
                    }
                }
            } else {
                // Within genome alignment
                let hits = RE_ALIGN.captures(&line)
                    .ok_or_else(|| ProleError::Exit(format!("Error parsing: {}", line)))?;
                let gene_id = hits[1].to_string();
                if seq.contains_key(&gene_id) {
                    return Err(ProleError::Exit(format!("Duplicate: {}", line)));
                }
                seq.insert(gene_id, hits[2].to_string());
            }
        }

        // Validate all components were read
        if seq.is_empty() {
            return Err(ProleError::Exit("Missing seq".to_string()));
        }
        if pp.is_empty() {
            return Err(ProleError::Exit("Missing pp".to_string()));
        }
        if seq.len() != pp.len() {
            return Err(ProleError::Exit("Seq and pp have different lengths".to_string()));
        }
        if pp_cons.is_empty() {
            return Err(ProleError::Exit("Missing PP_cons".to_string()));
        }
        if mask.is_empty() || mask_idx.is_empty() {
            return Err(ProleError::Exit("Missing mask".to_string()));
        }

        // All ok
        Ok(Self { seq, pp, pp_cons, mask, mask_idx })
    }

    /// Read the content from a [Path] and parse it into a [HmmAlignFile].
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::hmm::hmmalign_file::HmmAlignFile;
    ///
    /// let _ = HmmAlignFile::from_path(&Path::new("/path/to/file")).unwrap();
    /// ```
    pub fn from_path(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let reader = BufReader::new(file);
        Self::from_bufreader(reader)
    }

    /// Return the masked alignment for a given gene.
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use prole::hmm::hmmalign_file::HmmAlignFile;
    ///
    /// let alignment = HmmAlignFile::from_path(&Path::new("/path/to/file")).unwrap();
    /// let masked = alignment.get_alignment("G1").unwrap();
    /// ```
    pub fn get_alignment(&self, gene_id: &str) -> ProleResult<String> {
        let seq = self.seq.get(gene_id)
            .ok_or_else(|| ProleError::Exit(format!("Missing sequence for: {}", gene_id)))?;
        let seq_chars: Vec<_> = seq.chars().collect();
        let mut out = String::with_capacity(self.mask_idx.len());
        for &idx in &self.mask_idx {
            out.push(seq_chars[idx]);
        }
        Ok(out)
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
        writeln!(file, "# STOCKHOLM 1.0").unwrap();
        writeln!(file, "G1           .mAKIIN").unwrap();
        writeln!(file, "#=GR G1 PP   .*799**").unwrap();
        writeln!(file, "G2           maAKDIR").unwrap();
        writeln!(file, "#=GR G2 PP   **79***").unwrap();
        writeln!(file, "G3           .mAKEIK").unwrap();
        writeln!(file, "#=GR G3 PP   .*79***").unwrap();
        writeln!(file, "G4           maAKDVK").unwrap();
        writeln!(file, "#=GR G4 PP   **79***").unwrap();
        writeln!(file, "G5           .mSKKIL").unwrap();
        writeln!(file, "#=GR G5 PP   .*699**").unwrap();
        writeln!(file, "#=GC PP_cons ..79***").unwrap();
        writeln!(file, "#=GC RF      ..x.xx.").unwrap();
        writeln!(file, "//").unwrap();

        let result = HmmAlignFile::from_path(&file.into_temp_path());
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.seq.len(), 5);
        assert_eq!(result.pp.len(), 5);
        assert_eq!(result.pp_cons, "..79***");
        assert_eq!(result.mask, vec![false, false, true, false, true, true, false]);
        assert_eq!(result.mask_idx, vec![2, 4, 5]);

        assert_eq!(result.seq.get("G1").unwrap(), ".mAKIIN");
        assert_eq!(result.pp.get("G1").unwrap(), ".*799**");
        assert_eq!(result.get_alignment("G1").unwrap(), "AII");

        assert_eq!(result.seq.get("G2").unwrap(), "maAKDIR");
        assert_eq!(result.pp.get("G2").unwrap(), "**79***");
        assert_eq!(result.get_alignment("G2").unwrap(), "ADI");

        assert_eq!(result.seq.get("G3").unwrap(), ".mAKEIK");
        assert_eq!(result.pp.get("G3").unwrap(), ".*79***");
        assert_eq!(result.get_alignment("G3").unwrap(), "AEI");

        assert_eq!(result.seq.get("G4").unwrap(), "maAKDVK");
        assert_eq!(result.pp.get("G4").unwrap(), "**79***");
        assert_eq!(result.get_alignment("G4").unwrap(), "ADV");

        assert_eq!(result.seq.get("G5").unwrap(), ".mSKKIL");
        assert_eq!(result.pp.get("G5").unwrap(), ".*699**");
        assert_eq!(result.get_alignment("G5").unwrap(), "SKI");
    }
}
