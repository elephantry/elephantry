pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let parameters = crate::params::Container::from_ast(ast)?;

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

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

    let mut to_vec_body = Vec::new();
    let mut from_text_body = Vec::new();
    let mut from_binary_body = Vec::new();

    for (x, field) in fields.iter().enumerate() {
        let name = &field.ident;

        let to_vec_part = quote::quote! {
            vec.push(&self.#name as &dyn #elephantry::ToSql)
        };
        to_vec_body.push(to_vec_part);

        let ty = &field.ty;
        crate::check_type(ty)?;

        let from_text_part = quote::quote! {
            #name: <#ty>::from_text(ty, values[#x])?
        };
        from_text_body.push(from_text_part);

        let from_binary_part = quote::quote! {
            #name: <#ty>::from_binary(ty, values[#x])?
        };
        from_binary_body.push(from_binary_part);
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::FromSql for #name #ty_generics #where_clause {
            fn from_text(ty: &#elephantry::pq::Type, raw: ::std::option::Option<&str>) -> #elephantry::Result<Self> {
                let values = #elephantry::record::text_to_vec(raw)?;

                let s = Self {
                    #(#from_text_body, )*
                };

                Ok(s)
            }

            fn from_binary(ty: &#elephantry::pq::Type, raw: ::std::option::Option<&[u8]>) -> #elephantry::Result<Self> {
                let values = #elephantry::record::binary_to_vec(stringify!(#name), ty, raw)?;

                let s = Self {
                    #(#from_binary_body, )*
                };

                Ok(s)
            }
        }

        #[automatically_derived]
        impl #impl_generics #elephantry::ToSql for #name #ty_generics #where_clause {
            fn ty(&self) -> #elephantry::pq::Type {
                #elephantry::pq::types::Type {
                    oid: 0,
                    descr: "",
                    name: stringify!(#name),
                    kind: #elephantry::pq::types::Kind::Composite,

                }
            }

            fn to_text(&self) -> #elephantry::Result<::std::option::Option<Vec<u8>>> {
                let mut vec = Vec::new();

                #(#to_vec_body; )*

                #elephantry::record::vec_to_text(&vec)
            }

            fn to_binary(&self) -> #elephantry::Result<::std::option::Option<Vec<u8>>> {
                let mut vec = Vec::new();

                #(#to_vec_body; )*

                #elephantry::record::vec_to_binary(&vec)
            }
        }
    };

    Ok(gen)
}
