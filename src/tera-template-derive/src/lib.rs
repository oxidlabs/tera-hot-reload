extern crate proc_macro;
use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta};

#[proc_macro_derive(TeraTemplate, attributes(template))]
pub fn tera_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("template"))
        .unwrap();

    let template_path = match attr.parse_args() {
        Ok(Meta::NameValue(meta)) => {
            if let Expr::Lit(expr) = &meta.value {
                if let Lit::Str(lit_str) = &expr.lit {
                    lit_str.value()
                } else {
                    panic!("Expected a string literal");
                }
            } else {
                panic!("Expected a string literal");
            }
        }
        _ => panic!("Expected a string literal"),
    };

    let expanded = quote! {

        impl #name {
            fn render(&self) -> String {
                let tera = tera::Tera::new("templates/**/*").expect("Failed to create Tera instance");
                let context = tera::Context::from_serialize(self).expect("Failed to create context");
                let rendered = tera
                    .render(#template_path, &context)
                    .expect("Failed to render template");
                rendered
            }
        }
    };

    expanded.into()
}