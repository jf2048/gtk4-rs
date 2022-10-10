// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PageSetup;

impl glib::ToVariant for PageSetup {
    #[inline]
    fn to_variant(&self) -> glib::Variant {
        self.to_gvariant()
    }
}

impl glib::FromVariant for PageSetup {
    #[inline]
    fn from_variant(variant: &glib::Variant) -> Option<Self> {
        if !variant.is_type(&<Self as glib::StaticVariantType>::static_variant_type()) {
            None
        } else {
            Some(Self::from_gvariant(variant))
        }
    }
}

impl glib::StaticVariantType for PageSetup {
    #[inline]
    fn static_variant_type() -> std::borrow::Cow<'static, glib::VariantTy> {
        std::borrow::Cow::Borrowed(glib::VariantTy::VARDICT)
    }
}
