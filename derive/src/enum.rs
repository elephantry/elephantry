pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let parameters = crate::params::Container::from_ast(ast);

    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
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

    let from_text_body = variants.iter().map(|variant| {
        let name = &variant.ident;

        quote::quote! {
            stringify!(#name) => Self::#name
        }
    });

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::Enum for #name #ty_generics #where_clause {
            fn name() -> &'static str {
                stringify!(#name)
            }

            fn from_text(value: &str) -> #elephantry::Result<Box<Self>> {
                let v = match value {
                    #(#from_text_body, )*
                    _ => unreachable!(),
                };

                Ok(Box::new(v))
            }
        }
    };

    gen.into()
}
