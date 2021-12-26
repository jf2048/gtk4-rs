// Take a look at the license at the top of the repository in the LICENSE file.

use crate::util::*;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{parse::Parse, Token};

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on the `impl` block for a widget subclass or widget wrapper";

mod keywords {
    syn::custom_keyword!(value);
    syn::custom_keyword!(subclass);
    syn::custom_keyword!(name);
    syn::custom_keyword!(parameter_type);
}

pub enum Mode {
    Value,
    Subclass,
}

pub struct Args {
    mode: Mode,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut mode = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::value) {
                let kw = input.parse::<keywords::value>()?;
                if mode.is_some() {
                    abort!(kw, "Only one of `value` or `subclass` is allowed");
                }
                mode = Some(Mode::Value);
            } else if lookahead.peek(keywords::subclass) {
                let kw = input.parse::<keywords::subclass>()?;
                if mode.is_some() {
                    abort!(kw, "Only one of `value` or `subclass` is allowed");
                }
                mode = Some(Mode::Subclass);
            } else {
                return Err(lookahead.error());
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        if let Some(mode) = mode {
            Ok(Args { mode })
        } else {
            abort_call_site!("One of `value` or `subclass` must be specified");
        }
    }
}

pub struct ActionArgs {
    name: Option<String>,
    parameter_type: Option<String>,
}

impl Parse for ActionArgs {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self {
            name: None,
            parameter_type: None,
        };
        if stream.is_empty() {
            return Ok(args);
        }
        let input;
        syn::parenthesized!(input in stream);
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::name) {
                let kw = input.parse::<keywords::name>()?;
                if args.name.is_some() {
                    return Err(syn::Error::new_spanned(kw, "Duplicate `name` attribute"));
                }
                input.parse::<Token![=]>()?;
                let name = input.parse::<syn::LitStr>()?;
                args.name.replace(name.value());
            } else if lookahead.peek(keywords::parameter_type) {
                let kw = input.parse::<keywords::parameter_type>()?;
                if args.parameter_type.is_some() {
                    return Err(syn::Error::new_spanned(kw, "Duplicate `parameter_type` attribute"));
                }
                input.parse::<Token![=]>()?;
                let ty = input.parse::<syn::LitStr>()?;
                args.parameter_type.replace(ty.value());
            } else {
                return Err(lookahead.error());
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(args)
    }
}

pub fn impl_widget_actions(mut input: syn::ItemImpl, args: Args) -> TokenStream {
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = &mut input;
    if trait_.is_some() {
        abort_call_site!(WRONG_PLACE_MSG);
    }
    let gtk = crate_ident_new();

    let from_instance = match args.mode {
        Mode::Value => quote! {},
        Mode::Subclass => quote! {
            let widget = <#self_ty as #gtk::glib::subclass::types::ObjectSubclassExt>::from_instance(widget);
        },
    };

    let mut actions = vec![];
    for item in items.iter_mut() {
        if let syn::ImplItem::Method(method) = item {
            let mut i = 0;
            let mut attr = None;
            while i < method.attrs.len() {
                if method.attrs[i].path.is_ident("widget_action") {
                    if attr.is_some() {
                        abort!(method.attrs[i], "Duplicate `widget_action` attribute");
                    }
                    attr.replace(method.attrs.remove(i));
                } else {
                    i += 1;
                }
            }

            let attr = match attr {
                Some(attr) => attr,
                None => continue,
            };

            let ident = &method.sig.ident;
            let action_args =
                syn::parse2::<ActionArgs>(attr.tokens).unwrap_or_else(|e| abort!(e));
            let name = action_args
                .name
                .as_ref()
                .cloned()
                .unwrap_or_else(|| ident.to_string().replace("_", "-"));

            let mut inputs = method.sig.inputs.iter();
            let recv = inputs.next();
            if !matches!(recv, Some(syn::FnArg::Receiver(syn::Receiver {
                reference: Some(_),
                mutability: None,
                ..
            }))) {
                abort!(method.sig, "First argument to widget action must be `&self`");
            }
            let (param_get, param, call) = match inputs.next() {
                Some(syn::FnArg::Typed(param)) => {
                    let (param_get, param_str, param_from) = match action_args.parameter_type {
                        Some(s) => (
                            quote! {},
                            quote! { #s },
                            quote! {}
                        ),
                        None => {
                            let ty = &param.ty;
                            (
                                quote! {
                                    let ty = <#ty as #gtk::glib::StaticVariantType>::static_variant_type();
                                },
                                quote! { ty.as_str() },
                                quote! {
                                    let _param = <#ty as #gtk::glib::FromVariant>::from_variant(_param).unwrap();
                                },
                            )
                        }
                    };
                    (
                        param_get,
                        quote! { ::std::option::Option::Some(#param_str) },
                        quote! {
                            let _param = _param.unwrap();
                            #param_from
                            #self_ty::#ident(widget, _param);
                        },
                    )
                },
                None => {
                    (
                        quote! {},
                        quote! { ::std::option::Option::None },
                        quote! {
                            #self_ty::#ident(widget);
                        },
                    )
                },
                _ => unimplemented!(),
            };
            if inputs.next().is_some() {
                abort!(method.sig.inputs, "Only 1 or 2 arguments allowed for widget action");
            }

            actions.push(quote! {
                (
                    #name,
                    |name, klass| {
                        #param_get
                        klass.install_action(
                            name,
                            #param,
                            |widget, _name, _param| {
                                #from_instance
                                #call
                            }
                        );
                    }
                )
            });
        }
    }

    let widget_ty = match args.mode {
        Mode::Value => quote! { #self_ty },
        Mode::Subclass => quote! {
            <#self_ty as #gtk::glib::subclass::types::ObjectSubclass>::Type
        },
    };

    quote! {
        #(#attrs)*
        impl #generics #self_ty {
            #(#items)*
        }

        impl #gtk::subclass::widget::WidgetActions for #self_ty {
            type WidgetActionType = #widget_ty;
            const ACTIONS: &'static [#gtk::subclass::widget::WidgetActionDef<<Self as #gtk::subclass::widget::WidgetActions>::WidgetActionType>] = &[
                #(#actions),*
            ];
        }
    }
}
