use crate::sequence::tln_table::TranslationTable;

/// Translate a nucleotide sequence into the corresponding amino acid sequence.
pub fn translate_sequence(seq: &str, table: TranslationTable) -> String {
    let mut prot = String::with_capacity(seq.len() / 3);
    for i in (0..seq.len()).step_by(3) {
        let codon = &seq[i..i + 3];
        let aa = table.translate(codon);
        prot.push(*aa);
    }
    prot
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_11() {
        let dna = "AAAAACAAGAATACAACCACGACTAGAAGCAGGAGTATAATCATGATTCAACACCAGCATCCACCCCCGCCTCGACGCCGGCGTCTACTCCTGCTTGAAGACGAGGATGCAGCCGCGGCTGGAGGCGGGGGTGTAGTCGTGGTTTAATACTAGTATTCATCCTCGTCTTGATGCTGGTGTTTATTCTTGTTT";
        let prot_expected = "KNKNTTTTRSRSIIMIQHQHPPPPRRRRLLLLEDEDAAAAGGGGVVVV-Y-YSSSS-CWCLFLF";
        let prot = translate_sequence(dna, TranslationTable::T11);
    }
}