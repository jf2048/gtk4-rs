// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{PageRange, PrintSettings};
use glib::translate::*;

impl PrintSettings {
    #[doc(alias = "gtk_print_settings_set_page_ranges")]
    pub fn set_page_ranges(&self, page_ranges: &[PageRange]) {
        let num_ranges = page_ranges.len() as i32;
        unsafe {
            ffi::gtk_print_settings_set_page_ranges(
                self.to_glib_none().0,
                mut_override(page_ranges.to_glib_none().0),
                num_ranges,
            );
        }
    }
}

impl glib::ToVariant for PrintSettings {
    #[inline]
    fn to_variant(&self) -> glib::Variant {
        self.to_gvariant()
    }
}

impl glib::FromVariant for PrintSettings {
    #[inline]
    fn from_variant(variant: &glib::Variant) -> Option<Self> {
        if !variant.is_type(&<Self as glib::StaticVariantType>::static_variant_type()) {
            None
        } else {
            Some(Self::from_gvariant(variant))
        }
    }
}

impl glib::StaticVariantType for PrintSettings {
    #[inline]
    fn static_variant_type() -> std::borrow::Cow<'static, glib::VariantTy> {
        std::borrow::Cow::Borrowed(glib::VariantTy::VARDICT)
    }
}
