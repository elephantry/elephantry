#![warn(rust_2018_idioms)]

mod composite;
mod entity;
mod r#enum;

#[derive(Clone, Debug)]
struct Params {
    internal: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            internal: false,
        }
    }
}

impl syn::parse::Parse for Params {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let internal = match content.parse::<syn::Ident>() {
            Ok(internal) => internal == "internal",
            Err(_) => false,
        };

        Ok(Params {
            internal,
        })
    }
}

#[proc_macro_derive(Composite, attributes(composite))]
pub fn composite_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    composite::impl_macro(&ast)
}

#[proc_macro_derive(Entity, attributes(entity))]
pub fn entity_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    entity::impl_macro(&ast)
}

#[proc_macro_derive(Enum, attributes(r#enum))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    r#enum::impl_macro(&ast)
}
