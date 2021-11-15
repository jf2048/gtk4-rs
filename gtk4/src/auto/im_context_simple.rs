// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::IMContext;
use crate::InputHints;
use crate::InputPurpose;
use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkIMContextSimple")]
    pub struct IMContextSimple(Object<ffi::GtkIMContextSimple, ffi::GtkIMContextSimpleClass>) @extends IMContext;

    match fn {
        type_ => || ffi::gtk_im_context_simple_get_type(),
    }
}

impl IMContextSimple {
    #[doc(alias = "gtk_im_context_simple_new")]
    pub fn new() -> IMContextSimple {
        assert_initialized_main_thread!();
        unsafe { IMContext::from_glib_full(ffi::gtk_im_context_simple_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`IMContextSimple`] objects.
    ///
    /// This method returns an instance of [`IMContextSimpleBuilder`](crate::builders::IMContextSimpleBuilder) which can be used to create [`IMContextSimple`] objects.
    pub fn builder() -> IMContextSimpleBuilder {
        IMContextSimpleBuilder::default()
    }
}

impl Default for IMContextSimple {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`IMContextSimple`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct IMContextSimpleBuilder {
    input_hints: Option<InputHints>,
    input_purpose: Option<InputPurpose>,
}

impl IMContextSimpleBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`IMContextSimpleBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`IMContextSimple`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> IMContextSimple {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        glib::Object::new::<IMContextSimple>(&properties)
            .expect("Failed to create an instance of IMContextSimple")
    }

    pub fn input_hints(mut self, input_hints: InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }
}

impl IMContextSimple {
    pub const NONE: Option<&'static IMContextSimple> = None;
}

impl fmt::Display for IMContextSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IMContextSimple")
    }
}
