// Take a look at the license at the top of the repository in the LICENSE file.

//! # GTK 4 Macros
//!
//! The crate aims to provide useful macros to use with the GTK 4 Rust bindings.

mod attribute_parser;
mod composite_template_derive;
mod util;
mod widget_actions_attribute;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for using a composite template in a widget.
///
/// The `template` attribute specifies where the template should be loaded
/// from;  it can be a `file`, a `resource`, or a `string`.
///
/// The `template_child` attribute is used to mark all internal widgets
/// we need to have programmatic access to.
///
/// The `template_child` attribute can take two parameters:
///     - `id` which defaults to the item name if not defined
///     - `internal_child` whether the child should be accessible as an “internal-child”, defaults to `false`
///
/// # Example
///
/// Specify that `MyWidget` is using a composite template and load the
/// template file the `composite_template.ui` file.
///
/// Then, in the `ObjectSubclass` implementation you will need to call
/// `bind_template` in the `class_init` function, and `init_template` in
/// `instance_init` function.
///
///
/// ```no_run
/// # fn main() {}
/// use gtk::prelude::*;
/// use gtk::glib;
/// use gtk::CompositeTemplate;
/// use gtk::subclass::prelude::*;
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, CompositeTemplate)]
///     #[template(file = "test/template.ui")]
///     pub struct MyWidget {
///         #[template_child]
///         pub label: TemplateChild<gtk::Label>,
///         #[template_child(id = "my_button_id")]
///         pub button: TemplateChild<gtk::Button>,
///     }
///
///     #[glib::object_subclass]
///     impl ObjectSubclass for MyWidget {
///         const NAME: &'static str = "MyWidget";
///         type Type = super::MyWidget;
///         type ParentType = gtk::Box;
///
///         fn class_init(klass: &mut Self::Class) {
///             Self::bind_template(klass);
///         }
///
///         fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
///             obj.init_template();
///         }
///     }
///
///     impl ObjectImpl for MyWidget {}
///     impl WidgetImpl for MyWidget {}
///     impl BoxImpl for MyWidget {}
/// }
///
/// glib::wrapper! {
///     pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget, gtk::Box;
/// }
///
/// impl MyWidget {
///     pub fn new() -> Self {
///         glib::Object::new(&[]).expect("Failed to create an instance of MyWidget")
///     }
/// }
/// ```
#[proc_macro_derive(CompositeTemplate, attributes(template, template_child))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}

/// Attribute macro for installing methods as widget actions.
///
/// ```no_run
/// # fn main() {}
/// use gtk::prelude::*;
/// use gtk::glib;
/// use glib::subclass::prelude::*;
/// use gtk::subclass::prelude::*;
/// use std::cell::Cell;
///
/// glib::wrapper! {
///     pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
/// }
///
/// #[gtk::widget_actions(value)]
/// impl MyWidget {
///     pub fn new() -> Self {
///         glib::Object::new(&[]).expect("Failed to create an instance of MyWidget")
///     }
///     #[widget_action(name = "widget.reset-value")]
///     pub fn reset_value(&self) {
///         self.impl_().state.set(0);
///     }
///     #[widget_action(name = "widget.set-value")]
///     pub fn set_value(&self, value: i32) {
///         self.impl_().state.set(value);
///     }
/// }
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default)]
///     pub struct MyWidget {
///         pub state: Cell<i32>,
///     }
///
///     #[glib::object_subclass]
///     impl ObjectSubclass for MyWidget {
///         const NAME: &'static str = "MyWidget";
///         type Type = super::MyWidget;
///         type ParentType = gtk::Widget;
///
///         fn class_init(klass: &mut Self::Class) {
///             Self::install_actions(klass);
///             Self::Type::install_actions(klass);
///         }
///     }
///
///     #[gtk::widget_actions(subclass)]
///     impl MyWidget {
///         #[widget_action(name = "widget.set-value-string", parameter_type = "s")]
///         fn set_value_string(&self, value: &glib::Variant) {
///             if let Some(value) = value.str().and_then(|s| s.parse().ok()) {
///                 self.state.set(value);
///             }
///         }
///     }
///
///     impl ObjectImpl for MyWidget {}
///     impl WidgetImpl for MyWidget {}
/// }
///
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn widget_actions(attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro_error::abort_call_site;
    let args = parse_macro_input!(attr as widget_actions_attribute::Args);
    match syn::parse::<syn::ItemImpl>(item) {
        Ok(input) => widget_actions_attribute::impl_widget_actions(input, args).into(),
        Err(_) => abort_call_site!(widget_actions_attribute::WRONG_PLACE_MSG),
    }
}
