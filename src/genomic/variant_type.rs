#[derive(PartialEq)]
pub enum VariantType {
    Unknown,
    SingleNucleotide,
    MultiNucleotide,
    Symbolic,
    Deletion,
    DeletionME,
    DeletionALU,
    DeletionL1,
    DeletionSVA,
    DeletionHERV,
    Insertion,
    InsertionME,
    InsertionALU,
    InsertionL1,
    InsertionSVA,
    InsertionHERV,

    Duplication,
    DuplicationTandem,
    DuplicationInversionBefore,
    DuplicationInversionAfter,

    Inversion,
    CopyNumber,
    Breakend,

    CopyNumberGain,
    CopyNumberLoss,
    CopyNumberLOH,
    CopyNumberComplex,
    ShortTandemRepeat,
    Translocation
}
impl VariantType {
    pub fn parse_type_vcf(alt: &str) -> VariantType {
        if alt.is_empty() {
            return VariantType::Unknown;
        }
        let stripped: &str = VariantType::trim_angle_brackets(alt);
        let variant_type = match stripped {
            "SNP" | "SNV" => VariantType::SingleNucleotide,
            "MNP" | "MNV" => VariantType::MultiNucleotide,
            "DEL" => VariantType::Deletion,
            "INS" => VariantType::Insertion,
            "DUP" => VariantType::Duplication,
            "INV" => VariantType::Inversion,
            "CNV" => VariantType::CopyNumber,
            "BND" => VariantType::Breakend,
            "STR" => VariantType::ShortTandemRepeat,
            "TRA" => VariantType::Translocation,

            "DEL:ME" => VariantType::DeletionME,
            "DEL:ME:ALU" => VariantType::DeletionALU,
            "DEL:ME:LINE1" => VariantType::DeletionL1,
            "DEL:ME:SVA" => VariantType::DeletionSVA,
            "DEL:ME:HERV" => VariantType::DeletionHERV,

            "INS:ME" => VariantType::InsertionME,
            "INS:ME:ALU" => VariantType::InsertionALU,
            "INS:ME:LINE1" => VariantType::InsertionL1,
            "INS:ME:SVA" => VariantType::InsertionSVA,
            "INS:ME:HERV" => VariantType::InsertionHERV,

            "DUP:TANDEM" => VariantType::DuplicationTandem,
            "DUP:INV-BEFORE" => VariantType::DuplicationInversionBefore,
            "DUP:INV-AFTER" => VariantType::DuplicationInversionAfter,

            "CNV:GAIN" => VariantType::CopyNumberGain,
            "CNV:LOSS" => VariantType::CopyNumberLoss,
            &_ => VariantType::Unknown
        };

        if variant_type == VariantType::Unknown {
            return if stripped.starts_with("BND") || VariantType::is_breakend(stripped) {
                VariantType::Breakend
            } else if stripped.starts_with("DEL:ME") {
                VariantType::DeletionME
            } else if stripped.starts_with("DEL") {
                VariantType::Deletion
            } else if stripped.starts_with("INS:ME") {
                VariantType::InsertionME
            } else if stripped.starts_with("DUP:TANDEM") {
                VariantType::DuplicationTandem
            } else if stripped.starts_with("DUP") {
                VariantType::Duplication
            } else if stripped.starts_with("CNV") {
                VariantType::CopyNumber
            } else if stripped.starts_with("STR") {
                VariantType::ShortTandemRepeat
            } else if VariantType::is_symbolic(alt) {
                VariantType::Symbolic
            } else {
                VariantType::Unknown
            }
        }
        return variant_type;
    }

    pub fn parse_type(refr: &str, alt: &str) -> VariantType{
        if VariantType::is_symbolic_alleles(refr, alt){
            return VariantType::parse_type_vcf(alt);
        }

        if refr.len() == alt.len() {
            if alt.len() == 1 {
                return VariantType::SingleNucleotide;
            }
            return VariantType::MultiNucleotide;
        }

        return if refr.len() < alt.len() {VariantType::Insertion} else {VariantType::Deletion}
    }

    pub fn is_symbolic_alleles(refr: &str, alt: &str) -> bool {
        return VariantType::is_symbolic(alt) || VariantType::is_symbolic(refr);
    }

    pub fn is_symbolic(allele: &str) -> bool {
        return VariantType::is_large_symbolic(allele) || VariantType::is_breakend(allele);
    }

    pub fn is_breakend(allele: &str) -> bool {
        return VariantType::is_single_breakend(allele) || VariantType::is_mated_breakend(allele);
    }

    pub fn is_large_symbolic(allele: &str) -> bool {
        return allele.len() > 1 && (allele.chars().next().unwrap() == '<' || allele.chars().last().unwrap() == '>');
    }

    pub fn is_single_breakend(allele: &str) -> bool {
        return allele.len() > 1 && (allele.chars().next().unwrap() == '.' || allele.chars().last().unwrap() == '.');
    }

    pub fn is_mated_breakend(allele: &str) -> bool {
        return allele.len() > 1 && (allele.contains("[") || allele.contains("]"));
    }

    pub fn require_non_symbolic(alt: &str) -> &str {
        if alt.is_empty() || VariantType::is_symbolic(alt) {
            panic!("Illegal symbolic alt allele {item}", item = alt);
        } else if alt.contains(",") {
            panic!("Illegal multi-allelic alt allele {item}", item = alt);
        }
        alt
    }

    pub fn require_symbolic(alt: &str) -> &str {
        if alt.is_empty() || !VariantType::is_large_symbolic(alt) {
            panic!("Illegal non-symbolic or breakend alt allele {item}", item = alt);
        }
        alt
    }

    pub fn require_breakend(alt: &str) -> &str {
        if alt.is_empty() || !VariantType::is_breakend(alt){
            panic!("Illegal non-breakend allele {item}", item = alt);
        }
        alt
    }

    pub fn require_non_breakend(alt: &str) -> &str {
        if alt.is_empty() || VariantType::is_breakend(alt) {
            panic!("Illegal breakend allele {item}", item = alt);
        }
        alt
    }

    pub fn is_missing_upstream_deletion(allele: &str) -> bool {
            return allele.eq("*")
    }

    pub fn is_missing(allele: &str) -> bool {
        return allele.eq(".");
    }

    fn trim_angle_brackets(value: &str) -> &str {
        if value.starts_with("<") && value.ends_with(">") {
            return &value[1..value.len()-1]
        }
        value
    }
}

#[cfg(test)]
mod test {
    use crate::{AssignedMoleculeType, Strand};
    use crate::VariantType;
    use rstest::rstest;

    #[rstest]
    #[case("AT", VariantType::SingleNucleotide)]
    fn test_parse_ref_alt(#[case] input: &str, #[case] expected: VariantType){
        
    }
}