#[derive(Debug, Copy, Clone, PartialEq)]
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
impl From<&str> for AssignedMoleculeType {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "CHROMOSOME" => AssignedMoleculeType::Chromosome,
            "MITOCHONDRION" => AssignedMoleculeType::Mitochondrion,
            "CHLOROPLAST" => AssignedMoleculeType::Chloroplast,
            "MITOCHONDRIAL PLASMID" => AssignedMoleculeType::MitochondrialPlasmid,
            "PLASMID" => AssignedMoleculeType::Plasmid,
            "SEGMENT" => AssignedMoleculeType::Segment,
            "LINKAGE GROUP" => AssignedMoleculeType::LinkageGroup,
            "NA" | _ => AssignedMoleculeType::Unknown,
        }
    }
}

#[cfg(test)]
mod test {
    use super::AssignedMoleculeType;
    use rstest::rstest;

    #[rstest]
    #[case("chromosome", AssignedMoleculeType::Chromosome)]
    #[case("mitochondrion", AssignedMoleculeType::Mitochondrion)]
    #[case("chloroplast", AssignedMoleculeType::Chloroplast)]
    #[case("plasmid", AssignedMoleculeType::Plasmid)]
    #[case("segment", AssignedMoleculeType::Segment)]
    #[case("linkage group", AssignedMoleculeType::LinkageGroup)]
    #[case("na", AssignedMoleculeType::Unknown)]
    #[case("fake molecule", AssignedMoleculeType::Unknown)]
    fn test_assigned_molecule_from(#[case] input: &str, #[case] expected: AssignedMoleculeType) {
        assert_eq!(AssignedMoleculeType::from(input), expected);
    }
}