// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{TextBuffer, TextChildAnchor, TextMark, TextSearchFlags, TextTag};
use glib::{prelude::*, translate::*};
use std::cmp;

glib::wrapper! {
    #[derive(Debug)]
    pub struct TextIter(BoxedInline<ffi::GtkTextIter>);

    match fn {
        copy => |ptr| ffi::gtk_text_iter_copy(ptr),
        free => |ptr| ffi::gtk_text_iter_free(ptr),
        type_ => || ffi::gtk_text_iter_get_type(),
    }
}

impl TextIter {
    #[doc(alias = "gtk_text_iter_assign")]
    pub fn assign(&mut self, other: &TextIter) {
        unsafe {
            ffi::gtk_text_iter_assign(self.to_glib_none_mut().0, other.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_text_iter_backward_char")]
    pub fn backward_char(&mut self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_backward_char(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "gtk_text_iter_backward_chars")]
    pub fn backward_chars(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_chars(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_cursor_position")]
    pub fn backward_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_cursor_position(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_cursor_positions")]
    pub fn backward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_cursor_positions(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_find_char")]
    pub fn backward_find_char<P: FnMut(char) -> bool>(
        &mut self,
        pred: P,
        limit: Option<&TextIter>,
    ) -> bool {
        let pred_data: P = pred;
        unsafe extern "C" fn pred_func<P: FnMut(char) -> bool>(
            ch: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let ch = std::convert::TryFrom::try_from(ch)
                .expect("conversion from an invalid Unicode value attempted");
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(ch).into_glib()
        }
        let pred = Some(pred_func::<P> as _);
        let super_callback0: &P = &pred_data;
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_find_char(
                self.to_glib_none_mut().0,
                pred,
                super_callback0 as *const _ as usize as *mut _,
                limit.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_line")]
    pub fn backward_line(&mut self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_backward_line(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "gtk_text_iter_backward_lines")]
    pub fn backward_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_lines(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_search")]
    pub fn backward_search(
        &self,
        str: &str,
        flags: TextSearchFlags,
        limit: Option<&TextIter>,
    ) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut match_start = TextIter::uninitialized();
            let mut match_end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_iter_backward_search(
                self.to_glib_none().0,
                str.to_glib_none().0,
                flags.into_glib(),
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                limit.to_glib_none().0,
            ));
            if ret {
                Some((match_start, match_end))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_text_iter_backward_sentence_start")]
    pub fn backward_sentence_start(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_sentence_start(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_sentence_starts")]
    pub fn backward_sentence_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_sentence_starts(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_to_tag_toggle")]
    pub fn backward_to_tag_toggle(&mut self, tag: Option<&impl IsA<TextTag>>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_to_tag_toggle(
                self.to_glib_none_mut().0,
                tag.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_visible_cursor_position")]
    pub fn backward_visible_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_cursor_position(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_visible_cursor_positions")]
    pub fn backward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_cursor_positions(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_visible_line")]
    pub fn backward_visible_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_line(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_visible_lines")]
    pub fn backward_visible_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_lines(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_visible_word_start")]
    pub fn backward_visible_word_start(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_word_start(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_visible_word_starts")]
    pub fn backward_visible_word_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_word_starts(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_word_start")]
    pub fn backward_word_start(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_word_start(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_backward_word_starts")]
    pub fn backward_word_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_word_starts(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_can_insert")]
    pub fn can_insert(&self, default_editability: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_can_insert(
                self.to_glib_none().0,
                default_editability.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_compare")]
    fn compare(&self, rhs: &TextIter) -> i32 {
        unsafe { ffi::gtk_text_iter_compare(self.to_glib_none().0, rhs.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_editable")]
    pub fn editable(&self, default_setting: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_editable(
                self.to_glib_none().0,
                default_setting.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_ends_line")]
    pub fn ends_line(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_ends_line(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_ends_sentence")]
    pub fn ends_sentence(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_ends_sentence(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_ends_tag")]
    pub fn ends_tag(&self, tag: Option<&impl IsA<TextTag>>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_ends_tag(
                self.to_glib_none().0,
                tag.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_ends_word")]
    pub fn ends_word(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_ends_word(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_equal")]
    fn equal(&self, rhs: &TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_equal(
                self.to_glib_none().0,
                rhs.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_char")]
    pub fn forward_char(&mut self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_forward_char(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "gtk_text_iter_forward_chars")]
    pub fn forward_chars(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_chars(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_cursor_position")]
    pub fn forward_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_cursor_position(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_cursor_positions")]
    pub fn forward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_cursor_positions(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_find_char")]
    pub fn forward_find_char<P: FnMut(char) -> bool>(
        &mut self,
        pred: P,
        limit: Option<&TextIter>,
    ) -> bool {
        let pred_data: P = pred;
        unsafe extern "C" fn pred_func<P: FnMut(char) -> bool>(
            ch: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let ch = std::convert::TryFrom::try_from(ch)
                .expect("conversion from an invalid Unicode value attempted");
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(ch).into_glib()
        }
        let pred = Some(pred_func::<P> as _);
        let super_callback0: &P = &pred_data;
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_find_char(
                self.to_glib_none_mut().0,
                pred,
                super_callback0 as *const _ as usize as *mut _,
                limit.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_line")]
    pub fn forward_line(&mut self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_forward_line(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "gtk_text_iter_forward_lines")]
    pub fn forward_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_lines(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_search")]
    pub fn forward_search(
        &self,
        str: &str,
        flags: TextSearchFlags,
        limit: Option<&TextIter>,
    ) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut match_start = TextIter::uninitialized();
            let mut match_end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_iter_forward_search(
                self.to_glib_none().0,
                str.to_glib_none().0,
                flags.into_glib(),
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                limit.to_glib_none().0,
            ));
            if ret {
                Some((match_start, match_end))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_text_iter_forward_sentence_end")]
    pub fn forward_sentence_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_sentence_end(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_sentence_ends")]
    pub fn forward_sentence_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_sentence_ends(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_to_end")]
    pub fn forward_to_end(&mut self) {
        unsafe {
            ffi::gtk_text_iter_forward_to_end(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gtk_text_iter_forward_to_line_end")]
    pub fn forward_to_line_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_to_line_end(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_to_tag_toggle")]
    pub fn forward_to_tag_toggle(&mut self, tag: Option<&impl IsA<TextTag>>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_to_tag_toggle(
                self.to_glib_none_mut().0,
                tag.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_visible_cursor_position")]
    pub fn forward_visible_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_cursor_position(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_visible_cursor_positions")]
    pub fn forward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_cursor_positions(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_visible_line")]
    pub fn forward_visible_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_line(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_visible_lines")]
    pub fn forward_visible_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_lines(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_visible_word_end")]
    pub fn forward_visible_word_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_word_end(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_visible_word_ends")]
    pub fn forward_visible_word_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_word_ends(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_word_end")]
    pub fn forward_word_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_word_end(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_forward_word_ends")]
    pub fn forward_word_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_word_ends(
                self.to_glib_none_mut().0,
                count,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_buffer")]
    #[doc(alias = "get_buffer")]
    pub fn buffer(&self) -> TextBuffer {
        unsafe { from_glib_none(ffi::gtk_text_iter_get_buffer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_get_bytes_in_line")]
    #[doc(alias = "get_bytes_in_line")]
    pub fn bytes_in_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_bytes_in_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_char")]
    #[doc(alias = "get_char")]
    pub fn char(&self) -> char {
        unsafe {
            std::convert::TryFrom::try_from(ffi::gtk_text_iter_get_char(self.to_glib_none().0))
                .expect("conversion from an invalid Unicode value attempted")
        }
    }

    #[doc(alias = "gtk_text_iter_get_chars_in_line")]
    #[doc(alias = "get_chars_in_line")]
    pub fn chars_in_line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_chars_in_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_child_anchor")]
    #[doc(alias = "get_child_anchor")]
    pub fn child_anchor(&self) -> Option<TextChildAnchor> {
        unsafe { from_glib_none(ffi::gtk_text_iter_get_child_anchor(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_get_language")]
    #[doc(alias = "get_language")]
    pub fn language(&self) -> pango::Language {
        unsafe { from_glib_full(ffi::gtk_text_iter_get_language(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_get_line")]
    #[doc(alias = "get_line")]
    pub fn line(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_line_index")]
    #[doc(alias = "get_line_index")]
    pub fn line_index(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line_index(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_line_offset")]
    #[doc(alias = "get_line_offset")]
    pub fn line_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_line_offset(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_marks")]
    #[doc(alias = "get_marks")]
    pub fn marks(&self) -> glib::SList<TextMark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_iter_get_marks(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_offset")]
    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_offset(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_paintable")]
    #[doc(alias = "get_paintable")]
    pub fn paintable(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::gtk_text_iter_get_paintable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_get_slice")]
    #[doc(alias = "get_slice")]
    pub fn slice(&self, end: &TextIter) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_slice(
                self.to_glib_none().0,
                end.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_tags")]
    #[doc(alias = "get_tags")]
    pub fn tags(&self) -> glib::SList<TextTag> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_iter_get_tags(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_text")]
    #[doc(alias = "get_text")]
    pub fn text(&self, end: &TextIter) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_text(
                self.to_glib_none().0,
                end.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_toggled_tags")]
    #[doc(alias = "get_toggled_tags")]
    pub fn toggled_tags(&self, toggled_on: bool) -> glib::SList<TextTag> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_iter_get_toggled_tags(
                self.to_glib_none().0,
                toggled_on.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_visible_line_index")]
    #[doc(alias = "get_visible_line_index")]
    pub fn visible_line_index(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_visible_line_index(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_visible_line_offset")]
    #[doc(alias = "get_visible_line_offset")]
    pub fn visible_line_offset(&self) -> i32 {
        unsafe { ffi::gtk_text_iter_get_visible_line_offset(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_iter_get_visible_slice")]
    #[doc(alias = "get_visible_slice")]
    pub fn visible_slice(&self, end: &TextIter) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_visible_slice(
                self.to_glib_none().0,
                end.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_visible_text")]
    #[doc(alias = "get_visible_text")]
    pub fn visible_text(&self, end: &TextIter) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_visible_text(
                self.to_glib_none().0,
                end.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_has_tag")]
    pub fn has_tag(&self, tag: &impl IsA<TextTag>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_has_tag(
                self.to_glib_none().0,
                tag.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_in_range")]
    pub fn in_range(&self, start: &TextIter, end: &TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_in_range(
                self.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_inside_sentence")]
    pub fn inside_sentence(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_inside_sentence(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_inside_word")]
    pub fn inside_word(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_inside_word(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_is_cursor_position")]
    pub fn is_cursor_position(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_is_cursor_position(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_is_end")]
    pub fn is_end(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_is_end(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_is_start")]
    pub fn is_start(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_is_start(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_order")]
    pub fn order(&mut self, second: &mut TextIter) {
        unsafe {
            ffi::gtk_text_iter_order(self.to_glib_none_mut().0, second.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gtk_text_iter_set_line")]
    pub fn set_line(&mut self, line_number: i32) {
        unsafe {
            ffi::gtk_text_iter_set_line(self.to_glib_none_mut().0, line_number);
        }
    }

    #[doc(alias = "gtk_text_iter_set_line_index")]
    pub fn set_line_index(&mut self, byte_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_line_index(self.to_glib_none_mut().0, byte_on_line);
        }
    }

    #[doc(alias = "gtk_text_iter_set_line_offset")]
    pub fn set_line_offset(&mut self, char_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_line_offset(self.to_glib_none_mut().0, char_on_line);
        }
    }

    #[doc(alias = "gtk_text_iter_set_offset")]
    pub fn set_offset(&mut self, char_offset: i32) {
        unsafe {
            ffi::gtk_text_iter_set_offset(self.to_glib_none_mut().0, char_offset);
        }
    }

    #[doc(alias = "gtk_text_iter_set_visible_line_index")]
    pub fn set_visible_line_index(&mut self, byte_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_visible_line_index(self.to_glib_none_mut().0, byte_on_line);
        }
    }

    #[doc(alias = "gtk_text_iter_set_visible_line_offset")]
    pub fn set_visible_line_offset(&mut self, char_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_visible_line_offset(self.to_glib_none_mut().0, char_on_line);
        }
    }

    #[doc(alias = "gtk_text_iter_starts_line")]
    pub fn starts_line(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_starts_line(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_starts_sentence")]
    pub fn starts_sentence(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_starts_sentence(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_starts_tag")]
    pub fn starts_tag(&self, tag: Option<&impl IsA<TextTag>>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_starts_tag(
                self.to_glib_none().0,
                tag.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_starts_word")]
    pub fn starts_word(&self) -> bool {
        unsafe { from_glib(ffi::gtk_text_iter_starts_word(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_text_iter_toggles_tag")]
    pub fn toggles_tag(&self, tag: Option<&impl IsA<TextTag>>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_toggles_tag(
                self.to_glib_none().0,
                tag.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

impl PartialOrd for TextIter {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for TextIter {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl PartialEq for TextIter {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TextIter {}
