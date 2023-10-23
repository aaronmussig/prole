use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_CANONICAL: Regex = Regex::new(r"^(?:(?:(?:GB_)?(GCA_))|(?:(?:RS_)?(GCF_)))(\d{9})\.(\d)$").unwrap();
}


/// A genome accession, there is no restriction to the format of the accession.
///
/// ```
/// use prole::genome::genome_id::GenomeId;
///
/// let _ = GenomeId("GB_GCA_123456789.1".to_string());
/// let _ = GenomeId("RS_GCF_123456789.1".to_string());
/// let _ = GenomeId("G123456789".to_string());
/// ```
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GenomeId(pub String);


impl GenomeId {
    /// Converts the accession to canonical form, if it is a valid GenBank/RefSeq identifier,
    /// otherwise None is returned.
    ///
    /// ```
    /// use prole::genome::genome_id::GenomeId;
    ///
    /// let gid = GenomeId("GCF_123456789.1".to_string());
    /// assert_eq!(gid.to_canonical(), Some(GenomeId("G123456789".to_string())));
    /// ```
    #[must_use]
    pub fn to_canonical(&self) -> Option<Self> {
        if let Some(hit) = RE_CANONICAL.captures(&self.0) {
            return Some(GenomeId(format!("G{}", &hit[3])));
        }
        None
    }

    /// Converts the accession to short form, if it is a valid GenBank/RefSeq identifier,
    /// otherwise None is returned.
    /// ```
    /// use prole::genome::genome_id::GenomeId;
    ///
    /// let gid = GenomeId("RS_GCF_123456789.1".to_string());
    /// assert_eq!(gid.to_short(), Some(GenomeId("GCF_123456789.1".to_string())));
    /// ```
    #[must_use]
    pub fn to_short(&self) -> Option<Self> {
        if let Some(hit) = RE_CANONICAL.captures(&self.0) {
            let prefix: &str = {
                if let Some(grp_1) = hit.get(1) {
                    grp_1.as_str()
                } else {
                    hit.get(2).unwrap().as_str()
                }
            };
            return Some(GenomeId(format!("{}{}.{}", prefix, &hit[3], &hit[4])));
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::genome::genome_id::GenomeId;

    #[test]
    fn test_to_canonical() {
        assert_eq!(GenomeId("GB_GCA_123456789.1".to_string()).to_canonical(), Some(GenomeId("G123456789".to_string())));
        assert_eq!(GenomeId("RS_GCF_123456789.1".to_string()).to_canonical(), Some(GenomeId("G123456789".to_string())));
        assert_eq!(GenomeId("GCA_123456789.1".to_string()).to_canonical(), Some(GenomeId("G123456789".to_string())));
        assert_eq!(GenomeId("GCF_123456789.1".to_string()).to_canonical(), Some(GenomeId("G123456789".to_string())));
        assert_eq!(GenomeId("something".to_string()).to_canonical(), None);
        assert_eq!(GenomeId("".to_string()).to_canonical(), None);
    }

    #[test]
    fn test_to_short() {
        assert_eq!(GenomeId("GB_GCA_123456789.1".to_string()).to_short(), Some(GenomeId("GCA_123456789.1".to_string())));
        assert_eq!(GenomeId("RS_GCF_123456789.1".to_string()).to_short(), Some(GenomeId("GCF_123456789.1".to_string())));
        assert_eq!(GenomeId("GCA_123456789.1".to_string()).to_short(), Some(GenomeId("GCA_123456789.1".to_string())));
        assert_eq!(GenomeId("GCF_123456789.1".to_string()).to_short(), Some(GenomeId("GCF_123456789.1".to_string())));
        assert_eq!(GenomeId("something".to_string()).to_short(), None);
        assert_eq!(GenomeId("".to_string()).to_short(), None);
    }
}
