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
        impl #impl_generics #elephantry::Composite for #name #ty_generics #where_clause {
            fn name() -> &'static str {
                stringify!(#name)
            }

            fn to_vec(&self) -> Vec<&dyn #elephantry::ToSql> {
                let mut vec = Vec::new();

                #(#to_vec_body; )*

                vec
            }

            fn from_text_values(ty: &#elephantry::pq::Type, values: &[Option<&str>]) -> #elephantry::Result<Box<Self>> {
                use #elephantry::FromSql;

                let s = Self {
                    #(#from_text_body, )*
                };

                Ok(Box::new(s))
            }

            fn from_binary_values(ty: &#elephantry::pq::Type, values: &[Option<&[u8]>]) -> #elephantry::Result<Box<Self>> {
                use #elephantry::FromSql;

                let s = Self {
                    #(#from_binary_body, )*
                };

                Ok(Box::new(s))
            }
        }
    };

    Ok(gen)
}
