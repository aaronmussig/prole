use std::io::Read;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ProleError, ProleResult};
use crate::gtdb::taxonomy_rank::TaxonomyRank;
use crate::phylorank::red::RED;

lazy_static! {
    static ref RE_DICT: Regex = Regex::new(r".phylum.:\s?([\d.]+).+.class.:\s?([\d.]+).+.order.:\s?([\d.]+).+.family.:\s?([\d.]+).+.genus.:\s?([\d.]+).+").unwrap();
}

/// The [RED] dictionary output by PhyloRank.
pub struct PhyloRankRedDict {
    pub phylum: RED,
    pub class: RED,
    pub order: RED,
    pub family: RED,
    pub genus: RED,
}

impl PhyloRankRedDict {
    /// Returns the [TaxonomyRank] [RED] value.
    pub fn get(&self, rank: &TaxonomyRank) -> Option<RED> {
        match rank {
            TaxonomyRank::Phylum => Some(self.phylum),
            TaxonomyRank::Class => Some(self.class),
            TaxonomyRank::Order => Some(self.order),
            TaxonomyRank::Family => Some(self.family),
            TaxonomyRank::Genus => Some(self.genus),
            _ => None
        }
    }

    /// Load the [PhyloRankRedDict] from the [Path].
    pub fn load(path: &Path) -> ProleResult<Self> {
        let file = std::fs::File::open(path).map_err(ProleError::IoError)?;
        let mut reader = std::io::BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).map_err(ProleError::IoError)?;

        let hits = RE_DICT.captures(&contents).ok_or(ProleError::Exit("No hits found".to_string()))?;

        Ok(Self {
            phylum: RED(hits[1].parse().map_err(ProleError::ParseFloatError)?),
            class: RED(hits[2].parse().map_err(ProleError::ParseFloatError)?),
            order: RED(hits[3].parse().map_err(ProleError::ParseFloatError)?),
            family: RED(hits[4].parse().map_err(ProleError::ParseFloatError)?),
            genus: RED(hits[5].parse().map_err(ProleError::ParseFloatError)?),
        })
    }
}


#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::gtdb::taxonomy_rank::TaxonomyRank;
    use crate::phylorank::red::RED;

    use super::PhyloRankRedDict;

    #[test]
    fn test_get_rank() {
        let red_dict = PhyloRankRedDict {
            phylum: RED(0.21),
            class: RED(0.35),
            order: RED(0.51),
            family: RED(0.70),
            genus: RED(0.89),
        };
        assert_eq!(red_dict.get(&TaxonomyRank::Phylum).unwrap(), RED(0.21));
        assert_eq!(red_dict.get(&TaxonomyRank::Class).unwrap(), RED(0.35));
        assert_eq!(red_dict.get(&TaxonomyRank::Order).unwrap(), RED(0.51));
        assert_eq!(red_dict.get(&TaxonomyRank::Family).unwrap(), RED(0.70));
        assert_eq!(red_dict.get(&TaxonomyRank::Genus).unwrap(), RED(0.89));
    }

    #[test]
    fn test_load() {
        // Create a temporary file with sample RED data
        let mut tmp_file = NamedTempFile::new().unwrap();
        writeln!(tmp_file, r#"{{"phylum":0.21,"class":0.35,"order":0.51,"family":0.70,"genus":0.89}}"#).unwrap();

        let red_dict = PhyloRankRedDict::load(tmp_file.path()).unwrap();
        assert_eq!(red_dict.phylum.0, 0.21);
        assert_eq!(red_dict.class.0, 0.35);
        assert_eq!(red_dict.order.0, 0.51);
        assert_eq!(red_dict.family.0, 0.70);
        assert_eq!(red_dict.genus.0, 0.89);
    }
}
