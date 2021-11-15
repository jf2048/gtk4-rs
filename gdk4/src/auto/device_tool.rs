// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::AxisFlags;
use crate::DeviceToolType;
use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkDeviceTool")]
    pub struct DeviceTool(Object<ffi::GdkDeviceTool>);

    match fn {
        type_ => || ffi::gdk_device_tool_get_type(),
    }
}

impl DeviceTool {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DeviceTool`] objects.
    ///
    /// This method returns an instance of [`DeviceToolBuilder`](crate::builders::DeviceToolBuilder) which can be used to create [`DeviceTool`] objects.
    pub fn builder() -> DeviceToolBuilder {
        DeviceToolBuilder::default()
    }

    #[doc(alias = "gdk_device_tool_get_axes")]
    #[doc(alias = "get_axes")]
    pub fn axes(&self) -> AxisFlags {
        unsafe { from_glib(ffi::gdk_device_tool_get_axes(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_tool_get_hardware_id")]
    #[doc(alias = "get_hardware_id")]
    pub fn hardware_id(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_hardware_id(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_tool_get_serial")]
    #[doc(alias = "get_serial")]
    pub fn serial(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_tool_get_tool_type")]
    #[doc(alias = "get_tool_type")]
    pub fn tool_type(&self) -> DeviceToolType {
        unsafe { from_glib(ffi::gdk_device_tool_get_tool_type(self.to_glib_none().0)) }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DeviceTool`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct DeviceToolBuilder {
    axes: Option<AxisFlags>,
    hardware_id: Option<u64>,
    serial: Option<u64>,
    tool_type: Option<DeviceToolType>,
}

impl DeviceToolBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`DeviceToolBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DeviceTool`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> DeviceTool {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref axes) = self.axes {
            properties.push(("axes", axes));
        }
        if let Some(ref hardware_id) = self.hardware_id {
            properties.push(("hardware-id", hardware_id));
        }
        if let Some(ref serial) = self.serial {
            properties.push(("serial", serial));
        }
        if let Some(ref tool_type) = self.tool_type {
            properties.push(("tool-type", tool_type));
        }
        glib::Object::new::<DeviceTool>(&properties)
            .expect("Failed to create an instance of DeviceTool")
    }

    pub fn axes(mut self, axes: AxisFlags) -> Self {
        self.axes = Some(axes);
        self
    }

    pub fn hardware_id(mut self, hardware_id: u64) -> Self {
        self.hardware_id = Some(hardware_id);
        self
    }

    pub fn serial(mut self, serial: u64) -> Self {
        self.serial = Some(serial);
        self
    }

    pub fn tool_type(mut self, tool_type: DeviceToolType) -> Self {
        self.tool_type = Some(tool_type);
        self
    }
}

impl fmt::Display for DeviceTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceTool")
    }
}
