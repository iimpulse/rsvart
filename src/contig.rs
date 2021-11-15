use std::cmp::Ordering;
use std::fmt::Debug;


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

#[derive(Debug)]
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

impl<T: Contig> PartialEq<T> for UnknownContig {
    fn eq(&self, other: &T) -> bool {
        self.id() == other.id()
    }
}

impl<T: Contig> PartialOrd<T> for UnknownContig {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

pub fn unknown_contig() -> &'static UnknownContig {
    return &UnknownContig {
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
    use crate::default::ContigDefault;

    use super::*;

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

    #[test]
    fn test_eq() {
        let one = unknown_contig();
        let two = unknown_contig();
        assert_eq!(one, two);

        let other = get_contig();
        assert_ne!(one, &other);
    }

    #[test]
    fn test_cmp() {
        let one = unknown_contig();
        let two = unknown_contig();
        assert_eq!(one, two);

        let other = get_contig();
        assert_eq!(one, two);
        assert!(!(one > two));
        assert!(!(one < two));
        assert!(one < &other);
        assert!(&other > one);
    }

    fn get_contig() -> ContigDefault {
        ContigDefault::of(1, "1".to_string(),
                          SequenceRole::AssembledMolecule, "1".to_string(),
                          AssignedMoleculeType::Chromosome, 249_250_621,
                          "CM000663.1".to_string(),
                          "NC_000001.10".to_string(),
                          "chr1".to_string())
            .unwrap()
    }
}
