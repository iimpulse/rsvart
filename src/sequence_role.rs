#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SequenceRole {
    AssembledMolecule,
    UnlocalizedScaffold,
    UnplacedScaffold,
    FixPatch,
    NovelPatch,
    AltScaffold,
    Unknown,
}
impl From<&str> for SequenceRole {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "ASSEMBLED-MOLECULE" => SequenceRole::AssembledMolecule,
            "UNLOCALIZED-SCAFFOLD" => SequenceRole::UnlocalizedScaffold,
            "UNPLACED-SCAFFOLD" => SequenceRole::UnplacedScaffold,
            "FIX-PATCH" => SequenceRole::FixPatch,
            "NOVEL-PATCH" => SequenceRole::NovelPatch,
            "ALT-SCAFFOLD" => SequenceRole::AltScaffold,
            _ => SequenceRole::Unknown
        }
    }
}

#[cfg(test)]
mod test {
    use super::SequenceRole;
    use rstest::rstest;

    #[rstest]
    #[case("assembled-molecule", SequenceRole::AssembledMolecule)]
    #[case("unlocalized-scaffold", SequenceRole::UnlocalizedScaffold)]
    #[case("unplaced-scaffold", SequenceRole::UnplacedScaffold)]
    #[case("fix-patch", SequenceRole::FixPatch)]
    #[case("novel-patch", SequenceRole::NovelPatch)]
    #[case("alt-scaffold", SequenceRole::AltScaffold)]
    #[case("fake-nothing", SequenceRole::Unknown )]
    fn test_sequence_role_from(#[case] input: &str, #[case] expected: SequenceRole) {
        assert_eq!(SequenceRole::from(input), expected);
    }
}