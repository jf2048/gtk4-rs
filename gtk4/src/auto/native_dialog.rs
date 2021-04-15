// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ResponseType;
use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct NativeDialog(Object<ffi::GtkNativeDialog, ffi::GtkNativeDialogClass>);

    match fn {
        get_type => || ffi::gtk_native_dialog_get_type(),
    }
}

pub const NONE_NATIVE_DIALOG: Option<&NativeDialog> = None;

pub trait NativeDialogExt: 'static {
    #[doc(alias = "gtk_native_dialog_destroy")]
    fn destroy(&self);

    #[doc(alias = "gtk_native_dialog_get_modal")]
    fn is_modal(&self) -> bool;

    #[doc(alias = "gtk_native_dialog_get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_native_dialog_get_transient_for")]
    fn transient_for(&self) -> Option<Window>;

    #[doc(alias = "gtk_native_dialog_get_visible")]
    fn is_visible(&self) -> bool;

    #[doc(alias = "gtk_native_dialog_hide")]
    fn hide(&self);

    #[doc(alias = "gtk_native_dialog_set_modal")]
    fn set_modal(&self, modal: bool);

    #[doc(alias = "gtk_native_dialog_set_title")]
    fn set_title(&self, title: &str);

    #[doc(alias = "gtk_native_dialog_set_transient_for")]
    fn set_transient_for<P: IsA<Window>>(&self, parent: Option<&P>);

    #[doc(alias = "gtk_native_dialog_show")]
    fn show(&self);

    #[doc(alias = "set_property_visible")]
    fn set_visible(&self, visible: bool);

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NativeDialog>> NativeDialogExt for O {
    fn destroy(&self) {
        unsafe {
            ffi::gtk_native_dialog_destroy(self.as_ref().to_glib_none().0);
        }
    }

    fn is_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_native_dialog_get_modal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_native_dialog_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn transient_for(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_native_dialog_get_transient_for(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_native_dialog_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hide(&self) {
        unsafe {
            ffi::gtk_native_dialog_hide(self.as_ref().to_glib_none().0);
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_native_dialog_set_modal(self.as_ref().to_glib_none().0, modal.to_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_native_dialog_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn set_transient_for<P: IsA<Window>>(&self, parent: Option<&P>) {
        unsafe {
            ffi::gtk_native_dialog_set_transient_for(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn show(&self) {
        unsafe {
            ffi::gtk_native_dialog_show(self.as_ref().to_glib_none().0);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"visible\0".as_ptr() as *const _,
                glib::Value::from(&visible).to_glib_none().0,
            );
        }
    }

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<P, F: Fn(&P, ResponseType) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            response_id: ffi::GtkResponseType,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(
                &NativeDialog::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(response_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transient_for_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transient-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transient_for_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkNativeDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NativeDialog>,
        {
            let f: &F = &*(f as *const F);
            f(&NativeDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NativeDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NativeDialog")
    }
}