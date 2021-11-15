// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::FlowBoxChild;
use crate::LayoutManager;
use crate::MovementStep;
use crate::Orientable;
use crate::Orientation;
use crate::Overflow;
use crate::SelectionMode;
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
    #[doc(alias = "GtkFlowBox")]
    pub struct FlowBox(Object<ffi::GtkFlowBox>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_flow_box_get_type(),
    }
}

impl FlowBox {
    #[doc(alias = "gtk_flow_box_new")]
    pub fn new() -> FlowBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_flow_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FlowBox`] objects.
    ///
    /// This method returns an instance of [`FlowBoxBuilder`](crate::builders::FlowBoxBuilder) which can be used to create [`FlowBox`] objects.
    pub fn builder() -> FlowBoxBuilder {
        FlowBoxBuilder::default()
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_flow_box_append")]
    pub fn append(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_flow_box_append(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_bind_model")]
    pub fn bind_model<P: Fn(&glib::Object) -> Widget + 'static>(
        &self,
        model: Option<&impl IsA<gio::ListModel>>,
        create_widget_func: P,
    ) {
        let create_widget_func_data: Box_<P> = Box_::new(create_widget_func);
        unsafe extern "C" fn create_widget_func_func<P: Fn(&glib::Object) -> Widget + 'static>(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GtkWidget {
            let item = from_glib_borrow(item);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let create_widget_func = Some(create_widget_func_func::<P> as _);
        unsafe extern "C" fn user_data_free_func_func<P: Fn(&glib::Object) -> Widget + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(user_data_free_func_func::<P> as _);
        let super_callback0: Box_<P> = create_widget_func_data;
        unsafe {
            ffi::gtk_flow_box_bind_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                create_widget_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_get_activate_on_single_click")]
    #[doc(alias = "get_activate_on_single_click")]
    pub fn activates_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_activate_on_single_click(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_child_at_index")]
    #[doc(alias = "get_child_at_index")]
    pub fn child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_index(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_child_at_pos")]
    #[doc(alias = "get_child_at_pos")]
    pub fn child_at_pos(&self, x: i32, y: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_pos(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_column_spacing")]
    #[doc(alias = "get_column_spacing")]
    pub fn column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_column_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_flow_box_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_flow_box_get_max_children_per_line")]
    #[doc(alias = "get_max_children_per_line")]
    pub fn max_children_per_line(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_max_children_per_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_min_children_per_line")]
    #[doc(alias = "get_min_children_per_line")]
    pub fn min_children_per_line(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_min_children_per_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_row_spacing")]
    #[doc(alias = "get_row_spacing")]
    pub fn row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_flow_box_get_row_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_flow_box_get_selected_children")]
    #[doc(alias = "get_selected_children")]
    pub fn selected_children(&self) -> Vec<FlowBoxChild> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_flow_box_get_selected_children(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_flow_box_get_selection_mode")]
    #[doc(alias = "get_selection_mode")]
    pub fn selection_mode(&self) -> SelectionMode {
        unsafe { from_glib(ffi::gtk_flow_box_get_selection_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_flow_box_insert")]
    pub fn insert(&self, widget: &impl IsA<Widget>, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_invalidate_filter")]
    pub fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_invalidate_sort")]
    pub fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_flow_box_prepend")]
    pub fn prepend(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_flow_box_prepend(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_remove")]
    pub fn remove(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_flow_box_remove(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_select_all")]
    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_flow_box_select_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_select_child")]
    pub fn select_child(&self, child: &impl IsA<FlowBoxChild>) {
        unsafe {
            ffi::gtk_flow_box_select_child(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_selected_foreach")]
    pub fn selected_foreach<P: FnMut(&FlowBox, &FlowBoxChild)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&FlowBox, &FlowBoxChild)>(
            box_: *mut ffi::GtkFlowBox,
            child: *mut ffi::GtkFlowBoxChild,
            user_data: glib::ffi::gpointer,
        ) {
            let box_ = from_glib_borrow(box_);
            let child = from_glib_borrow(child);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&box_, &child);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_flow_box_selected_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_activate_on_single_click")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(
                self.to_glib_none().0,
                single.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_column_spacing")]
    pub fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_column_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_flow_box_set_filter_func")]
    pub fn set_filter_func<P: Fn(&FlowBoxChild) -> bool + 'static>(&self, filter_func: P) {
        let filter_func_data: Box_<P> = Box_::new(filter_func);
        unsafe extern "C" fn filter_func_func<P: Fn(&FlowBoxChild) -> bool + 'static>(
            child: *mut ffi::GtkFlowBoxChild,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let child = from_glib_borrow(child);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&child);
            res.into_glib()
        }
        let filter_func = Some(filter_func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&FlowBoxChild) -> bool + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = filter_func_data;
        unsafe {
            ffi::gtk_flow_box_set_filter_func(
                self.to_glib_none().0,
                filter_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_hadjustment")]
    pub fn set_hadjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(
                self.to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_set_homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_flow_box_set_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_flow_box_set_max_children_per_line")]
    pub fn set_max_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_max_children_per_line(self.to_glib_none().0, n_children);
        }
    }

    #[doc(alias = "gtk_flow_box_set_min_children_per_line")]
    pub fn set_min_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_min_children_per_line(self.to_glib_none().0, n_children);
        }
    }

    #[doc(alias = "gtk_flow_box_set_row_spacing")]
    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_row_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_flow_box_set_selection_mode")]
    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_flow_box_set_selection_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gtk_flow_box_set_vadjustment")]
    pub fn set_vadjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(
                self.to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_flow_box_unselect_all")]
    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_flow_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_flow_box_unselect_child")]
    pub fn unselect_child(&self, child: &impl IsA<FlowBoxChild>) {
        unsafe {
            ffi::gtk_flow_box_unselect_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn accepts_unpaired_release(&self) -> bool {
        glib::ObjectExt::property(self, "accept-unpaired-release")
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn set_accept_unpaired_release(&self, accept_unpaired_release: bool) {
        glib::ObjectExt::set_property(self, "accept-unpaired-release", &accept_unpaired_release)
    }

    #[doc(alias = "activate-cursor-child")]
    pub fn connect_activate_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_cursor_child_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-cursor-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_cursor_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate_cursor_child(&self) {
        let _ = self.emit_by_name("activate-cursor-child", &[]);
    }

    #[doc(alias = "child-activated")]
    pub fn connect_child_activated<F: Fn(&Self, &FlowBoxChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_activated_trampoline<
            F: Fn(&FlowBox, &FlowBoxChild) + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
            child: *mut ffi::GtkFlowBoxChild,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "move-cursor")]
    pub fn connect_move_cursor<F: Fn(&Self, MovementStep, i32, bool, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<
            F: Fn(&FlowBox, MovementStep, i32, bool, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
            step: ffi::GtkMovementStep,
            count: libc::c_int,
            extend: glib::ffi::gboolean,
            modify: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(step),
                count,
                from_glib(extend),
                from_glib(modify),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_cursor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_cursor(
        &self,
        step: MovementStep,
        count: i32,
        extend: bool,
        modify: bool,
    ) -> bool {
        let res = self.emit_by_name("move-cursor", &[&step, &count, &extend, &modify]);
        res.unwrap()
            .get()
            .expect("Return Value for `emit_move_cursor`")
    }

    #[doc(alias = "select-all")]
    pub fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn select_all_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    select_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_select_all(&self) {
        let _ = self.emit_by_name("select-all", &[]);
    }

    #[doc(alias = "selected-children-changed")]
    pub fn connect_selected_children_changed<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn selected_children_changed_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selected-children-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selected_children_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toggle-cursor-child")]
    pub fn connect_toggle_cursor_child<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_cursor_child_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-cursor-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_cursor_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_toggle_cursor_child(&self) {
        let _ = self.emit_by_name("toggle-cursor-child", &[]);
    }

    #[doc(alias = "unselect-all")]
    pub fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unselect_all_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unselect-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unselect_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_unselect_all(&self) {
        let _ = self.emit_by_name("unselect-all", &[]);
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn connect_accept_unpaired_release_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_unpaired_release_trampoline<
            F: Fn(&FlowBox) + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::accept-unpaired-release\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_unpaired_release_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "activate-on-single-click")]
    pub fn connect_activate_on_single_click_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activate_on_single_click_trampoline<
            F: Fn(&FlowBox) + 'static,
        >(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::activate-on-single-click\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activate_on_single_click_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-spacing")]
    pub fn connect_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "homogeneous")]
    pub fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-children-per-line")]
    pub fn connect_max_children_per_line_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_children_per_line_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::max-children-per-line\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_children_per_line_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-children-per-line")]
    pub fn connect_min_children_per_line_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_children_per_line_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::min-children-per-line\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_children_per_line_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-spacing")]
    pub fn connect_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selection-mode")]
    pub fn connect_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selection_mode_trampoline<F: Fn(&FlowBox) + 'static>(
            this: *mut ffi::GtkFlowBox,
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
                b"notify::selection-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selection_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FlowBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FlowBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct FlowBoxBuilder {
    accept_unpaired_release: Option<bool>,
    activate_on_single_click: Option<bool>,
    column_spacing: Option<u32>,
    homogeneous: Option<bool>,
    max_children_per_line: Option<u32>,
    min_children_per_line: Option<u32>,
    row_spacing: Option<u32>,
    selection_mode: Option<SelectionMode>,
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

impl FlowBoxBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FlowBoxBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FlowBox`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> FlowBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accept_unpaired_release) = self.accept_unpaired_release {
            properties.push(("accept-unpaired-release", accept_unpaired_release));
        }
        if let Some(ref activate_on_single_click) = self.activate_on_single_click {
            properties.push(("activate-on-single-click", activate_on_single_click));
        }
        if let Some(ref column_spacing) = self.column_spacing {
            properties.push(("column-spacing", column_spacing));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref max_children_per_line) = self.max_children_per_line {
            properties.push(("max-children-per-line", max_children_per_line));
        }
        if let Some(ref min_children_per_line) = self.min_children_per_line {
            properties.push(("min-children-per-line", min_children_per_line));
        }
        if let Some(ref row_spacing) = self.row_spacing {
            properties.push(("row-spacing", row_spacing));
        }
        if let Some(ref selection_mode) = self.selection_mode {
            properties.push(("selection-mode", selection_mode));
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
        glib::Object::new::<FlowBox>(&properties).expect("Failed to create an instance of FlowBox")
    }

    pub fn accept_unpaired_release(mut self, accept_unpaired_release: bool) -> Self {
        self.accept_unpaired_release = Some(accept_unpaired_release);
        self
    }

    pub fn activate_on_single_click(mut self, activate_on_single_click: bool) -> Self {
        self.activate_on_single_click = Some(activate_on_single_click);
        self
    }

    pub fn column_spacing(mut self, column_spacing: u32) -> Self {
        self.column_spacing = Some(column_spacing);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn max_children_per_line(mut self, max_children_per_line: u32) -> Self {
        self.max_children_per_line = Some(max_children_per_line);
        self
    }

    pub fn min_children_per_line(mut self, min_children_per_line: u32) -> Self {
        self.min_children_per_line = Some(min_children_per_line);
        self
    }

    pub fn row_spacing(mut self, row_spacing: u32) -> Self {
        self.row_spacing = Some(row_spacing);
        self
    }

    pub fn selection_mode(mut self, selection_mode: SelectionMode) -> Self {
        self.selection_mode = Some(selection_mode);
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

impl fmt::Display for FlowBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FlowBox")
    }
}
