use std::collections::HashMap;

use lazy_static::lazy_static;

/// Specify the translation table for encoding.
pub enum TranslationTable {
    T11
}


impl TranslationTable {
    pub fn translate(&self, codon: &str) -> &char {
        match self {
            TranslationTable::T11 => TLN_TABLE_11.get(codon).unwrap(),
        }
    }
}

lazy_static! {
    /// Specify the encoding for each translation table.
    static ref TLN_TABLE_11: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert("AAA", 'K');
        m.insert("AAC", 'N');
        m.insert("AAG", 'K');
        m.insert("AAT", 'N');
        m.insert("ACA", 'T');
        m.insert("ACC", 'T');
        m.insert("ACG", 'T');
        m.insert("ACT", 'T');
        m.insert("AGA", 'R');
        m.insert("AGC", 'S');
        m.insert("AGG", 'R');
        m.insert("AGT", 'S');
        m.insert("ATA", 'I');
        m.insert("ATC", 'I');
        m.insert("ATG", 'M');
        m.insert("ATT", 'I');
        m.insert("CAA", 'Q');
        m.insert("CAC", 'H');
        m.insert("CAG", 'Q');
        m.insert("CAT", 'H');
        m.insert("CCA", 'P');
        m.insert("CCC", 'P');
        m.insert("CCG", 'P');
        m.insert("CCT", 'P');
        m.insert("CGA", 'R');
        m.insert("CGC", 'R');
        m.insert("CGG", 'R');
        m.insert("CGT", 'R');
        m.insert("CTA", 'L');
        m.insert("CTC", 'L');
        m.insert("CTG", 'L');
        m.insert("CTT", 'L');
        m.insert("GAA", 'E');
        m.insert("GAC", 'D');
        m.insert("GAG", 'E');
        m.insert("GAT", 'D');
        m.insert("GCA", 'A');
        m.insert("GCC", 'A');
        m.insert("GCG", 'A');
        m.insert("GCT", 'A');
        m.insert("GGA", 'G');
        m.insert("GGC", 'G');
        m.insert("GGG", 'G');
        m.insert("GGT", 'G');
        m.insert("GTA", 'V');
        m.insert("GTC", 'V');
        m.insert("GTG", 'V');
        m.insert("GTT", 'V');
        m.insert("TAA", '-');
        m.insert("TAC", 'Y');
        m.insert("TAG", '-');
        m.insert("TAT", 'Y');
        m.insert("TCA", 'S');
        m.insert("TCC", 'S');
        m.insert("TCG", 'S');
        m.insert("TCT", 'S');
        m.insert("TGA", '-');
        m.insert("TGC", 'C');
        m.insert("TGG", 'W');
        m.insert("TGT", 'C');
        m.insert("TTA", 'L');
        m.insert("TTC", 'F');
        m.insert("TTG", 'L');
        m.insert("TTT", 'F');
        m
    };
}



