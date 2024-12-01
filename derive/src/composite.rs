pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let name = &ast.ident;
    let elephantry = crate::elephantry();

    let mut to_vec_body = Vec::new();
    let mut from_text_body = Vec::new();
    let mut from_binary_body = Vec::new();

    for (x, field) in fields.iter().enumerate() {
        let ty = &field.ty;
        crate::check_type(ty)?;

        let (to_vec_part, from_text_part, from_binary_part) = if let Some(name) = &field.ident {
            (
                quote::quote! { vec.push(&self.#name as &dyn #elephantry::ToSql) },
                quote::quote! { #name: <#ty>::from_text(ty, values[#x])? },
                quote::quote! { #name: <#ty>::from_binary(ty, values[#x])? },
            )
        } else {
            let x = syn::Index::from(x);

            (
                quote::quote! { vec.push(&self.#x as &dyn #elephantry::ToSql) },
                quote::quote! { <#ty>::from_text(ty, values[#x])? },
                quote::quote! { <#ty>::from_binary(ty, values[#x])? },
            )
        };

        to_vec_body.push(to_vec_part);
        from_text_body.push(from_text_part);
        from_binary_body.push(from_binary_part);
    }

    let (self_from_text, self_from_binary) = if matches!(fields, syn::Fields::Unnamed(_)) {
        (
            quote::quote! { Self ( #(#from_text_body, )*) },
            quote::quote! { Self ( #(#from_binary_body, )*) },
        )
    } else {
        (
            quote::quote! { Self { #(#from_text_body, )* } },
            quote::quote! { Self { #(#from_binary_body, )* } },
        )
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::FromSql for #name #ty_generics #where_clause {
            fn from_text(ty: &#elephantry::pq::Type, raw: ::std::option::Option<&str>) -> #elephantry::Result<Self> {
                let values = #elephantry::record::text_to_vec(raw)?;

                ::std::result::Result::Ok(#self_from_text)
            }

            fn from_binary(ty: &#elephantry::pq::Type, raw: ::std::option::Option<&[u8]>) -> #elephantry::Result<Self> {
                let values = #elephantry::record::binary_to_vec(stringify!(#name), ty, raw)?;

                ::std::result::Result::Ok(#self_from_binary)
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

            fn to_text(&self) -> #elephantry::Result<::std::option::Option<::std::string::String>> {
                let mut vec = ::std::vec::Vec::new();

                #(#to_vec_body; )*

                #elephantry::record::vec_to_text(&vec)
            }

            fn to_binary(&self) -> #elephantry::Result<::std::option::Option<::std::vec::Vec<u8>>> {
                let mut vec = ::std::vec::Vec::new();

                #(#to_vec_body; )*

                #elephantry::record::vec_to_binary(&vec)
            }
        }
    };

    Ok(gen)
}
