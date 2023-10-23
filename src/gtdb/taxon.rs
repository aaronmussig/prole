
/// A [Taxon] within the GTDB.
///
/// ```
/// use prole::gtdb::taxon::Taxon;
///
/// let _ = Taxon("d__Bacteria".to_string());
/// ```
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Taxon(pub String);
