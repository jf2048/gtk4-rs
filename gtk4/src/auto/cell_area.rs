// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    Buildable, CellAreaContext, CellEditable, CellLayout, CellRenderer, CellRendererState,
    DirectionType, Orientation, SizeRequestMode, Snapshot, TreeIter, TreeModel, TreePath, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellArea")]
    pub struct CellArea(Object<ffi::GtkCellArea, ffi::GtkCellAreaClass>) @implements Buildable, CellLayout;

    match fn {
        type_ => || ffi::gtk_cell_area_get_type(),
    }
}

impl CellArea {
    pub const NONE: Option<&'static CellArea> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::CellArea>> Sealed for T {}
}

pub trait CellAreaExt: IsA<CellArea> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_activate")]
    fn activate(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
        edit_only: bool,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_activate(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
                edit_only.into_glib(),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_activate_cell")]
    fn activate_cell(
        &self,
        widget: &impl IsA<Widget>,
        renderer: &impl IsA<CellRenderer>,
        event: impl AsRef<gdk::Event>,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_activate_cell(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_add")]
    fn add(&self, renderer: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_cell_area_add(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_add_focus_sibling")]
    fn add_focus_sibling(
        &self,
        renderer: &impl IsA<CellRenderer>,
        sibling: &impl IsA<CellRenderer>,
    ) {
        unsafe {
            ffi::gtk_cell_area_add_focus_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_apply_attributes")]
    fn apply_attributes(
        &self,
        tree_model: &impl IsA<TreeModel>,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_apply_attributes(
                self.as_ref().to_glib_none().0,
                tree_model.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                is_expander.into_glib(),
                is_expanded.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_attribute_connect")]
    fn attribute_connect(&self, renderer: &impl IsA<CellRenderer>, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_area_attribute_connect(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_attribute_disconnect")]
    fn attribute_disconnect(&self, renderer: &impl IsA<CellRenderer>, attribute: &str) {
        unsafe {
            ffi::gtk_cell_area_attribute_disconnect(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_attribute_get_column")]
    fn attribute_get_column(&self, renderer: &impl IsA<CellRenderer>, attribute: &str) -> i32 {
        unsafe {
            ffi::gtk_cell_area_attribute_get_column(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_copy_context")]
    fn copy_context(&self, context: &impl IsA<CellAreaContext>) -> CellAreaContext {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_copy_context(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_create_context")]
    fn create_context(&self) -> CellAreaContext {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_create_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_event")]
    fn event(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        event: impl AsRef<gdk::Event>,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> i32 {
        unsafe {
            ffi::gtk_cell_area_event(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_focus")]
    fn focus(&self, direction: DirectionType) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_focus(
                self.as_ref().to_glib_none().0,
                direction.into_glib(),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_foreach")]
    fn foreach<P: FnMut(&CellRenderer) -> bool>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&CellRenderer) -> bool>(
            renderer: *mut ffi::GtkCellRenderer,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let renderer = from_glib_borrow(renderer);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&renderer).into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gtk_cell_area_foreach(
                self.as_ref().to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_cell_area_foreach_alloc")]
    fn foreach_alloc<P: FnMut(&CellRenderer, &gdk::Rectangle, &gdk::Rectangle) -> bool>(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        cell_area: &gdk::Rectangle,
        background_area: &gdk::Rectangle,
        callback: P,
    ) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<
            P: FnMut(&CellRenderer, &gdk::Rectangle, &gdk::Rectangle) -> bool,
        >(
            renderer: *mut ffi::GtkCellRenderer,
            cell_area: *const gdk::ffi::GdkRectangle,
            cell_background: *const gdk::ffi::GdkRectangle,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let renderer = from_glib_borrow(renderer);
            let cell_area = from_glib_borrow(cell_area);
            let cell_background = from_glib_borrow(cell_background);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&renderer, &cell_area, &cell_background).into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gtk_cell_area_foreach_alloc(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                background_area.to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_cell_allocation")]
    #[doc(alias = "get_cell_allocation")]
    fn cell_allocation(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        renderer: &impl IsA<CellRenderer>,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let mut allocation = gdk::Rectangle::uninitialized();
            ffi::gtk_cell_area_get_cell_allocation(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                allocation.to_glib_none_mut().0,
            );
            allocation
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_cell_at_position")]
    #[doc(alias = "get_cell_at_position")]
    fn cell_at_position(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        cell_area: &gdk::Rectangle,
        x: i32,
        y: i32,
    ) -> (CellRenderer, gdk::Rectangle) {
        unsafe {
            let mut alloc_area = gdk::Rectangle::uninitialized();
            let ret = from_glib_none(ffi::gtk_cell_area_get_cell_at_position(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                x,
                y,
                alloc_area.to_glib_none_mut().0,
            ));
            (ret, alloc_area)
        }
    }

    #[doc(alias = "gtk_cell_area_get_current_path_string")]
    #[doc(alias = "get_current_path_string")]
    fn current_path_string(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_current_path_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_edit_widget")]
    #[doc(alias = "get_edit_widget")]
    fn edit_widget(&self) -> Option<CellEditable> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edit_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_edited_cell")]
    #[doc(alias = "get_edited_cell")]
    fn edited_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edited_cell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_focus_cell")]
    #[doc(alias = "get_focus_cell")]
    fn focus_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_cell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_focus_from_sibling")]
    #[doc(alias = "get_focus_from_sibling")]
    fn focus_from_sibling(&self, renderer: &impl IsA<CellRenderer>) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_from_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_focus_siblings")]
    #[doc(alias = "get_focus_siblings")]
    fn focus_siblings(&self, renderer: &impl IsA<CellRenderer>) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_cell_area_get_focus_siblings(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_preferred_height")]
    #[doc(alias = "get_preferred_height")]
    fn preferred_height(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_height(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            (minimum_height.assume_init(), natural_height.assume_init())
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_preferred_height_for_width")]
    #[doc(alias = "get_preferred_height_for_width")]
    fn preferred_height_for_width(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        width: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_height_for_width(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            (minimum_height.assume_init(), natural_height.assume_init())
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_preferred_width")]
    #[doc(alias = "get_preferred_width")]
    fn preferred_width(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_width(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            (minimum_width.assume_init(), natural_width.assume_init())
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_get_preferred_width_for_height")]
    #[doc(alias = "get_preferred_width_for_height")]
    fn preferred_width_for_height(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_width_for_height(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                height,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            (minimum_width.assume_init(), natural_width.assume_init())
        }
    }

    #[doc(alias = "gtk_cell_area_get_request_mode")]
    #[doc(alias = "get_request_mode")]
    fn request_mode(&self) -> SizeRequestMode {
        unsafe {
            from_glib(ffi::gtk_cell_area_get_request_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_has_renderer")]
    fn has_renderer(&self, renderer: &impl IsA<CellRenderer>) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_has_renderer(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_inner_cell_area")]
    fn inner_cell_area(
        &self,
        widget: &impl IsA<Widget>,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let mut inner_area = gdk::Rectangle::uninitialized();
            ffi::gtk_cell_area_inner_cell_area(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                inner_area.to_glib_none_mut().0,
            );
            inner_area
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_is_activatable")]
    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_is_focus_sibling")]
    fn is_focus_sibling(
        &self,
        renderer: &impl IsA<CellRenderer>,
        sibling: &impl IsA<CellRenderer>,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_focus_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_remove")]
    fn remove(&self, renderer: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_cell_area_remove(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_remove_focus_sibling")]
    fn remove_focus_sibling(
        &self,
        renderer: &impl IsA<CellRenderer>,
        sibling: &impl IsA<CellRenderer>,
    ) {
        unsafe {
            ffi::gtk_cell_area_remove_focus_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_request_renderer")]
    fn request_renderer(
        &self,
        renderer: &impl IsA<CellRenderer>,
        orientation: Orientation,
        widget: &impl IsA<Widget>,
        for_size: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_request_renderer(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                orientation.into_glib(),
                widget.as_ref().to_glib_none().0,
                for_size,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_set_focus_cell")]
    fn set_focus_cell(&self, renderer: Option<&impl IsA<CellRenderer>>) {
        unsafe {
            ffi::gtk_cell_area_set_focus_cell(
                self.as_ref().to_glib_none().0,
                renderer.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_snapshot")]
    fn snapshot(
        &self,
        context: &impl IsA<CellAreaContext>,
        widget: &impl IsA<Widget>,
        snapshot: &impl IsA<Snapshot>,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
        paint_focus: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_snapshot(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                snapshot.as_ref().to_glib_none().0,
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
                paint_focus.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_stop_editing")]
    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_area_stop_editing(self.as_ref().to_glib_none().0, canceled.into_glib());
        }
    }

    #[doc(alias = "add-editable")]
    fn connect_add_editable<
        F: Fn(&Self, &CellRenderer, &CellEditable, &gdk::Rectangle, TreePath) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_editable_trampoline<
            P: IsA<CellArea>,
            F: Fn(&P, &CellRenderer, &CellEditable, &gdk::Rectangle, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            renderer: *mut ffi::GtkCellRenderer,
            editable: *mut ffi::GtkCellEditable,
            cell_area: *mut gdk::ffi::GdkRectangle,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(
                CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(renderer),
                &from_glib_borrow(editable),
                &from_glib_borrow(cell_area),
                path,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    add_editable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "apply-attributes")]
    fn connect_apply_attributes<F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn apply_attributes_trampoline<
            P: IsA<CellArea>,
            F: Fn(&P, &TreeModel, &TreeIter, bool, bool) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            is_expander: glib::ffi::gboolean,
            is_expanded: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(model),
                &from_glib_borrow(iter),
                from_glib(is_expander),
                from_glib(is_expanded),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply-attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    apply_attributes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-changed")]
    fn connect_focus_changed<F: Fn(&Self, &CellRenderer, TreePath) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn focus_changed_trampoline<
            P: IsA<CellArea>,
            F: Fn(&P, &CellRenderer, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            renderer: *mut ffi::GtkCellRenderer,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(
                CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(renderer),
                path,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    focus_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "remove-editable")]
    fn connect_remove_editable<F: Fn(&Self, &CellRenderer, &CellEditable) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn remove_editable_trampoline<
            P: IsA<CellArea>,
            F: Fn(&P, &CellRenderer, &CellEditable) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            renderer: *mut ffi::GtkCellRenderer,
            editable: *mut ffi::GtkCellEditable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(renderer),
                &from_glib_borrow(editable),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    remove_editable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "edit-widget")]
    fn connect_edit_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_edit_widget_trampoline<
            P: IsA<CellArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::edit-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_edit_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "edited-cell")]
    fn connect_edited_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_edited_cell_trampoline<
            P: IsA<CellArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::edited-cell\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_edited_cell_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-cell")]
    fn connect_focus_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_cell_trampoline<P: IsA<CellArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focus-cell\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focus_cell_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<CellArea>> CellAreaExt for O {}

impl fmt::Display for CellArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellArea")
    }
}
