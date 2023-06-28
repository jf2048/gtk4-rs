// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Accessible, AccessibleRole, Actionable, Align, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCheckButton")]
    pub struct CheckButton(Object<ffi::GtkCheckButton, ffi::GtkCheckButtonClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Actionable;

    match fn {
        type_ => || ffi::gtk_check_button_get_type(),
    }
}

impl CheckButton {
    pub const NONE: Option<&'static CheckButton> = None;

    #[doc(alias = "gtk_check_button_new")]
    pub fn new() -> CheckButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_check_button_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_check_button_new_with_label")]
    #[doc(alias = "new_with_label")]
    pub fn with_label(label: &str) -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new_with_label(label.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_check_button_new_with_mnemonic")]
    #[doc(alias = "new_with_mnemonic")]
    pub fn with_mnemonic(label: &str) -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new_with_mnemonic(
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CheckButton`] objects.
    ///
    /// This method returns an instance of [`CheckButtonBuilder`](crate::builders::CheckButtonBuilder) which can be used to create [`CheckButton`] objects.
    pub fn builder() -> CheckButtonBuilder {
        CheckButtonBuilder::new()
    }
}

impl Default for CheckButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CheckButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CheckButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, CheckButton>,
}

impl CheckButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn group(self, group: &impl IsA<CheckButton>) -> Self {
        Self {
            builder: self.builder.property("group", group.clone().upcast()),
        }
    }

    pub fn inconsistent(self, inconsistent: bool) -> Self {
        Self {
            builder: self.builder.property("inconsistent", inconsistent),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CheckButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CheckButton {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::CheckButton>> Sealed for T {}
}

pub trait CheckButtonExt: IsA<CheckButton> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_check_button_get_active")]
    #[doc(alias = "get_active")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_button_get_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_check_button_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_check_button_get_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_check_button_get_inconsistent")]
    #[doc(alias = "get_inconsistent")]
    fn is_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_button_get_inconsistent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_check_button_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_check_button_get_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_check_button_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    fn uses_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_button_get_use_underline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_check_button_set_active")]
    fn set_active(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_button_set_active(self.as_ref().to_glib_none().0, setting.into_glib());
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_check_button_set_child")]
    fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_check_button_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_check_button_set_group")]
    fn set_group(&self, group: Option<&impl IsA<CheckButton>>) {
        unsafe {
            ffi::gtk_check_button_set_group(
                self.as_ref().to_glib_none().0,
                group.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_check_button_set_inconsistent")]
    fn set_inconsistent(&self, inconsistent: bool) {
        unsafe {
            ffi::gtk_check_button_set_inconsistent(
                self.as_ref().to_glib_none().0,
                inconsistent.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_check_button_set_label")]
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_check_button_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_check_button_set_use_underline")]
    fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_button_set_use_underline(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[cfg(feature = "v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<CheckButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "toggled")]
    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggled_trampoline<P: IsA<CheckButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P: IsA<CheckButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<CheckButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "group")]
    fn connect_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<P: IsA<CheckButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inconsistent")]
    fn connect_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inconsistent_trampoline<
            P: IsA<CheckButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCheckButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inconsistent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inconsistent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<CheckButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-underline")]
    fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<
            P: IsA<CheckButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCheckButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<CheckButton>> CheckButtonExt for O {}

impl fmt::Display for CheckButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CheckButton")
    }
}
