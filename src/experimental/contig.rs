use std::cmp::Ordering;

use crate::contig::{AssignedMoleculeType, SequenceRole};

#[derive(Debug)]
pub struct Contig {
    id: usize,
    name: String,
    sequence_role: SequenceRole,
    assigned_molecule: String,
    assigned_molecule_type: AssignedMoleculeType,
    length: usize,
    gen_bank_accession: String,
    ref_seq_accession: String,
    ucsc_name: String,
}

impl Contig {
    pub fn new(id: usize,
               name: String,
               sequence_role: SequenceRole,
               assigned_molecule: String,
               assigned_molecule_type: AssignedMoleculeType,
               length: usize,
               gen_bank_accession: String,
               ref_seq_accession: String,
               ucsc_name: String) -> Contig {
        Contig {
            id,
            name,
            sequence_role,
            assigned_molecule,
            assigned_molecule_type,
            length,
            gen_bank_accession,
            ref_seq_accession,
            ucsc_name,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
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
    pub fn length(&self) -> usize {
        self.length
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

    pub fn is_unknown(&self) -> bool {
        self.id == 0
    }
}

impl PartialEq<Contig> for Contig {
    fn eq(&self, other: &Contig) -> bool {
        self.id() == other.id()
    }
}

impl PartialOrd<Contig> for Contig {
    fn partial_cmp(&self, other: &Contig) -> Option<Ordering> {
        self.id().partial_cmp(&other.id())
    }
}


#[cfg(test)]
mod contig_tests {
    use std::rc::Rc;
    use crate::contig::{AssignedMoleculeType, SequenceRole};

    use super::Contig;

    #[test]
    fn new_contig() {
        let contig = get_contig();
        assert_eq!(contig.id(), 1);
        assert_eq!(contig.name(), "1");
        assert_eq!(contig.sequence_role(), &SequenceRole::AssembledMolecule);
        assert_eq!(contig.assigned_molecule(), "1");
        assert_eq!(contig.assigned_molecule_type(), &AssignedMoleculeType::Chromosome);
        assert_eq!(contig.length(), 249_250_621);
        assert_eq!(contig.gen_bank_accession(), "CM000663.1");
        assert_eq!(contig.ref_seq_accession(), "NC_000001.10");
        assert_eq!(contig.ucsc_name(), "chr1");
        assert_eq!(contig.is_unknown(), false)
    }

    #[test]
    fn contig_size() {
        assert_eq!(std::mem::size_of::<usize>(), 8); // usize
        assert_eq!(std::mem::size_of::<SequenceRole>(), 1); // enum
        assert_eq!(std::mem::size_of::<String>(), 24); // pointer, length, and capacity

        assert_eq!(std::mem::size_of::<Contig>(), 144);
        assert_eq!(std::mem::size_of::<Rc>(), 8);
    }

    fn get_contig() -> Contig {
        Contig::new(
            1,
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