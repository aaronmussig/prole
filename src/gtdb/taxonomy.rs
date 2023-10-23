use crate::error::{ProleError, ProleResult};
use crate::gtdb::taxon::Taxon;
use crate::gtdb::taxonomy_rank::TaxonomyRank;

/// A 7-rank [Taxonomy] containing taxa ([Taxon]) for each [TaxonomyRank].
pub struct Taxonomy {
    pub domain: Taxon,
    pub phylum: Taxon,
    pub class: Taxon,
    pub order: Taxon,
    pub family: Taxon,
    pub genus: Taxon,
    pub species: Taxon,
}

impl Taxonomy {
    /// Creates a [Taxonomy] struct from a taxonomy string.
    ///
    /// ```
    /// use prole::gtdb::taxon::Taxon;
    /// use prole::gtdb::taxonomy::Taxonomy;
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// let taxonomy = Taxonomy::from_string("d__d1;p__p1;c__c1;o__o1;f__f1;g__g1;s__s1 s2").unwrap();
    /// assert_eq!(taxonomy.get(&TaxonomyRank::Domain), &Taxon("d__d1".to_string()));
    /// ```
    pub fn from_string(string: &str) -> ProleResult<Self> {
        let string_split = string.split(';').collect::<Vec<&str>>();
        if string_split.len() != 7 {
            return Err(ProleError::Exit(format!("Taxonomy string has {} fields, expected 7", string_split.len())));
        }
        Ok(Self {
            domain: Taxon(string_split[0].trim().to_string()),
            phylum: Taxon(string_split[1].trim().to_string()),
            class: Taxon(string_split[2].trim().to_string()),
            order: Taxon(string_split[3].trim().to_string()),
            family: Taxon(string_split[4].trim().to_string()),
            genus: Taxon(string_split[5].trim().to_string()),
            species: Taxon(string_split[6].trim().to_string()),
        })
    }

    /// Returns the [Taxon] for a given [TaxonomyRank].
    ///
    /// ```
    /// use prole::gtdb::taxon::Taxon;
    /// use prole::gtdb::taxonomy::Taxonomy;
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// let taxonomy = Taxonomy::from_string("d__d1;p__p1;c__c1;o__o1;f__f1;g__g1;s__s1 s2").unwrap();
    /// assert_eq!(taxonomy.get(&TaxonomyRank::Domain), &Taxon("d__d1".to_string()));
    /// ```
    pub fn get(&self, rank: &TaxonomyRank) -> &Taxon {
        match rank {
            TaxonomyRank::Domain => &self.domain,
            TaxonomyRank::Phylum => &self.phylum,
            TaxonomyRank::Class => &self.class,
            TaxonomyRank::Order => &self.order,
            TaxonomyRank::Family => &self.family,
            TaxonomyRank::Genus => &self.genus,
            TaxonomyRank::Species => &self.species,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let result = Taxonomy::from_string("d__d1;p__p1;c__c1;o__o1;f__f1;g__g1;s__s1 s2");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get(&TaxonomyRank::Domain).0, "d__d1");
        assert_eq!(result.get(&TaxonomyRank::Phylum).0, "p__p1");
        assert_eq!(result.get(&TaxonomyRank::Class).0, "c__c1");
        assert_eq!(result.get(&TaxonomyRank::Order).0, "o__o1");
        assert_eq!(result.get(&TaxonomyRank::Family).0, "f__f1");
        assert_eq!(result.get(&TaxonomyRank::Genus).0, "g__g1");
        assert_eq!(result.get(&TaxonomyRank::Species).0, "s__s1 s2");
    }

    #[test]
    fn test_from_string_spaces() {
        let result = Taxonomy::from_string("d__d1; p__p1; c__c1; o__o1; f__f1; g__g1; s__s1 s2");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.get(&TaxonomyRank::Domain).0, "d__d1");
        assert_eq!(result.get(&TaxonomyRank::Phylum).0, "p__p1");
        assert_eq!(result.get(&TaxonomyRank::Class).0, "c__c1");
        assert_eq!(result.get(&TaxonomyRank::Order).0, "o__o1");
        assert_eq!(result.get(&TaxonomyRank::Family).0, "f__f1");
        assert_eq!(result.get(&TaxonomyRank::Genus).0, "g__g1");
        assert_eq!(result.get(&TaxonomyRank::Species).0, "s__s1 s2");
    }
}