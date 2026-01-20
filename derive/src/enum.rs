use darling::FromVariant as _;

pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => return crate::error(ast, "this derive macro only works on enum"),
    };

    let name = &ast.ident;
    let elephantry = crate::elephantry();

    let mut from_text_body = Vec::new();
    let mut to_text_body = Vec::new();

    for variant in variants {
        let name = &variant.ident;

        let params = crate::params::Value::from_variant(variant)?;
        let value = params.value.unwrap_or_else(|| name.to_string());

        from_text_body.push(quote::quote! {
            #value => Self::#name
        });

        to_text_body.push(quote::quote! {
            Self::#name => #value
        });
    }

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
                let value = match self {
                    #(#to_text_body, )*
                };

                Ok(value.to_string())
            }
        }

        #[automatically_derived]
        impl #impl_generics #elephantry::entity::Simple for #name #ty_generics #where_clause {
        }
    };

    Ok(r#gen)
}
