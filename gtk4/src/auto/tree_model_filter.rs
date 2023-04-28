// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{TreeDragSource, TreeIter, TreeModel, TreePath};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt};

glib::wrapper! {
    #[doc(alias = "GtkTreeModelFilter")]
    pub struct TreeModelFilter(Object<ffi::GtkTreeModelFilter, ffi::GtkTreeModelFilterClass>) @implements TreeDragSource, TreeModel;

    match fn {
        type_ => || ffi::gtk_tree_model_filter_get_type(),
    }
}

impl TreeModelFilter {
    pub const NONE: Option<&'static TreeModelFilter> = None;
}

pub trait TreeModelFilterExt: IsA<TreeModelFilter> + 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_clear_cache")]
    fn clear_cache(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_convert_child_iter_to_iter")]
    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut filter_iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_filter_convert_child_iter_to_iter(
                self.as_ref().to_glib_none().0,
                filter_iter.to_glib_none_mut().0,
                mut_override(child_iter.to_glib_none().0),
            ));
            if ret {
                Some(filter_iter)
            } else {
                None
            }
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_convert_child_path_to_path")]
    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_child_path_to_path(
                self.as_ref().to_glib_none().0,
                mut_override(child_path.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_convert_iter_to_child_iter")]
    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter {
        unsafe {
            let mut child_iter = TreeIter::uninitialized();
            ffi::gtk_tree_model_filter_convert_iter_to_child_iter(
                self.as_ref().to_glib_none().0,
                child_iter.to_glib_none_mut().0,
                mut_override(filter_iter.to_glib_none().0),
            );
            child_iter
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_convert_path_to_child_path")]
    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_path_to_child_path(
                self.as_ref().to_glib_none().0,
                mut_override(filter_path.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_get_model")]
    #[doc(alias = "get_model")]
    fn model(&self) -> TreeModel {
        unsafe {
            from_glib_none(ffi::gtk_tree_model_filter_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_refilter")]
    fn refilter(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_refilter(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_set_visible_column")]
    fn set_visible_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_column(self.as_ref().to_glib_none().0, column);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_filter_set_visible_func")]
    fn set_visible_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let model = from_glib_borrow(model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            (*callback)(&model, &iter).into_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "child-model")]
    fn child_model(&self) -> Option<TreeModel> {
        glib::ObjectExt::property(self.as_ref(), "child-model")
    }
}

impl<O: IsA<TreeModelFilter>> TreeModelFilterExt for O {}

impl fmt::Display for TreeModelFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeModelFilter")
    }
}
