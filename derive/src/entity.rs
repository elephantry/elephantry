pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let parameters: crate::Params = ast
        .attrs
        .iter()
        .find(|a| {
            a.path.segments.len() == 1 && a.path.segments[0].ident == "entity"
        })
        .map(|x| {
            syn::parse2(x.tokens.clone()).expect("Invalid entity attribute!")
        })
        .unwrap_or_default();

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let from_body = fields.iter().map(|field| {
        let field_params: crate::FieldParams = field
            .attrs
            .iter()
            .find(|a| {
                a.path.segments.len() == 1
                    && a.path.segments[0].ident == "elephantry"
            })
            .map(|x| {
                syn::parse2(x.tokens.clone())
                    .expect("Invalid entity attribute!")
            })
            .unwrap_or_default();

        let name = &field.ident;
        let ty = &field.ty;
        crate::check_type(ty);

        if field_params.default {
            quote::quote! {
                #name: tuple.try_get(stringify!(#name)).unwrap_or_default()
            }
        }
        else if is_option(ty) {
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

    let (impl_generics, ty_generics, where_clause) =
        ast.generics.split_for_impl();

    let gen = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::Entity for #name #ty_generics #where_clause
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
        && typepath.path.segments.iter().next().map(|x| x.ident.to_string()) == Some("Option".to_string())
}
