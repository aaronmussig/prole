use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::error::{ProleError, ProleResult};
use crate::genome::genome_id::GenomeId;
use crate::gtdb::taxonomy::Taxonomy;

// fn parse_float<T: std::str::FromStr<Err=ParseFloatError>>(value: &str) -> ProleResult<T> {
//     value.parse().map_err(ProleError::ParseFloatError)
// }
//
// fn parse_opt_float<T: std::str::FromStr<Err=ParseFloatError>>(value: &str) -> ProleResult<Option<T>> {
//     if value == "none" {
//         Ok(None)
//     } else {
//         value.parse().map(Some).map_err(ProleError::ParseFloatError)
//     }
// }
//
// fn parse_int<T: std::str::FromStr<Err=ParseIntError>>(value: &str) -> ProleResult<T> {
//     value.parse().map_err(ProleError::ParseIntError)
// }
//
// fn parse_opt_int<T: std::str::FromStr<Err=ParseIntError>>(value: &str) -> ProleResult<Option<T>> {
//     if value == "none" {
//         Ok(None)
//     } else {
//         value.parse().map(Some).map_err(ProleError::ParseIntError)
//     }
// }
//
// fn parse_opt_string(value: &str) -> Option<String> {
//     if value == "none" {
//         None
//     } else {
//         Some(value.to_string())
//     }
// }

/// A row within the [GtdbMetadataR214] file.
pub struct GtdbMetadataR214Row {
    pub accession: GenomeId,
    // pub ambiguous_bases: usize,
    // pub checkm_completeness: f32,
    // pub checkm_contamination: f32,
    // pub checkm_marker_count: usize,
    // pub checkm_marker_lineage: String,
    // pub checkm_marker_set_count: usize,
    // pub checkm_strain_heterogeneity: f32,
    // pub coding_bases: usize,
    // pub coding_density: f64,
    // pub contig_count: usize,
    // pub gc_count: usize,
    // pub gc_percentage: f64,
    // pub genome_size: usize,
    // pub gtdb_genome_representative: GenomeId,
    pub gtdb_representative: bool,
    pub gtdb_taxonomy: Taxonomy,
    // pub gtdb_type_designation_ncbi_taxa: String,
    // pub gtdb_type_designation_ncbi_taxa_sources: Option<String>,
    // pub gtdb_type_species_of_genus: bool,
    // pub l50_contigs: usize,
    // pub l50_scaffolds: usize,
    // pub longest_contig: usize,
    // pub longest_scaffold: usize,
    // pub lsu_23s_contig_len: Option<usize>,
    // pub lsu_23s_count: usize,
    // pub lsu_23s_length: Option<usize>,
    // pub lsu_23s_query_id: Option<String>,
    // pub lsu_5s_contig_len: Option<usize>,
    // pub lsu_5s_count: usize,
    // pub lsu_5s_length: Option<usize>,
    // pub lsu_5s_query_id: Option<String>,
    // pub lsu_silva_23s_blast_align_len: Option<usize>,
    // pub lsu_silva_23s_blast_bitscore: Option<usize>,
    // pub lsu_silva_23s_blast_evalue: Option<f64>,
    // pub lsu_silva_23s_blast_perc_identity: Option<f64>,
    // pub lsu_silva_23s_blast_subject_id: Option<String>,
    // pub lsu_silva_23s_taxonomy: Option<String>,
    // pub mean_contig_length: usize,
    // pub mean_scaffold_length: usize,
    // pub mimag_high_quality: bool,
    // pub mimag_low_quality: bool,
    // pub mimag_medium_quality: bool,
    // pub n50_contigs: usize,
    // pub n50_scaffolds: usize,
    // pub ncbi_assembly_level: String,
    // pub ncbi_assembly_name: String,
    // pub ncbi_assembly_type: Option<String>,
    // pub ncbi_bioproject: String,
    // pub ncbi_biosample: String,
    // pub ncbi_contig_count: Option<usize>,
    // pub ncbi_contig_n50: Option<usize>,
    // pub ncbi_country: Option<String>,
    // pub ncbi_date: String,
    // pub ncbi_genbank_assembly_accession: GenomeId,
    // pub ncbi_genome_category: Option<String>,
    // pub ncbi_genome_representation: String,
    // pub ncbi_isolate: Option<String>,
    // pub ncbi_isolation_source: Option<String>,
    // pub ncbi_lat_lon: Option<String>,
    // pub ncbi_molecule_count: usize,
    // pub ncbi_ncrna_count: usize,
    // pub ncbi_organism_name: String,
    // pub ncbi_protein_count: usize,
    // pub ncbi_refseq_category: Option<String>,
    // pub ncbi_rrna_count: usize,
    // pub ncbi_scaffold_count: Option<usize>,
    // pub ncbi_scaffold_l50: Option<usize>,
    // pub ncbi_scaffold_n50: Option<usize>,
    // pub ncbi_scaffold_n75: Option<usize>,
    // pub ncbi_scaffold_n90: Option<usize>,
    // pub ncbi_seq_rel_date: String,
    // pub ncbi_spanned_gaps: usize,
    // pub ncbi_species_taxid: usize,
    // pub ncbi_ssu_count: usize,
    // pub ncbi_strain_identifiers: String,
    // pub ncbi_submitter: String,
    // pub ncbi_taxid: usize,
    // pub ncbi_taxonomy: String,
    // pub ncbi_taxonomy_unfiltered: String,
    // pub ncbi_total_gap_length: usize,
    // pub ncbi_total_length: usize,
    // pub ncbi_translation_table: usize,
    // pub ncbi_trna_count: usize,
    // pub ncbi_type_material_designation: Option<String>,
    // pub ncbi_ungapped_length: usize,
    // pub ncbi_unspanned_gaps: usize,
    // pub ncbi_wgs_master: Option<String>,
    // pub protein_count: usize,
    // pub scaffold_count: usize,
    // pub ssu_contig_len: usize,
    // pub ssu_count: usize,
    // pub ssu_gg_blast_align_len: Option<usize>,
    // pub ssu_gg_blast_bitscore: Option<usize>,
    // pub ssu_gg_blast_evalue: Option<f64>,
    // pub ssu_gg_blast_perc_identity: Option<f64>,
    // pub ssu_gg_blast_subject_id: Option<usize>,
    // pub ssu_gg_taxonomy: Option<String>,
    // pub ssu_length: Option<usize>,
    // pub ssu_query_id: Option<String>,
    // pub ssu_silva_blast_align_len: Option<usize>,
    // pub ssu_silva_blast_bitscore: Option<usize>,
    // pub ssu_silva_blast_evalue: Option<usize>,
    // pub ssu_silva_blast_perc_identity: Option<usize>,
    // pub ssu_silva_blast_subject_id: Option<String>,
    // pub ssu_silva_taxonomy: Option<String>,
    // pub total_gap_length: usize,
    // pub trna_aa_count: usize,
    // pub trna_count: usize,
    // pub trna_selenocysteine_count: usize,
}

impl GtdbMetadataR214Row {
    pub fn from_string(string: &str) -> ProleResult<Self> {
        println!("string: {}\n", string);
        let split = string.split('\t').collect::<Vec<&str>>();
        if split.len() != 110 {
            return Err(ProleError::Exit(format!("Expected 110 columns, got {}", split.len())));
        }
        let out = Self {
            accession: GenomeId(split[0].to_string()),
            // ambiguous_bases: parse_int(split[1])?,
            // checkm_completeness: parse_float(split[2])?,
            // checkm_contamination: parse_float(split[3])?,
            // checkm_marker_count: parse_int(split[4])?,
            // checkm_marker_lineage: split[5].to_string(),
            // checkm_marker_set_count: parse_int(split[6])?,
            // checkm_strain_heterogeneity: parse_float(split[7])?,
            // coding_bases: parse_int(split[8])?,
            // coding_density: parse_float(split[9])?,
            // contig_count: parse_int(split[10])?,
            // gc_count: parse_int(split[11])?,
            // gc_percentage: parse_float(split[12])?,
            // genome_size: parse_int(split[13])?,
            // gtdb_genome_representative: GenomeId(split[14].to_string()),
            gtdb_representative: split[15] == "t",
            gtdb_taxonomy: Taxonomy::from_string(split[16])?,
            // gtdb_type_designation_ncbi_taxa: split[17].to_string(),
            // gtdb_type_designation_ncbi_taxa_sources: parse_opt_string(split[18]),
            // gtdb_type_species_of_genus: split[19] == "t",
            // l50_contigs: parse_int(split[20])?,
            // l50_scaffolds: parse_int(split[21])?,
            // longest_contig: parse_int(split[22])?,
            // longest_scaffold: parse_int(split[23])?,
            // lsu_23s_contig_len: parse_opt_int(split[24])?,
            // lsu_23s_count: parse_int(split[25])?,
            // lsu_23s_length: parse_opt_int(split[26])?,
            // lsu_23s_query_id: parse_opt_string(split[27]),
            // lsu_5s_contig_len: parse_opt_int(split[28])?,
            // lsu_5s_count: parse_int(split[29])?,
            // lsu_5s_length: parse_opt_int(split[30])?,
            // lsu_5s_query_id: parse_opt_string(split[31]),
            // lsu_silva_23s_blast_align_len: parse_opt_int(split[32])?,
            // lsu_silva_23s_blast_bitscore: parse_opt_int(split[33])?,
            // lsu_silva_23s_blast_evalue: parse_opt_float(split[34])?,
            // lsu_silva_23s_blast_perc_identity: parse_opt_float(split[35])?,
            // lsu_silva_23s_blast_subject_id: parse_opt_string(split[36]),
            // lsu_silva_23s_taxonomy: parse_opt_string(split[37]),
            // mean_contig_length: parse_int(split[38])?,
            // mean_scaffold_length: parse_int(split[39])?,
            // mimag_high_quality: split[40] == "t",
            // mimag_low_quality: split[41] == "t",
            // mimag_medium_quality: split[42] == "t",
            // n50_contigs: parse_int(split[43])?,
            // n50_scaffolds: parse_int(split[44])?,
            // ncbi_assembly_level: split[45].to_string(),
            // ncbi_assembly_name: split[46].to_string(),
            // ncbi_assembly_type: parse_opt_string(split[47]),
            // ncbi_bioproject: split[48].to_string(),
            // ncbi_biosample: split[49].to_string(),
            // ncbi_contig_count: parse_opt_int(split[50])?,
            // ncbi_contig_n50: parse_opt_int(split[51])?,
            // ncbi_country: parse_opt_string(split[52]),
            // ncbi_date: split[53].to_string(),
            // ncbi_genbank_assembly_accession: GenomeId(split[54].to_string()),
            // ncbi_genome_category: parse_opt_string(split[55]),
            // ncbi_genome_representation: split[56].to_string(),
            // ncbi_isolate: parse_opt_string(split[57]),
            // ncbi_isolation_source: parse_opt_string(split[58]),
            // ncbi_lat_lon: parse_opt_string(split[59]),
            // ncbi_molecule_count: parse_int(split[60])?,
            // ncbi_ncrna_count: parse_int(split[61])?,
            // ncbi_organism_name: split[62].to_string(),
            // ncbi_protein_count: parse_int(split[63])?,
            // ncbi_refseq_category: parse_opt_string(split[64]),
            // ncbi_rrna_count: parse_int(split[65])?,
            // ncbi_scaffold_count: parse_opt_int(split[66])?,
            // ncbi_scaffold_l50: parse_opt_int(split[67])?,
            // ncbi_scaffold_n50: parse_opt_int(split[68])?,
            // ncbi_scaffold_n75: parse_opt_int(split[69])?,
            // ncbi_scaffold_n90: parse_opt_int(split[70])?,
            // ncbi_seq_rel_date: split[71].to_string(),
            // ncbi_spanned_gaps: parse_int(split[72])?,
            // ncbi_species_taxid: parse_int(split[73])?,
            // ncbi_ssu_count: parse_int(split[74])?,
            // ncbi_strain_identifiers: split[75].to_string(),
            // ncbi_submitter: split[76].to_string(),
            // ncbi_taxid: parse_int(split[77])?,
            // ncbi_taxonomy: split[78].to_string(),
            // ncbi_taxonomy_unfiltered: split[79].to_string(),
            // ncbi_total_gap_length: parse_int(split[80])?,
            // ncbi_total_length: parse_int(split[81])?,
            // ncbi_translation_table: parse_int(split[82])?,
            // ncbi_trna_count: parse_int(split[83])?,
            // ncbi_type_material_designation: parse_opt_string(split[84]),
            // ncbi_ungapped_length: parse_int(split[85])?,
            // ncbi_unspanned_gaps: parse_int(split[86])?,
            // ncbi_wgs_master: parse_opt_string(split[87]),
            // protein_count: parse_int(split[88])?,
            // scaffold_count: parse_int(split[89])?,
            // ssu_contig_len: parse_int(split[90])?,
            // ssu_count: parse_int(split[91])?,
            // ssu_gg_blast_align_len: parse_opt_int(split[92])?,
            // ssu_gg_blast_bitscore: parse_opt_int(split[93])?,
            // ssu_gg_blast_evalue: parse_opt_float(split[94])?,
            // ssu_gg_blast_perc_identity: parse_opt_float(split[95])?,
            // ssu_gg_blast_subject_id: parse_opt_int(split[96])?,
            // ssu_gg_taxonomy: parse_opt_string(split[97]),
            // ssu_length: parse_opt_int(split[98])?,
            // ssu_query_id: parse_opt_string(split[99]),
            // ssu_silva_blast_align_len: parse_opt_int(split[100])?,
            // ssu_silva_blast_bitscore: parse_opt_int(split[101])?,
            // ssu_silva_blast_evalue: parse_opt_int(split[102])?,
            // ssu_silva_blast_perc_identity: parse_opt_int(split[103])?,
            // ssu_silva_blast_subject_id: parse_opt_string(split[104]),
            // ssu_silva_taxonomy: parse_opt_string(split[105]),
            // total_gap_length: parse_int(split[106])?,
            // trna_aa_count: parse_int(split[107])?,
            // trna_count: parse_int(split[108])?,
            // trna_selenocysteine_count: parse_int(split[109])?,
        };
        Ok(out)
    }
}


/// The GTDB R214 metadata file.
pub struct GtdbMetadataR214 {
    pub rows: HashMap<GenomeId, GtdbMetadataR214Row>,
}

impl GtdbMetadataR214 {
    pub fn from_bufreader<T: std::io::Read>(buf: BufReader<T>) -> ProleResult<Self> {
        let mut out: HashMap<GenomeId, GtdbMetadataR214Row> = HashMap::new();
        for line in buf.lines() {
            let line = line.map_err(ProleError::IoError)?;
            if line.starts_with("accession\tambiguous_bases") || line.is_empty() {
                continue;
            }
            let row = GtdbMetadataR214Row::from_string(&line)?;
            out.insert(row.accession.clone(), row);
        }
        Ok(Self {
            rows: out
        })
    }

    pub fn from_path(path: &Path) -> ProleResult<Self> {
        let file = File::open(path).map_err(ProleError::IoError)?;
        let reader = BufReader::new(file);
        Self::from_bufreader(reader)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let result = GtdbMetadataR214Row::from_string("RS_GCF_000246985.2\t44\t99.5\t0.5\t299\tp__Euryarchaeota (UID4)\t202\t0\t2014456\t90.93903317665627\t1\t954455\t43.08802922449628\t2215172\tRS_GCF_024054535.1\tf\td__Archaea;p__Methanobacteriota_B;c__Thermococci;o__Thermococcales;f__Thermococcaceae;g__Thermococcus_A;s__Thermococcus_A alcaliphilus\ttype strain of species\tLPSN\tf\t1\t1\t2215172\t2215172\t2215172\t1\t3020\tNC_022084.1\t2215172\t2\t103\tNC_022084.1\t3020\t5561\t0\t99.901\tAKID01000054.18410.21433\tArchaea;Euryarchaeota;Thermococci;Thermococcales;Thermococcaceae;Thermococcus;Thermococcus sp. PK\t2215172\t2215172\tt\tf\tf\t2215172\t2215172\tComplete Genome\tASM24698v3\tna\tPRJNA224116\tSAMN02603679\tnone\tnone\tnone\t2013-08-13\tGCA_000246985.3\tnone\tfull\tnone\tnone\tnone\t1\t0\tThermococcus litoralis DSM 5473\t2402\trepresentative genome\t4\t1\t1\t2215172\t2215172\t2215172\t2013/08/13\t0\t2265\t1\tDSM 5473\tNew England Biolabs, Inc.\t523849\td__Archaea;p__Euryarchaeota;c__Thermococci;o__Thermococcales;f__Thermococcaceae;g__Thermococcus;s__Thermococcus litoralis\td__Archaea;p__Euryarchaeota;c__Thermococci;o__Thermococcales;f__Thermococcaceae;g__Thermococcus;s__Thermococcus litoralis;x__Thermococcus litoralis DSM 5473\t0\t2215172\t11\t46\tassembly from type material\t2215172\t0\tnone\t2497\t1\t2215172\t1\tnone\tnone\tnone\tnone\tnone\tnone\t1485\tNC_022084.1\t1485\t2743\t0\t100\tCP006670.774259.775759\tArchaea;Euryarchaeota;Thermococci;Thermococcales;Thermococcaceae;Thermococcus;Thermococcus litoralis DSM 5473\t0\t19\t45\t0");
        assert!(result.is_ok());
    }
}