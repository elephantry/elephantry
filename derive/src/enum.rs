pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => return crate::error(ast, "this derive macro only works on enum"),
    };

    let name = &ast.ident;
    let elephantry = crate::elephantry();

    let from_text_body = variants.iter().map(|variant| {
        let name = &variant.ident;

        quote::quote! {
            stringify!(#name) => Self::#name
        }
    });

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let r#gen = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::FromText for #name #ty_generics #where_clause {
            /*
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L150
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L174
             */
            fn from_text(raw: &str) -> #elephantry::Result<Self> {
                let value = match raw {
                    #(#from_text_body, )*
                    _ => return ::std::result::Result::Err(Self::error(raw)),
                };

                ::std::result::Result::Ok(value)
            }
        }

        #[automatically_derived]
        impl #impl_generics #elephantry::ToText for #name #ty_generics #where_clause {
            /*
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L216
             * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/enum.c#L110
             */
            fn to_text(&self) -> #elephantry::Result<String> {
                Ok(format!("{self:?}"))
            }
        }

        #[automatically_derived]
        impl #impl_generics #elephantry::entity::Simple for #name #ty_generics #where_clause {
        }
    };

    Ok(r#gen)
}
