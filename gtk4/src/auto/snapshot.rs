// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::StyleContext;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkSnapshot")]
    pub struct Snapshot(Object<ffi::GtkSnapshot, ffi::GtkSnapshotClass>) @extends gdk::Snapshot;

    match fn {
        type_ => || ffi::gtk_snapshot_get_type(),
    }
}

impl Snapshot {
    pub const NONE: Option<&'static Snapshot> = None;

    #[doc(alias = "gtk_snapshot_new")]
    pub fn new() -> Snapshot {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_snapshot_new()) }
    }
}

impl Default for Snapshot {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SnapshotExt: IsA<Snapshot> + 'static {
    #[doc(alias = "gtk_snapshot_append_cairo")]
    fn append_cairo(&self, bounds: &graphene::Rect) -> cairo::Context {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_append_cairo(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_append_color")]
    fn append_color(&self, color: &gdk::RGBA, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_append_color(
                self.as_ref().to_glib_none().0,
                color.to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_conic_gradient")]
    fn append_conic_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        rotation: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as _;
        unsafe {
            ffi::gtk_snapshot_append_conic_gradient(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                rotation,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_inset_shadow")]
    fn append_inset_shadow(
        &self,
        outline: &gsk::RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_inset_shadow(
                self.as_ref().to_glib_none().0,
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_layout")]
    fn append_layout(&self, layout: &pango::Layout, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_snapshot_append_layout(
                self.as_ref().to_glib_none().0,
                layout.to_glib_none().0,
                color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_linear_gradient")]
    fn append_linear_gradient(
        &self,
        bounds: &graphene::Rect,
        start_point: &graphene::Point,
        end_point: &graphene::Point,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as _;
        unsafe {
            ffi::gtk_snapshot_append_linear_gradient(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_node")]
    fn append_node(&self, node: impl AsRef<gsk::RenderNode>) {
        unsafe {
            ffi::gtk_snapshot_append_node(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_outset_shadow")]
    fn append_outset_shadow(
        &self,
        outline: &gsk::RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_outset_shadow(
                self.as_ref().to_glib_none().0,
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_radial_gradient")]
    fn append_radial_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as _;
        unsafe {
            ffi::gtk_snapshot_append_radial_gradient(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_repeating_linear_gradient")]
    fn append_repeating_linear_gradient(
        &self,
        bounds: &graphene::Rect,
        start_point: &graphene::Point,
        end_point: &graphene::Point,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as _;
        unsafe {
            ffi::gtk_snapshot_append_repeating_linear_gradient(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_repeating_radial_gradient")]
    fn append_repeating_radial_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as _;
        unsafe {
            ffi::gtk_snapshot_append_repeating_radial_gradient(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_snapshot_append_scaled_texture")]
    fn append_scaled_texture(
        &self,
        texture: &impl IsA<gdk::Texture>,
        filter: gsk::ScalingFilter,
        bounds: &graphene::Rect,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_scaled_texture(
                self.as_ref().to_glib_none().0,
                texture.as_ref().to_glib_none().0,
                filter.into_glib(),
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_texture")]
    fn append_texture(&self, texture: &impl IsA<gdk::Texture>, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_append_texture(
                self.as_ref().to_glib_none().0,
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_gl_shader_pop_texture")]
    fn gl_shader_pop_texture(&self) {
        unsafe {
            ffi::gtk_snapshot_gl_shader_pop_texture(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_perspective")]
    fn perspective(&self, depth: f32) {
        unsafe {
            ffi::gtk_snapshot_perspective(self.as_ref().to_glib_none().0, depth);
        }
    }

    #[doc(alias = "gtk_snapshot_pop")]
    fn pop(&self) {
        unsafe {
            ffi::gtk_snapshot_pop(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_blend")]
    fn push_blend(&self, blend_mode: gsk::BlendMode) {
        unsafe {
            ffi::gtk_snapshot_push_blend(self.as_ref().to_glib_none().0, blend_mode.into_glib());
        }
    }

    #[doc(alias = "gtk_snapshot_push_blur")]
    fn push_blur(&self, radius: f64) {
        unsafe {
            ffi::gtk_snapshot_push_blur(self.as_ref().to_glib_none().0, radius);
        }
    }

    #[doc(alias = "gtk_snapshot_push_clip")]
    fn push_clip(&self, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_push_clip(self.as_ref().to_glib_none().0, bounds.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_color_matrix")]
    fn push_color_matrix(&self, color_matrix: &graphene::Matrix, color_offset: &graphene::Vec4) {
        unsafe {
            ffi::gtk_snapshot_push_color_matrix(
                self.as_ref().to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_cross_fade")]
    fn push_cross_fade(&self, progress: f64) {
        unsafe {
            ffi::gtk_snapshot_push_cross_fade(self.as_ref().to_glib_none().0, progress);
        }
    }

    #[doc(alias = "gtk_snapshot_push_gl_shader")]
    fn push_gl_shader(
        &self,
        shader: &gsk::GLShader,
        bounds: &graphene::Rect,
        take_args: glib::Bytes,
    ) {
        unsafe {
            ffi::gtk_snapshot_push_gl_shader(
                self.as_ref().to_glib_none().0,
                shader.to_glib_none().0,
                bounds.to_glib_none().0,
                take_args.into_glib_ptr(),
            );
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_snapshot_push_mask")]
    fn push_mask(&self, mask_mode: gsk::MaskMode) {
        unsafe {
            ffi::gtk_snapshot_push_mask(self.as_ref().to_glib_none().0, mask_mode.into_glib());
        }
    }

    #[doc(alias = "gtk_snapshot_push_opacity")]
    fn push_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gtk_snapshot_push_opacity(self.as_ref().to_glib_none().0, opacity);
        }
    }

    #[doc(alias = "gtk_snapshot_push_repeat")]
    fn push_repeat(&self, bounds: &graphene::Rect, child_bounds: Option<&graphene::Rect>) {
        unsafe {
            ffi::gtk_snapshot_push_repeat(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                child_bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_rounded_clip")]
    fn push_rounded_clip(&self, bounds: &gsk::RoundedRect) {
        unsafe {
            ffi::gtk_snapshot_push_rounded_clip(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_shadow")]
    fn push_shadow(&self, shadow: &[gsk::Shadow]) {
        let n_shadows = shadow.len() as _;
        unsafe {
            ffi::gtk_snapshot_push_shadow(
                self.as_ref().to_glib_none().0,
                shadow.to_glib_none().0,
                n_shadows,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_snapshot_render_background")]
    fn render_background(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_background(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_snapshot_render_focus")]
    fn render_focus(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_focus(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_snapshot_render_frame")]
    fn render_frame(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_frame(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_snapshot_render_insertion_cursor")]
    fn render_insertion_cursor(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        layout: &pango::Layout,
        index: i32,
        direction: pango::Direction,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_insertion_cursor(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                layout.to_glib_none().0,
                index,
                direction.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_snapshot_render_layout")]
    fn render_layout(
        &self,
        context: &impl IsA<StyleContext>,
        x: f64,
        y: f64,
        layout: &pango::Layout,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_layout(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                layout.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_restore")]
    fn restore(&self) {
        unsafe {
            ffi::gtk_snapshot_restore(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_rotate")]
    fn rotate(&self, angle: f32) {
        unsafe {
            ffi::gtk_snapshot_rotate(self.as_ref().to_glib_none().0, angle);
        }
    }

    #[doc(alias = "gtk_snapshot_rotate_3d")]
    fn rotate_3d(&self, angle: f32, axis: &graphene::Vec3) {
        unsafe {
            ffi::gtk_snapshot_rotate_3d(
                self.as_ref().to_glib_none().0,
                angle,
                axis.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_save")]
    fn save(&self) {
        unsafe {
            ffi::gtk_snapshot_save(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_scale")]
    fn scale(&self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::gtk_snapshot_scale(self.as_ref().to_glib_none().0, factor_x, factor_y);
        }
    }

    #[doc(alias = "gtk_snapshot_scale_3d")]
    fn scale_3d(&self, factor_x: f32, factor_y: f32, factor_z: f32) {
        unsafe {
            ffi::gtk_snapshot_scale_3d(
                self.as_ref().to_glib_none().0,
                factor_x,
                factor_y,
                factor_z,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_to_node")]
    fn to_node(self) -> Option<gsk::RenderNode> {
        unsafe { from_glib_full(ffi::gtk_snapshot_to_node(self.upcast().into_glib_ptr())) }
    }

    #[doc(alias = "gtk_snapshot_to_paintable")]
    fn to_paintable(self, size: Option<&graphene::Size>) -> Option<gdk::Paintable> {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_to_paintable(
                self.upcast().into_glib_ptr(),
                size.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_transform")]
    fn transform(&self, transform: Option<&gsk::Transform>) {
        unsafe {
            ffi::gtk_snapshot_transform(self.as_ref().to_glib_none().0, transform.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_transform_matrix")]
    fn transform_matrix(&self, matrix: &graphene::Matrix) {
        unsafe {
            ffi::gtk_snapshot_transform_matrix(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_translate")]
    fn translate(&self, point: &graphene::Point) {
        unsafe {
            ffi::gtk_snapshot_translate(self.as_ref().to_glib_none().0, point.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_translate_3d")]
    fn translate_3d(&self, point: &graphene::Point3D) {
        unsafe {
            ffi::gtk_snapshot_translate_3d(self.as_ref().to_glib_none().0, point.to_glib_none().0);
        }
    }
}

impl<O: IsA<Snapshot>> SnapshotExt for O {}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Snapshot")
    }
}
