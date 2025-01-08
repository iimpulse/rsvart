use crate::{AssignedMoleculeType, SequenceRole, Located, Unit};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Contig<C> {
    name: String,
    sequence_role: SequenceRole,
    assigned_molecule: String,
    assigned_molecule_type: AssignedMoleculeType,
    gen_bank_accession: String,
    ref_seq_accession: String,
    ucsc_name: String,
    start: C,
    end: C,
}

impl<C> Contig<C> {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn sequence_role(&self) -> &SequenceRole {
        &self.sequence_role
    }

    pub fn assigned_molecule(&self) -> &str {
        self.assigned_molecule.as_str()
    }

    pub fn assigned_molecule_type(&self) -> &AssignedMoleculeType {
        &self.assigned_molecule_type
    }

    pub fn gen_bank_accession(&self) -> &str {
        self.gen_bank_accession.as_str()
    }

    pub fn ref_seq_accession(&self) -> &str {
        self.ref_seq_accession.as_str()
    }

    pub fn ucsc_name(&self) -> &str {
        self.ucsc_name.as_str()
    }
}

impl<C> Contig<C> where C: Unit {
    pub fn new (
        name: String,
        sequence_role: SequenceRole,
        assigned_molecule: String,
        assigned_molecule_type: AssignedMoleculeType,
        length: C,
        gen_bank_accession: String,
        ref_seq_accession: String,
        ucsc_name: String) -> Option<Self> {
        
        if length < C::zero() {
            None
        } else {
            Some(Self {
                name,
                sequence_role,
                assigned_molecule,
                assigned_molecule_type,
                gen_bank_accession,
                ref_seq_accession,
                ucsc_name,
                start: C::zero(),
                end: length,
            })
        }
    }
}

impl<C> Located<C> for Contig<C> where C: Unit {
    fn start(&self) -> &C {
        &self.start
    }

    fn end(&self) -> &C {
        &self.end
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_contig() {
        let contig = get_contig();
        assert!(contig.is_some());

        let contig = contig.unwrap();
        assert_eq!(contig.name(), "1");
        assert_eq!(contig.sequence_role(), &SequenceRole::AssembledMolecule);
        assert_eq!(contig.assigned_molecule(), "1");
        assert_eq!(contig.assigned_molecule_type(), &AssignedMoleculeType::Chromosome);
        assert_eq!(contig.gen_bank_accession(), "CM000663.1");
        assert_eq!(contig.ref_seq_accession(), "NC_000001.10");
        assert_eq!(contig.ucsc_name(), "chr1");
    }

    #[test]
    fn test_eq() {
        let one = get_contig().unwrap();
        let two = get_contig().unwrap();
        assert_eq!(one, two);
    }

    #[test]
    fn test_cmp() {
        let one = get_contig().unwrap();
        let two = Contig::new("2".to_string(),
                             SequenceRole::AssembledMolecule, "2".to_string(),
                             AssignedMoleculeType::Chromosome,
                             123_456,
                             "".to_string(),
                             "".to_string(),
                             "".to_string()).unwrap();
        assert_ne!(one, two);
        assert!(one < two);
        assert!(two > one);
    }

    fn get_contig() -> Option<Contig<u32>> {
        Contig::new(
            "1".to_string(),
            SequenceRole::AssembledMolecule,
            "1".to_string(),
            AssignedMoleculeType::Chromosome,
            249_250_621,
            "CM000663.1".to_string(),
            "NC_000001.10".to_string(),
            "chr1".to_string())
    }
}
