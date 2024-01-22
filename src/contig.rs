use std::cmp::Ordering;

use crate::assigned_molecule_type::AssignedMoleculeType;
use crate::sequence_role::SequenceRole;
use crate::err::SvartError;

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

    pub fn of(id: usize,
              name: String,
              sequence_role: SequenceRole,
              assigned_molecule: String,
              assigned_molecule_type: AssignedMoleculeType,
              length: usize,
              gen_bank_accession: String,
              ref_seq_accession: String,
              ucsc_name: String) -> Result<Contig, SvartError> {
        if id == 0 {
            return Err(SvartError::IllegalValueError("Id 0 is reserved for the unknown contig"));
        }

        Ok(Contig {
            id,
            name,
            sequence_role,
            assigned_molecule,
            assigned_molecule_type,
            length,
            gen_bank_accession,
            ref_seq_accession,
            ucsc_name,
        })
    }

    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn sequence_role(&self) -> &SequenceRole {
        &self.sequence_role
    }

    fn assigned_molecule(&self) -> &str {
        self.assigned_molecule.as_str()
    }

    fn assigned_molecule_type(&self) -> &AssignedMoleculeType {
        &self.assigned_molecule_type
    }

    fn length(&self) -> usize {
        self.length
    }

    fn gen_bank_accession(&self) -> &str {
        self.gen_bank_accession.as_str()
    }

    fn ref_seq_accession(&self) -> &str {
        self.ref_seq_accession.as_str()
    }

    fn ucsc_name(&self) -> &str {
        self.ucsc_name.as_str()
    }
}

impl PartialEq for Contig {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id()
            && self.name() == other.name()
            && self.sequence_role() == other.sequence_role()
            && self.assigned_molecule() == other.assigned_molecule()
            && self.assigned_molecule_type() == other.assigned_molecule_type()
            && self.length() == other.length()
            && self.gen_bank_accession() == other.gen_bank_accession()
            && self.ref_seq_accession() == other.ref_seq_accession()
            && self.ucsc_name() == other.ucsc_name()
    }
}

impl PartialOrd for Contig {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_contig() {
        let contig = get_contig();
        assert!(contig.is_ok());

        let contig = contig.unwrap();
        assert_eq!(contig.id(), 1);
        assert_eq!(contig.name(), "1");
        assert_eq!(contig.sequence_role(), &SequenceRole::AssembledMolecule);
        assert_eq!(contig.assigned_molecule(), "1");
        assert_eq!(contig.assigned_molecule_type(), &AssignedMoleculeType::Chromosome);
        assert_eq!(contig.gen_bank_accession(), "CM000663.1");
        assert_eq!(contig.ref_seq_accession(), "NC_000001.10");
        assert_eq!(contig.ucsc_name(), "chr1");
    }

    #[test]
    fn test_contig_from_bad_id() {
        let contig = Contig::of(
            0,
            "1".to_string(),
            SequenceRole::AssembledMolecule,
            "1".to_string(),
            AssignedMoleculeType::Chromosome,
            249_250_621,
            "CM000663.1".to_string(),
            "NC_000001.10".to_string(),
            "chr1".to_string());
        assert!(contig.is_err());
        assert_eq!(contig.err().unwrap().to_string(), "Illegal value error: Id 0 is reserved for the unknown contig".to_string())
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
        let two = Contig::of(2, "2".to_string(),
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

    fn get_contig() -> Result<Contig, SvartError> {
        Contig::of(
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
