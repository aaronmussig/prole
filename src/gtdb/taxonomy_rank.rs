/// The 7-rank taxonomy used by GTDB.
pub const TAXONOMY_RANKS: [TaxonomyRank; 7] = [
    TaxonomyRank::Domain,
    TaxonomyRank::Phylum,
    TaxonomyRank::Class,
    TaxonomyRank::Order,
    TaxonomyRank::Family,
    TaxonomyRank::Genus,
    TaxonomyRank::Species,
];

/// An enum of the 7-rank taxonomy used by GTDB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaxonomyRank {
    Domain,
    Phylum,
    Class,
    Order,
    Family,
    Genus,
    Species,
}

impl TaxonomyRank {
    /// Return the prefix associated with this [TaxonomyRank].
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// assert_eq!(TaxonomyRank::Domain.prefix(), "d");
    /// ```
    pub fn prefix(&self) -> String {
        match self {
            TaxonomyRank::Domain => "d".to_string(),
            TaxonomyRank::Phylum => "p".to_string(),
            TaxonomyRank::Class => "c".to_string(),
            TaxonomyRank::Order => "o".to_string(),
            TaxonomyRank::Family => "f".to_string(),
            TaxonomyRank::Genus => "g".to_string(),
            TaxonomyRank::Species => "s".to_string(),
        }
    }

    /// Returns the next [TaxonomyRank] lower than this one.
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// assert_eq!(TaxonomyRank::Order.lower(), Some(TaxonomyRank::Family));
    /// ```
    pub fn lower(&self) -> Option<Self> {
        if matches!(self, TaxonomyRank::Species) {
            return None;
        }
        Some(TAXONOMY_RANKS[(*self as usize) + 1])
    }

    /// Returns an [Iterator] to the [TaxonomyRank]s lower than this one (exclusive bounds).
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// let ranks: Vec<_> = TaxonomyRank::Class.lower_ranks().collect();
    /// assert_eq!(ranks, vec![TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);
    /// ```
    pub fn lower_ranks(&self) -> impl Iterator<Item=TaxonomyRank> {
        let start_idx = (*self as usize) + 1;
        TAXONOMY_RANKS[start_idx..].iter().cloned()
    }

    /// Returns an [Iterator] to the [TaxonomyRank]s lower than this one (inclusive bounds).
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// let ranks: Vec<TaxonomyRank> = TaxonomyRank::Class.lower_ranks_inclusive().collect();
    /// assert_eq!(ranks, vec![TaxonomyRank::Class, TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);
    /// ```
    pub fn lower_ranks_inclusive(&self) -> impl Iterator<Item=TaxonomyRank> {
        let start_idx = *self as usize;
        TAXONOMY_RANKS[start_idx..].iter().cloned()
    }

    /// Returns the next [TaxonomyRank] higher than this one.
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// assert_eq!(TaxonomyRank::Order.higher(), Some(TaxonomyRank::Class));
    /// ```
    pub fn higher(&self) -> Option<Self> {
        if matches!(self, TaxonomyRank::Domain) {
            return None;
        }
        let end_idx = (*self as usize).saturating_sub(1);
        Some(TAXONOMY_RANKS[end_idx])
    }

    /// Returns an [Iterator] to the [TaxonomyRank]s higher than this one (exclusive bounds).
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// let ranks: Vec<_> = TaxonomyRank::Class.higher_ranks().collect();
    /// assert_eq!(ranks, vec![TaxonomyRank::Phylum, TaxonomyRank::Domain]);
    /// ```
    pub fn higher_ranks(&self) -> impl Iterator<Item=TaxonomyRank> {
        let end_idx = if matches!(self, TaxonomyRank::Domain) {
            0
        } else {
            *self as usize
        };
        TAXONOMY_RANKS[..end_idx].iter().rev().cloned()
    }

    /// Returns an [Iterator] to the [TaxonomyRank]s higher than this one (exclusive bounds).
    ///
    /// ```
    /// use prole::gtdb::taxonomy_rank::TaxonomyRank;
    ///
    /// let ranks: Vec<TaxonomyRank> = TaxonomyRank::Class.higher_ranks_inclusive().collect();
    /// assert_eq!(ranks, vec![TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);
    /// ```
    pub fn higher_ranks_inclusive(&self) -> impl Iterator<Item=TaxonomyRank> {
        let end_idx = *self as usize;
        TAXONOMY_RANKS[..=end_idx].iter().rev().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        assert_eq!(TaxonomyRank::Domain.prefix(), "d");
        assert_eq!(TaxonomyRank::Phylum.prefix(), "p");
        assert_eq!(TaxonomyRank::Class.prefix(), "c");
        assert_eq!(TaxonomyRank::Order.prefix(), "o");
        assert_eq!(TaxonomyRank::Family.prefix(), "f");
        assert_eq!(TaxonomyRank::Genus.prefix(), "g");
        assert_eq!(TaxonomyRank::Species.prefix(), "s");
    }

    #[test]
    fn test_lower() {
        assert_eq!(TaxonomyRank::Domain.lower(), Some(TaxonomyRank::Phylum));
        assert_eq!(TaxonomyRank::Phylum.lower(), Some(TaxonomyRank::Class));
        assert_eq!(TaxonomyRank::Class.lower(), Some(TaxonomyRank::Order));
        assert_eq!(TaxonomyRank::Order.lower(), Some(TaxonomyRank::Family));
        assert_eq!(TaxonomyRank::Family.lower(), Some(TaxonomyRank::Genus));
        assert_eq!(TaxonomyRank::Genus.lower(), Some(TaxonomyRank::Species));
        assert_eq!(TaxonomyRank::Species.lower(), None);
    }

    #[test]
    fn test_lower_ranks() {
        let domain: Vec<TaxonomyRank> = TaxonomyRank::Domain.lower_ranks().collect();
        assert_eq!(domain, vec![TaxonomyRank::Phylum, TaxonomyRank::Class, TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let phylum: Vec<TaxonomyRank> = TaxonomyRank::Phylum.lower_ranks().collect();
        assert_eq!(phylum, vec![TaxonomyRank::Class, TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let class: Vec<TaxonomyRank> = TaxonomyRank::Class.lower_ranks().collect();
        assert_eq!(class, vec![TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let order: Vec<TaxonomyRank> = TaxonomyRank::Order.lower_ranks().collect();
        assert_eq!(order, vec![TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let family: Vec<TaxonomyRank> = TaxonomyRank::Family.lower_ranks().collect();
        assert_eq!(family, vec![TaxonomyRank::Genus, TaxonomyRank::Species]);

        let genus: Vec<TaxonomyRank> = TaxonomyRank::Genus.lower_ranks().collect();
        assert_eq!(genus, vec![TaxonomyRank::Species]);

        let species: Vec<TaxonomyRank> = TaxonomyRank::Species.lower_ranks().collect();
        assert_eq!(species, vec![]);
    }

    #[test]
    fn test_lower_ranks_inclusive() {
        let domain: Vec<TaxonomyRank> = TaxonomyRank::Domain.lower_ranks_inclusive().collect();
        assert_eq!(domain, vec![TaxonomyRank::Domain, TaxonomyRank::Phylum, TaxonomyRank::Class, TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let phylum: Vec<TaxonomyRank> = TaxonomyRank::Phylum.lower_ranks_inclusive().collect();
        assert_eq!(phylum, vec![TaxonomyRank::Phylum, TaxonomyRank::Class, TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let class: Vec<TaxonomyRank> = TaxonomyRank::Class.lower_ranks_inclusive().collect();
        assert_eq!(class, vec![TaxonomyRank::Class, TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let order: Vec<TaxonomyRank> = TaxonomyRank::Order.lower_ranks_inclusive().collect();
        assert_eq!(order, vec![TaxonomyRank::Order, TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let family: Vec<TaxonomyRank> = TaxonomyRank::Family.lower_ranks_inclusive().collect();
        assert_eq!(family, vec![TaxonomyRank::Family, TaxonomyRank::Genus, TaxonomyRank::Species]);

        let genus: Vec<TaxonomyRank> = TaxonomyRank::Genus.lower_ranks_inclusive().collect();
        assert_eq!(genus, vec![TaxonomyRank::Genus, TaxonomyRank::Species]);

        let species: Vec<TaxonomyRank> = TaxonomyRank::Species.lower_ranks_inclusive().collect();
        assert_eq!(species, vec![TaxonomyRank::Species]);
    }

    #[test]
    fn test_higher() {
        assert_eq!(TaxonomyRank::Domain.higher(), None);
        assert_eq!(TaxonomyRank::Phylum.higher(), Some(TaxonomyRank::Domain));
        assert_eq!(TaxonomyRank::Class.higher(), Some(TaxonomyRank::Phylum));
        assert_eq!(TaxonomyRank::Order.higher(), Some(TaxonomyRank::Class));
        assert_eq!(TaxonomyRank::Family.higher(), Some(TaxonomyRank::Order));
        assert_eq!(TaxonomyRank::Genus.higher(), Some(TaxonomyRank::Family));
        assert_eq!(TaxonomyRank::Species.higher(), Some(TaxonomyRank::Genus));
    }

    #[test]
    fn test_higher_ranks() {
        let domain: Vec<TaxonomyRank> = TaxonomyRank::Domain.higher_ranks().collect();
        assert_eq!(domain, vec![]);

        let phylum: Vec<TaxonomyRank> = TaxonomyRank::Phylum.higher_ranks().collect();
        assert_eq!(phylum, vec![TaxonomyRank::Domain]);

        let class: Vec<TaxonomyRank> = TaxonomyRank::Class.higher_ranks().collect();
        assert_eq!(class, vec![TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let order: Vec<TaxonomyRank> = TaxonomyRank::Order.higher_ranks().collect();
        assert_eq!(order, vec![TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let family: Vec<TaxonomyRank> = TaxonomyRank::Family.higher_ranks().collect();
        assert_eq!(family, vec![TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let genus: Vec<TaxonomyRank> = TaxonomyRank::Genus.higher_ranks().collect();
        assert_eq!(genus, vec![TaxonomyRank::Family, TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let species: Vec<TaxonomyRank> = TaxonomyRank::Species.higher_ranks().collect();
        assert_eq!(species, vec![TaxonomyRank::Genus, TaxonomyRank::Family, TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);
    }

    #[test]
    fn test_higher_ranks_inclusive() {
        let domain: Vec<TaxonomyRank> = TaxonomyRank::Domain.higher_ranks_inclusive().collect();
        assert_eq!(domain, vec![TaxonomyRank::Domain]);

        let phylum: Vec<TaxonomyRank> = TaxonomyRank::Phylum.higher_ranks_inclusive().collect();
        assert_eq!(phylum, vec![TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let class: Vec<TaxonomyRank> = TaxonomyRank::Class.higher_ranks_inclusive().collect();
        assert_eq!(class, vec![TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let order: Vec<TaxonomyRank> = TaxonomyRank::Order.higher_ranks_inclusive().collect();
        assert_eq!(order, vec![TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let family: Vec<TaxonomyRank> = TaxonomyRank::Family.higher_ranks_inclusive().collect();
        assert_eq!(family, vec![TaxonomyRank::Family, TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let genus: Vec<TaxonomyRank> = TaxonomyRank::Genus.higher_ranks_inclusive().collect();
        assert_eq!(genus, vec![TaxonomyRank::Genus, TaxonomyRank::Family, TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);

        let species: Vec<TaxonomyRank> = TaxonomyRank::Species.higher_ranks_inclusive().collect();
        assert_eq!(species, vec![TaxonomyRank::Species, TaxonomyRank::Genus, TaxonomyRank::Family, TaxonomyRank::Order, TaxonomyRank::Class, TaxonomyRank::Phylum, TaxonomyRank::Domain]);
    }
}