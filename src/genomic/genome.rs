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

    #[rstest]
    fn test_geneome_build_identifier() {
        let id = GenomeBuildIdentifier::new("GRCh38".to_string(), "p13".to_string());
        assert_eq!(id.major_assembly(), "GRCh38");
        assert_eq!(id.patch(), "p13");
    }

    #[rstest]
    fn test_genome_build() {
        let id = GenomeBuildIdentifier::new("GRCh38".to_string(), "p13".to_string());
        let contigs = vec![];
        let build = GenomeBuild::new(id, contigs);
        assert_eq!(build.id().major_assembly(), "GRCh38");
        assert_eq!(build.id().patch(), "p13");
        assert_eq!(build.contigs().len(), 0);
    }

    fn get_few_contigs(){
        // create me a few fake contigs

    }
}


