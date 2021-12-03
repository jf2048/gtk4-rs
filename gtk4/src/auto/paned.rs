// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Orientable;
use crate::Orientation;
use crate::Overflow;
use crate::ScrollType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    #[doc(alias = "GtkPaned")]
    pub struct Paned(Object<ffi::GtkPaned>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_paned_get_type(),
    }
}

impl Paned {
    #[doc(alias = "gtk_paned_new")]
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_paned_new(orientation.into_glib())).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Paned`] objects.
    ///
    /// This method returns an instance of [`PanedBuilder`](crate::builders::PanedBuilder) which can be used to create [`Paned`] objects.
    pub fn builder() -> PanedBuilder {
        PanedBuilder::default()
    }

    #[doc(alias = "gtk_paned_get_end_child")]
    #[doc(alias = "get_end_child")]
    pub fn end_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_end_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> i32 {
        unsafe { ffi::gtk_paned_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_paned_get_resize_end_child")]
    #[doc(alias = "get_resize_end_child")]
    pub fn resizes_end_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_resize_end_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_resize_start_child")]
    #[doc(alias = "get_resize_start_child")]
    pub fn resizes_start_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_resize_start_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_shrink_end_child")]
    #[doc(alias = "get_shrink_end_child")]
    pub fn shrinks_end_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_shrink_end_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_shrink_start_child")]
    #[doc(alias = "get_shrink_start_child")]
    pub fn shrinks_start_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_shrink_start_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_start_child")]
    #[doc(alias = "get_start_child")]
    pub fn start_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_start_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_wide_handle")]
    #[doc(alias = "get_wide_handle")]
    pub fn is_wide_handle(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_wide_handle(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_set_end_child")]
    pub fn set_end_child(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_paned_set_end_child(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_paned_set_position")]
    pub fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_paned_set_resize_end_child")]
    pub fn set_resize_end_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_resize_end_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_resize_start_child")]
    pub fn set_resize_start_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_resize_start_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_shrink_end_child")]
    pub fn set_shrink_end_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_shrink_end_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_shrink_start_child")]
    pub fn set_shrink_start_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_shrink_start_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_start_child")]
    pub fn set_start_child(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_paned_set_start_child(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_paned_set_wide_handle")]
    pub fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.to_glib_none().0, wide.into_glib());
        }
    }

    #[doc(alias = "max-position")]
    pub fn max_position(&self) -> i32 {
        glib::ObjectExt::property(self, "max-position")
    }

    #[doc(alias = "min-position")]
    pub fn min_position(&self) -> i32 {
        glib::ObjectExt::property(self, "min-position")
    }

    #[doc(alias = "position-set")]
    pub fn is_position_set(&self) -> bool {
        glib::ObjectExt::property(self, "position-set")
    }

    #[doc(alias = "accept-position")]
    pub fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accept_position_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_accept_position(&self) -> bool {
        self.emit_by_name("accept-position", &[])
    }

    #[doc(alias = "cancel-position")]
    pub fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_position_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cancel_position(&self) -> bool {
        self.emit_by_name("cancel-position", &[])
    }

    #[doc(alias = "cycle-child-focus")]
    pub fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_child_focus_trampoline<F: Fn(&Paned, bool) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reversed)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-child-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_child_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        self.emit_by_name("cycle-child-focus", &[&reversed])
    }

    #[doc(alias = "cycle-handle-focus")]
    pub fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_handle_focus_trampoline<
            F: Fn(&Paned, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reversed)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_handle_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        self.emit_by_name("cycle-handle-focus", &[&reversed])
    }

    #[doc(alias = "move-handle")]
    pub fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_handle_trampoline<F: Fn(&Paned, ScrollType) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            scroll_type: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(scroll_type)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        self.emit_by_name("move-handle", &[&scroll_type])
    }

    #[doc(alias = "toggle-handle-focus")]
    pub fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn toggle_handle_focus_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_handle_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_toggle_handle_focus(&self) -> bool {
        self.emit_by_name("toggle-handle-focus", &[])
    }

    #[doc(alias = "end-child")]
    pub fn connect_end_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::end-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-position")]
    pub fn connect_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::max-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-position")]
    pub fn connect_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::min-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position-set")]
    pub fn connect_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_set_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::position-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize-end-child")]
    pub fn connect_resize_end_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_end_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::resize-end-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resize_end_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize-start-child")]
    pub fn connect_resize_start_child_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_start_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::resize-start-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resize_start_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shrink-end-child")]
    pub fn connect_shrink_end_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_end_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::shrink-end-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shrink_end_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shrink-start-child")]
    pub fn connect_shrink_start_child_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_start_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::shrink-start-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shrink_start_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "start-child")]
    pub fn connect_start_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::start-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wide-handle")]
    pub fn connect_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wide_handle_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                b"notify::wide-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wide_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Paned {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct Paned object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Paned`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PanedBuilder {
    end_child: Option<Widget>,
    position: Option<i32>,
    position_set: Option<bool>,
    resize_end_child: Option<bool>,
    resize_start_child: Option<bool>,
    shrink_end_child: Option<bool>,
    shrink_start_child: Option<bool>,
    start_child: Option<Widget>,
    wide_handle: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    orientation: Option<Orientation>,
}

impl PanedBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PanedBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Paned`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Paned {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref end_child) = self.end_child {
            properties.push(("end-child", end_child));
        }
        if let Some(ref position) = self.position {
            properties.push(("position", position));
        }
        if let Some(ref position_set) = self.position_set {
            properties.push(("position-set", position_set));
        }
        if let Some(ref resize_end_child) = self.resize_end_child {
            properties.push(("resize-end-child", resize_end_child));
        }
        if let Some(ref resize_start_child) = self.resize_start_child {
            properties.push(("resize-start-child", resize_start_child));
        }
        if let Some(ref shrink_end_child) = self.shrink_end_child {
            properties.push(("shrink-end-child", shrink_end_child));
        }
        if let Some(ref shrink_start_child) = self.shrink_start_child {
            properties.push(("shrink-start-child", shrink_start_child));
        }
        if let Some(ref start_child) = self.start_child {
            properties.push(("start-child", start_child));
        }
        if let Some(ref wide_handle) = self.wide_handle {
            properties.push(("wide-handle", wide_handle));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<Paned>(&properties).expect("Failed to create an instance of Paned")
    }

    pub fn end_child(mut self, end_child: &impl IsA<Widget>) -> Self {
        self.end_child = Some(end_child.clone().upcast());
        self
    }

    pub fn position(mut self, position: i32) -> Self {
        self.position = Some(position);
        self
    }

    pub fn position_set(mut self, position_set: bool) -> Self {
        self.position_set = Some(position_set);
        self
    }

    pub fn resize_end_child(mut self, resize_end_child: bool) -> Self {
        self.resize_end_child = Some(resize_end_child);
        self
    }

    pub fn resize_start_child(mut self, resize_start_child: bool) -> Self {
        self.resize_start_child = Some(resize_start_child);
        self
    }

    pub fn shrink_end_child(mut self, shrink_end_child: bool) -> Self {
        self.shrink_end_child = Some(shrink_end_child);
        self
    }

    pub fn shrink_start_child(mut self, shrink_start_child: bool) -> Self {
        self.shrink_start_child = Some(shrink_start_child);
        self
    }

    pub fn start_child(mut self, start_child: &impl IsA<Widget>) -> Self {
        self.start_child = Some(start_child.clone().upcast());
        self
    }

    pub fn wide_handle(mut self, wide_handle: bool) -> Self {
        self.wide_handle = Some(wide_handle);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for Paned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Paned")
    }
}
