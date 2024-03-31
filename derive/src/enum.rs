pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let parameters = crate::params::Container::from_ast(ast)?;

    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => return crate::error(ast, "this derive macro only works on enum"),
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

    let from_text_body = variants.iter().map(|variant| {
        let name = &variant.ident;

        quote::quote! {
            stringify!(#name) => Self::#name
        }
    });

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::FromSql for #name #ty_generics #where_clause {
            /*
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L150
             */
            fn from_text(ty: &#elephantry::pq::Type, raw: ::std::option::Option<&str>) -> #elephantry::Result<Self> {
                let buf = #elephantry::from_sql::not_null(raw)?;

                let value = match buf {
                    #(#from_text_body, )*
                    _ => unreachable!(),
                };

                ::std::result::Result::Ok(value)
            }

            /*
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L174
             */
            fn from_binary(ty: &#elephantry::pq::Type, raw: ::std::option::Option<&[u8]>) -> #elephantry::Result<Self> {
                let buf = #elephantry::from_sql::not_null(raw)?;
                let s = ::std::string::String::from_utf8(buf.to_vec())?;

                Self::from_text(ty, ::std::option::Option::Some(&s))
            }
        }

        #[automatically_derived]
        impl #impl_generics #elephantry::ToSql for #name #ty_generics #where_clause {
            fn ty(&self) -> #elephantry::pq::Type {
                #elephantry::pq::types::Type {
                    oid: 0,
                    descr: stringify!(#name),
                    name: stringify!(#name),
                    kind: #elephantry::pq::types::Kind::Enum,
                }
            }

            /*
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L216
             */
            fn to_text(&self) -> #elephantry::Result<::std::option::Option<String>> {
                format!("{self:?}").to_text()
            }

            /*
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L110
             */
            fn to_binary(&self) -> #elephantry::Result<::std::option::Option<Vec<u8>>> {
                format!("{:?}", self).to_binary()
            }
        }

        #[automatically_derived]
        impl #impl_generics #elephantry::entity::Simple for #name #ty_generics #where_clause {
        }
    };

    Ok(gen)
}
