#![warn(rust_2018_idioms)]

#[derive(Clone, Debug)]
struct Params {
    internal: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self { internal: false }
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

        Ok(Params { internal })
    }
}

#[proc_macro_derive(Entity, attributes(entity))]
pub fn entity_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_entity_macro(&ast)
}

fn impl_entity_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let attribute = ast
        .attrs
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "entity");

    let parameters = match attribute {
        Some(attribute) => {
            syn::parse2(attribute.tokens.clone()).expect("Invalid entity attribute!")
        }
        None => Params::default(),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let from_body = fields.iter().map(|field| {
        let name = &field.ident;

        quote::quote! {
            #name: tuple.get(stringify!(#name))
        }
    });

    let get_body = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        if is_option(ty) {
            quote::quote! {
                stringify!(#name) => match self.#name {
                    Some(ref value) => Some(value),
                    None => None,
                }
            }
        } else {
            quote::quote! {
                stringify!(#name) => Some(&self.#name)
            }
        }
    });

    let name = &ast.ident;
    let elephantry = if parameters.internal {
        quote::quote! {
            crate
        }
    } else {
        quote::quote! {
            elephantry
        }
    };

    let gen = quote::quote! {
        impl #elephantry::Entity for #name
        {
            fn from(tuple: &#elephantry::pq::Tuple<'_>) -> Self
            {
                Self {
                    #(#from_body, )*
                }
            }

            fn get(&self, field: &str) -> Option<&dyn #elephantry::ToSql> {
                match field {
                    #(#get_body, )*
                    _ => None,
                }
            }
        }
    };

    gen.into()
}

fn is_option(ty: &syn::Type) -> bool {
    let typepath = match ty {
        syn::Type::Path(typepath) => typepath,
        _ => unimplemented!(),
    };

    typepath.path.leading_colon.is_none()
        && typepath.path.segments.len() == 1
        && typepath.path.segments.iter().next().unwrap().ident == "Option"
}
