use crate::Contig;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenomeBuildIdentifier {
    major_assembly: String,
    patch: String,
}

impl GenomeBuildIdentifier {
    pub fn new(major_assembly: String, patch: String) -> Self {
        GenomeBuildIdentifier {
            major_assembly,
            patch,
        }
    }

    pub fn major_assembly(&self) -> &str {
        &self.major_assembly
    }
    pub fn patch(&self) -> &str {
        &self.patch
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenomeBuild<C> {
    id: GenomeBuildIdentifier,
    contigs: Vec<Contig<C>>,
}

impl<C> GenomeBuild<C> {
    pub fn new(id: GenomeBuildIdentifier, mut contigs: Vec<Contig<C>>) -> Self {
        contigs.sort_by(|l, r| l.name().cmp(r.name()));
        GenomeBuild { id, contigs }
    }

    pub fn id(&self) -> &GenomeBuildIdentifier {
        &self.id
    }

    pub fn contigs(&self) -> &[Contig<C>] {
        &self.contigs
    }

    pub fn contig_from_genbank(&self, name: &str) -> Option<&Contig<C>> {
        self.contigs
            .iter()
            .find(|&c| c.name().eq(name) || c.gen_bank_accession().eq(name))
    }

    pub fn contig_from_refseq(&self, name: &str) -> Option<&Contig<C>> {
        self.contigs
            .iter()
            .find(|&c| c.name().eq(name) || c.ref_seq_accession().eq(name))
    }

    pub fn contig_from_ucsc(&self, name: &str) -> Option<&Contig<C>> {
        self.contigs
            .iter()
            .find(|&c| c.name().eq(name) || c.ucsc_name().eq(name))
    }
}

#[cfg(test)]
mod test {
    use super::GenomeBuildIdentifier;
    use super::GenomeBuild;
    use rstest::rstest;
    use crate::{AssignedMoleculeType, Contig, SequenceRole};

    #[rstest]
    fn test_genome_build_identifier() {
        let id = GenomeBuildIdentifier::new("GRCh38".to_string(), "p13".to_string());
        assert_eq!(id.major_assembly(), "GRCh38");
        assert_eq!(id.patch(), "p13");
    }

    #[rstest]
    fn test_genome_build() {
        let id = GenomeBuildIdentifier::new("GRCh38".to_string(), "p13".to_string());
        let contigs:Vec<Contig<u8>> = get_few_contigs();
        let build = GenomeBuild::new(id, contigs);
        assert_eq!(build.id().major_assembly(), "GRCh38");
        assert_eq!(build.id().patch(), "p13");
        assert_eq!(build.contigs().len(), 3);
        assert_eq!(*build.contig_from_genbank("CM000664.1").unwrap(), *&build.contigs()[1]);
        assert_eq!(*build.contig_from_refseq("NC_000001.10").unwrap(), *&build.contigs()[0]);
        assert_eq!(*build.contig_from_ucsc("chr3").unwrap(), *&build.contigs()[2]);
    }

    fn get_few_contigs() -> Vec<Contig<u8>>{
        vec![Contig::new(
            "1".to_string(),
            SequenceRole::AssembledMolecule,
            "1".to_string(),
            AssignedMoleculeType::Chromosome,60,
            "CM000663.1".to_string(),
            "NC_000001.10".to_string(),
            "chr1".to_string())
            .unwrap(), Contig::new(
            "2".to_string(),
            SequenceRole::AssembledMolecule,
            "2".to_string(),
            AssignedMoleculeType::Chromosome,60,
            "CM000664.1".to_string(),
            "NC_000002.10".to_string(),
            "chr2".to_string())
            .unwrap(), Contig::new(
            "3".to_string(),
            SequenceRole::AssembledMolecule,
            "3".to_string(),
            AssignedMoleculeType::Chromosome,60,
            "CM000665.1".to_string(),
            "NC_000003.10".to_string(),
            "chr3".to_string())
            .unwrap()]
    }
}


