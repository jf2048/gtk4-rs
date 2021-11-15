// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ShortcutTrigger;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkAlternativeTrigger")]
    pub struct AlternativeTrigger(Object<ffi::GtkAlternativeTrigger, ffi::GtkAlternativeTriggerClass>) @extends ShortcutTrigger;

    match fn {
        type_ => || ffi::gtk_alternative_trigger_get_type(),
    }
}

impl AlternativeTrigger {
    #[doc(alias = "gtk_alternative_trigger_new")]
    pub fn new(
        first: &impl IsA<ShortcutTrigger>,
        second: &impl IsA<ShortcutTrigger>,
    ) -> AlternativeTrigger {
        skip_assert_initialized!();
        unsafe {
            ShortcutTrigger::from_glib_full(ffi::gtk_alternative_trigger_new(
                first.as_ref().to_glib_full(),
                second.as_ref().to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AlternativeTrigger`] objects.
    ///
    /// This method returns an instance of [`AlternativeTriggerBuilder`](crate::builders::AlternativeTriggerBuilder) which can be used to create [`AlternativeTrigger`] objects.
    pub fn builder() -> AlternativeTriggerBuilder {
        AlternativeTriggerBuilder::default()
    }

    #[doc(alias = "gtk_alternative_trigger_get_first")]
    #[doc(alias = "get_first")]
    pub fn first(&self) -> ShortcutTrigger {
        unsafe {
            from_glib_none(ffi::gtk_alternative_trigger_get_first(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_alternative_trigger_get_second")]
    #[doc(alias = "get_second")]
    pub fn second(&self) -> ShortcutTrigger {
        unsafe {
            from_glib_none(ffi::gtk_alternative_trigger_get_second(
                self.to_glib_none().0,
            ))
        }
    }
}

impl Default for AlternativeTrigger {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct AlternativeTrigger object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AlternativeTrigger`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct AlternativeTriggerBuilder {
    first: Option<ShortcutTrigger>,
    second: Option<ShortcutTrigger>,
}

impl AlternativeTriggerBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AlternativeTriggerBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AlternativeTrigger`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> AlternativeTrigger {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref first) = self.first {
            properties.push(("first", first));
        }
        if let Some(ref second) = self.second {
            properties.push(("second", second));
        }
        glib::Object::new::<AlternativeTrigger>(&properties)
            .expect("Failed to create an instance of AlternativeTrigger")
    }

    pub fn first(mut self, first: &impl IsA<ShortcutTrigger>) -> Self {
        self.first = Some(first.clone().upcast());
        self
    }

    pub fn second(mut self, second: &impl IsA<ShortcutTrigger>) -> Self {
        self.second = Some(second.clone().upcast());
        self
    }
}

impl fmt::Display for AlternativeTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AlternativeTrigger")
    }
}
