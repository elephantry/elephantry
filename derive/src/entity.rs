pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let attribute = ast.attrs.iter().find(|a| {
        a.path.segments.len() == 1 && a.path.segments[0].ident == "entity"
    });

    let parameters = match attribute {
        Some(attribute) => {
            syn::parse2(attribute.tokens.clone())
                .expect("Invalid entity attribute!")
        },
        None => crate::Params::default(),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let from_body = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        if is_option(ty) {
            quote::quote! {
                #name: tuple.try_get(stringify!(#name)).ok()
            }
        }
        else {
            quote::quote! {
                #name: tuple.get(stringify!(#name))
            }
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
        }
        else {
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
    }
    else {
        quote::quote! {
            elephantry
        }
    };

    let gen = quote::quote! {
        impl #elephantry::Entity for #name
        {
            fn from(tuple: &#elephantry::Tuple<'_>) -> Self
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
