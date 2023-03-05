// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    AxisUse, Device, DeviceTool, Display, EventSequence, EventType, ModifierType, Seat, Surface,
    TimeCoord,
};
use glib::translate::*;
use std::{fmt, mem, ptr};

glib::wrapper! {
    #[doc(alias = "GdkEvent")]
    pub struct Event(Shared<ffi::GdkEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr),
        unref => |ptr| ffi::gdk_event_unref(ptr),
    }
}

impl glib::StaticType for Event {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_event_get_type()) }
    }
}

impl Event {
    pub const NONE: Option<&'static Event> = None;

    #[doc(alias = "gdk_event_get_axes")]
    #[doc(alias = "get_axes")]
    pub fn axes(&self) -> Option<Vec<f64>> {
        unsafe {
            let mut axes = ptr::null_mut();
            let mut n_axes = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_event_get_axes(
                self.as_ref().to_glib_none().0,
                &mut axes,
                n_axes.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_none_num(
                    axes,
                    n_axes.assume_init() as _,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_event_get_axis")]
    #[doc(alias = "get_axis")]
    pub fn axis(&self, axis_use: AxisUse) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_event_get_axis(
                self.as_ref().to_glib_none().0,
                axis_use.into_glib(),
                value.as_mut_ptr(),
            ));
            if ret {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_event_get_device")]
    #[doc(alias = "get_device")]
    pub fn device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_event_get_device(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_device_tool")]
    #[doc(alias = "get_device_tool")]
    pub fn device_tool(&self) -> Option<DeviceTool> {
        unsafe {
            from_glib_none(ffi::gdk_event_get_device_tool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_event_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_event_get_display(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_event_sequence")]
    #[doc(alias = "get_event_sequence")]
    pub fn event_sequence(&self) -> EventSequence {
        unsafe {
            from_glib_none(ffi::gdk_event_get_event_sequence(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_event_get_event_type")]
    #[doc(alias = "get_event_type")]
    pub fn event_type(&self) -> EventType {
        unsafe {
            from_glib(ffi::gdk_event_get_event_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_event_get_history")]
    #[doc(alias = "get_history")]
    pub fn history(&self) -> glib::Slice<TimeCoord> {
        unsafe {
            let mut out_n_coords = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(
                ffi::gdk_event_get_history(
                    self.as_ref().to_glib_none().0,
                    out_n_coords.as_mut_ptr(),
                ),
                out_n_coords.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gdk_event_get_modifier_state")]
    #[doc(alias = "get_modifier_state")]
    pub fn modifier_state(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::gdk_event_get_modifier_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_event_get_pointer_emulated")]
    #[doc(alias = "get_pointer_emulated")]
    pub fn is_pointer_emulated(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_event_get_pointer_emulated(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_event_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_event_get_position(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            if ret {
                Some((x.assume_init(), y.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_event_get_seat")]
    #[doc(alias = "get_seat")]
    pub fn seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_event_get_seat(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_event_get_surface(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_time")]
    #[doc(alias = "get_time")]
    pub fn time(&self) -> u32 {
        unsafe { ffi::gdk_event_get_time(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_event_triggers_context_menu")]
    pub fn triggers_context_menu(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_event_triggers_context_menu(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Event")
    }
}
