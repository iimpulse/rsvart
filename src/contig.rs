use std::cmp::Ordering;
use std::fmt::{Debug, Error};
use std::string::ParseError;

use self::err::SvartError;

// ------------------------------------------- CONTIG ----------------------------------------------
// TODO - require implementation of PartialOrd
pub trait Contig {
    fn id(&self) -> usize;
    fn name(&self) -> &str;
    fn sequence_role(&self) -> &SequenceRole;
    fn assigned_molecule(&self) -> &str;
    fn assigned_molecule_type(&self) -> &AssignedMoleculeType;
    fn length(&self) -> usize;
    fn gen_bank_accession(&self) -> &str;
    fn ref_seq_accession(&self) -> &str;
    fn ucsc_name(&self) -> &str;
}

// ----------------------------------- DEFAULT CONTIG ----------------------------------------------

#[derive(Debug)]
pub struct ContigDefault {
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

impl ContigDefault {
    pub fn of(
        id: usize,
        name: String,
        sequence_role: SequenceRole,
        assigned_molecule: String,
        assigned_molecule_type: AssignedMoleculeType,
        length: usize,
        gen_bank_accession: String,
        ref_seq_accession: String,
        ucsc_name: String) -> Result<ContigDefault, SvartError> {
        if id == 0 {
            return Err(SvartError::IllegalValueError("Id 0 is reserved for the unknown contig".to_string()));
        }

        Ok(ContigDefault {
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
}

impl Contig for ContigDefault {
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

impl<T: Contig> PartialEq<T> for ContigDefault {
    fn eq(&self, other: &T) -> bool {
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

impl<T: Contig> PartialOrd<T> for ContigDefault {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.id.partial_cmp(&other.id())
    }
}

// ----------------------------------- UNKNOWN CONTIG ----------------------------------------------

#[derive(Debug, PartialEq)]
pub struct UnknownContig {
    id: usize,
    name: &'static str,
    sequence_role: SequenceRole,
    assigned_molecule: &'static str,
    assigned_molecule_type: AssignedMoleculeType,
    length: usize,
    gen_bank_accession: &'static str,
    ref_seq_accession: &'static str,
    ucsc_name: &'static str,
}

impl Contig for UnknownContig {
    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn sequence_role(&self) -> &SequenceRole {
        &self.sequence_role
    }

    fn assigned_molecule(&self) -> &str {
        self.assigned_molecule
    }

    fn assigned_molecule_type(&self) -> &AssignedMoleculeType {
        &self.assigned_molecule_type
    }

    fn length(&self) -> usize {
        self.length
    }

    fn gen_bank_accession(&self) -> &str {
        self.gen_bank_accession
    }

    fn ref_seq_accession(&self) -> &str {
        self.ref_seq_accession
    }

    fn ucsc_name(&self) -> &str {
        self.ucsc_name
    }
}

// TODO - PartialEq, PartialOrd for UnknownContig

// singleton UnknownContig
const UNKNOWN_CONTIG: UnknownContig = UnknownContig {
    id: 0,
    name: "na",
    sequence_role: SequenceRole::Unknown,
    assigned_molecule: "na",
    assigned_molecule_type: AssignedMoleculeType::Unknown,
    length: 0,
    gen_bank_accession: "",
    ref_seq_accession: "",
    ucsc_name: "na",
};

pub fn unknown_contig() -> &'static UnknownContig {
    &UNKNOWN_CONTIG
}

// -------------------------------------- SEQUENCE ROLE --------------------------------------------

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SequenceRole {
    AssembledMolecule,
    UnlocalizedScaffold,
    UnplacedScaffold,
    FixPatch,
    NovelPatch,
    AltScaffold,
    Unknown,
}

impl SequenceRole {
    pub fn parse_role(value: &str) -> &SequenceRole {
        match value {
            "assembled-molecule" => &SequenceRole::AssembledMolecule,
            "unlocalized-scaffold" => &SequenceRole::UnlocalizedScaffold,
            "unplaced-scaffold" => &SequenceRole::UnplacedScaffold,
            "fix-patch" => &SequenceRole::FixPatch,
            "novel-patch" => &SequenceRole::NovelPatch,
            "alt-scaffold" => &SequenceRole::AltScaffold,
            _ => &SequenceRole::Unknown,
        }
    }
}

// -------------------------------------- ASSIGNED MOLECULE TYPE -----------------------------------

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AssignedMoleculeType {
    Chromosome,
    Mitochondrion,
    Chloroplast,
    MitochondrialPlasmid,
    Plasmid,
    Segment,
    LinkageGroup,
    Unknown,
}

impl AssignedMoleculeType {
    pub fn parse_molecule_type(value: &str) -> &AssignedMoleculeType {
        match value {
            "Chromosome" => &AssignedMoleculeType::Chromosome,
            "Mitochondrion" => &AssignedMoleculeType::Mitochondrion,
            "Chloroplast" => &AssignedMoleculeType::Chloroplast,
            "Mitochondrial Plasmid" => &AssignedMoleculeType::MitochondrialPlasmid,
            "Plasmid" => &AssignedMoleculeType::Plasmid,
            "Segment" => &AssignedMoleculeType::Segment,
            "Linkage Group" => &AssignedMoleculeType::LinkageGroup,
            "na" | _ => &AssignedMoleculeType::Unknown,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // These are just SOME and not necessarily all tests

    #[test]
    fn test_default_contig() {
        let contig = ContigDefault::of(
            1,
            "1".to_string(),
            SequenceRole::AssembledMolecule,
            "1".to_string(),
            AssignedMoleculeType::Chromosome,
            249_250_621,
            "CM000663.1".to_string(),
            "NC_000001.10".to_string(),
            "chr1".to_string());
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
        let contig = ContigDefault::of(
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
        assert_eq!(contig.err().unwrap().to_string(), "Illegal value error: id 0 is reserved for the unknown contig".to_string())
    }

    #[test]
    fn test_unknown_contig() {
        let contig = unknown_contig();

        assert_eq!(contig.id(), 0);
        assert_eq!(contig.name(), "na");
        assert_eq!(contig.sequence_role(), &SequenceRole::Unknown);
        assert_eq!(contig.assigned_molecule(), "na");
        assert_eq!(
            contig.assigned_molecule_type(),
            &AssignedMoleculeType::Unknown
        );
        assert_eq!(contig.length(), 0);
        assert_eq!(contig.gen_bank_accession(), "");
        assert_eq!(contig.ref_seq_accession(), "");
        assert_eq!(contig.ucsc_name(), "na");
    }
}
