use crate::genomic::AssignedMoleculeType;
use crate::genomic::Contig;
use crate::genomic::SequenceRole;

pub fn get_contigs() -> Vec<Contig<u32>> {
    vec![
        Contig::new("1".to_string(),SequenceRole::AssembledMolecule,"1".to_string(),AssignedMoleculeType::Chromosome,249_250_621,"CM000663.1".to_string(),"NC_000001.10".to_string(),"chr1".to_string()).unwrap(),
        Contig::new("2".to_string(),SequenceRole::AssembledMolecule,"2".to_string(),AssignedMoleculeType::Chromosome,242_193_529,"CM000664.2".to_string(),"NC_000002.12".to_string(),"chr2".to_string()).unwrap(),
        Contig::new("X".to_string(),SequenceRole::AssembledMolecule,"X".to_string(),AssignedMoleculeType::Chromosome,156_040_895,"CM000685.2".to_string(),"NC_000023.11".to_string(),"chrX".to_string()).unwrap(),
        Contig::new("Y".to_string(),SequenceRole::AssembledMolecule,"Y".to_string(),AssignedMoleculeType::Chromosome,57_227_415,"CM000686.2".to_string(),"NC_000024.10".to_string(),"chrY".to_string()).unwrap(),
        Contig::new("MT".to_string(),SequenceRole::AssembledMolecule,"MT".to_string(),AssignedMoleculeType::Mitochondrion,16_569,"J01415.2".to_string(),"NC_012920.1".to_string(),"chrM".to_string()).unwrap()
    ]
}
