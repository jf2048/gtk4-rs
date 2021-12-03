// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Expression;
use crate::Filter;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkBoolFilter")]
    pub struct BoolFilter(Object<ffi::GtkBoolFilter, ffi::GtkBoolFilterClass>) @extends Filter;

    match fn {
        type_ => || ffi::gtk_bool_filter_get_type(),
    }
}

impl BoolFilter {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BoolFilter`] objects.
    ///
    /// This method returns an instance of [`BoolFilterBuilder`](crate::builders::BoolFilterBuilder) which can be used to create [`BoolFilter`] objects.
    pub fn builder() -> BoolFilterBuilder {
        BoolFilterBuilder::default()
    }

    #[doc(alias = "gtk_bool_filter_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe { from_glib_none(ffi::gtk_bool_filter_get_expression(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bool_filter_get_invert")]
    #[doc(alias = "get_invert")]
    pub fn inverts(&self) -> bool {
        unsafe { from_glib(ffi::gtk_bool_filter_get_invert(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bool_filter_set_invert")]
    pub fn set_invert(&self, invert: bool) {
        unsafe {
            ffi::gtk_bool_filter_set_invert(self.to_glib_none().0, invert.into_glib());
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&BoolFilter) + 'static>(
            this: *mut ffi::GtkBoolFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "invert")]
    pub fn connect_invert_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_invert_trampoline<F: Fn(&BoolFilter) + 'static>(
            this: *mut ffi::GtkBoolFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::invert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_invert_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BoolFilter`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BoolFilterBuilder {
    expression: Option<Expression>,
    invert: Option<bool>,
}

impl BoolFilterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`BoolFilterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BoolFilter`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BoolFilter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expression) = self.expression {
            properties.push(("expression", expression));
        }
        if let Some(ref invert) = self.invert {
            properties.push(("invert", invert));
        }
        glib::Object::new::<BoolFilter>(&properties)
            .expect("Failed to create an instance of BoolFilter")
    }

    pub fn expression(mut self, expression: &Expression) -> Self {
        self.expression = Some(expression.clone());
        self
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = Some(invert);
        self
    }
}

impl fmt::Display for BoolFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BoolFilter")
    }
}
