// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::AppChooser;
use crate::Application;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Dialog;
use crate::DialogFlags;
use crate::LayoutManager;
use crate::Native;
use crate::Overflow;
use crate::Root;
use crate::ShortcutManager;
use crate::Widget;
use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
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
    #[doc(alias = "GtkAppChooserDialog")]
    pub struct AppChooserDialog(Object<ffi::GtkAppChooserDialog>) @extends Dialog, Window, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager, AppChooser;

    match fn {
        type_ => || ffi::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    #[doc(alias = "gtk_app_chooser_dialog_new")]
    pub fn new(
        parent: Option<&impl IsA<Window>>,
        flags: DialogFlags,
        file: &impl IsA<gio::File>,
    ) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                file.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_app_chooser_dialog_new_for_content_type")]
    #[doc(alias = "new_for_content_type")]
    pub fn for_content_type(
        parent: Option<&impl IsA<Window>>,
        flags: DialogFlags,
        content_type: &str,
    ) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new_for_content_type(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                content_type.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppChooserDialog`] objects.
    ///
    /// This method returns an instance of [`AppChooserDialogBuilder`](crate::builders::AppChooserDialogBuilder) which can be used to create [`AppChooserDialog`] objects.
    pub fn builder() -> AppChooserDialogBuilder {
        AppChooserDialogBuilder::default()
    }

    #[doc(alias = "gtk_app_chooser_dialog_get_heading")]
    #[doc(alias = "get_heading")]
    pub fn heading(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_heading(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_dialog_get_widget")]
    #[doc(alias = "get_widget")]
    pub fn widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_dialog_set_heading")]
    pub fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(
                self.to_glib_none().0,
                heading.to_glib_none().0,
            );
        }
    }

    pub fn gfile(&self) -> Option<gio::File> {
        glib::ObjectExt::property(self, "gfile")
    }

    #[doc(alias = "heading")]
    pub fn connect_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_trampoline<F: Fn(&AppChooserDialog) + 'static>(
            this: *mut ffi::GtkAppChooserDialog,
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
                b"notify::heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_heading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for AppChooserDialog {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct AppChooserDialog object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppChooserDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppChooserDialogBuilder {
    gfile: Option<gio::File>,
    heading: Option<String>,
    use_header_bar: Option<i32>,
    application: Option<Application>,
    child: Option<Widget>,
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_widget: Option<Widget>,
    default_width: Option<i32>,
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    display: Option<gdk::Display>,
    focus_visible: Option<bool>,
    focus_widget: Option<Widget>,
    fullscreened: Option<bool>,
    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    handle_menubar_accel: Option<bool>,
    hide_on_close: Option<bool>,
    icon_name: Option<String>,
    maximized: Option<bool>,
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    startup_id: Option<String>,
    title: Option<String>,
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    titlebar: Option<Widget>,
    transient_for: Option<Window>,
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
    content_type: Option<String>,
}

impl AppChooserDialogBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AppChooserDialogBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AppChooserDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppChooserDialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref gfile) = self.gfile {
            properties.push(("gfile", gfile));
        }
        if let Some(ref heading) = self.heading {
            properties.push(("heading", heading));
        }
        if let Some(ref use_header_bar) = self.use_header_bar {
            properties.push(("use-header-bar", use_header_bar));
        }
        if let Some(ref application) = self.application {
            properties.push(("application", application));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref decorated) = self.decorated {
            properties.push(("decorated", decorated));
        }
        if let Some(ref default_height) = self.default_height {
            properties.push(("default-height", default_height));
        }
        if let Some(ref default_widget) = self.default_widget {
            properties.push(("default-widget", default_widget));
        }
        if let Some(ref default_width) = self.default_width {
            properties.push(("default-width", default_width));
        }
        if let Some(ref deletable) = self.deletable {
            properties.push(("deletable", deletable));
        }
        if let Some(ref destroy_with_parent) = self.destroy_with_parent {
            properties.push(("destroy-with-parent", destroy_with_parent));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref focus_visible) = self.focus_visible {
            properties.push(("focus-visible", focus_visible));
        }
        if let Some(ref focus_widget) = self.focus_widget {
            properties.push(("focus-widget", focus_widget));
        }
        if let Some(ref fullscreened) = self.fullscreened {
            properties.push(("fullscreened", fullscreened));
        }
        #[cfg(any(feature = "v4_2", feature = "dox"))]
        if let Some(ref handle_menubar_accel) = self.handle_menubar_accel {
            properties.push(("handle-menubar-accel", handle_menubar_accel));
        }
        if let Some(ref hide_on_close) = self.hide_on_close {
            properties.push(("hide-on-close", hide_on_close));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref maximized) = self.maximized {
            properties.push(("maximized", maximized));
        }
        if let Some(ref mnemonics_visible) = self.mnemonics_visible {
            properties.push(("mnemonics-visible", mnemonics_visible));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref startup_id) = self.startup_id {
            properties.push(("startup-id", startup_id));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        #[cfg(any(feature = "v4_6", feature = "dox"))]
        if let Some(ref titlebar) = self.titlebar {
            properties.push(("titlebar", titlebar));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
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
        if let Some(ref content_type) = self.content_type {
            properties.push(("content-type", content_type));
        }
        glib::Object::new::<AppChooserDialog>(&properties)
            .expect("Failed to create an instance of AppChooserDialog")
    }

    pub fn gfile(mut self, gfile: &impl IsA<gio::File>) -> Self {
        self.gfile = Some(gfile.clone().upcast());
        self
    }

    pub fn heading(mut self, heading: &str) -> Self {
        self.heading = Some(heading.to_string());
        self
    }

    pub fn use_header_bar(mut self, use_header_bar: i32) -> Self {
        self.use_header_bar = Some(use_header_bar);
        self
    }

    pub fn application(mut self, application: &impl IsA<Application>) -> Self {
        self.application = Some(application.clone().upcast());
        self
    }

    pub fn child(mut self, child: &impl IsA<Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_widget(mut self, default_widget: &impl IsA<Widget>) -> Self {
        self.default_widget = Some(default_widget.clone().upcast());
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    pub fn display(mut self, display: &impl IsA<gdk::Display>) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    pub fn focus_widget(mut self, focus_widget: &impl IsA<Widget>) -> Self {
        self.focus_widget = Some(focus_widget.clone().upcast());
        self
    }

    pub fn fullscreened(mut self, fullscreened: bool) -> Self {
        self.fullscreened = Some(fullscreened);
        self
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(mut self, handle_menubar_accel: bool) -> Self {
        self.handle_menubar_accel = Some(handle_menubar_accel);
        self
    }

    pub fn hide_on_close(mut self, hide_on_close: bool) -> Self {
        self.hide_on_close = Some(hide_on_close);
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = Some(maximized);
        self
    }

    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn titlebar(mut self, titlebar: &impl IsA<Widget>) -> Self {
        self.titlebar = Some(titlebar.clone().upcast());
        self
    }

    pub fn transient_for(mut self, transient_for: &impl IsA<Window>) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
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

    pub fn content_type(mut self, content_type: &str) -> Self {
        self.content_type = Some(content_type.to_string());
        self
    }
}

impl fmt::Display for AppChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooserDialog")
    }
}
