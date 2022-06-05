use crate::GenomicRegion;
use crate::variant_type::VariantType;

trait Variant: GenomicRegion {

    fn id(&self) -> &str;

    fn reference(&self) -> &str;

    fn alternate(&self) -> &str;

    fn change_length(&self) -> u32;

    // TODO - I think we need to own the VariantType in the Variant implementations.
    //  We may not be able to provide a default implementation as in Java, unless we change may need to
    fn variant_type(&self) -> &VariantType;
}