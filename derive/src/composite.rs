pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let attribute = ast.attrs.iter().find(|a| {
        a.path.segments.len() == 1 && a.path.segments[0].ident == "composite"
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

    let to_vec_body = fields.iter().map(|field| {
        let name = &field.ident;

        quote::quote! {
            vec.push(&self.#name as &dyn #elephantry::ToSql)
        }
    });

    let from_text_body = fields.iter().enumerate().map(|(x, field)| {
        let name = &field.ident;
        let ty = &field.ty;
        crate::check_type(ty);

        quote::quote! {
            #name: <#ty>::from_text(ty, values[#x])?
        }
    });

    let from_binary_body = fields.iter().enumerate().map(|(x, field)| {
        let name = &field.ident;
        let ty = &field.ty;

        quote::quote! {
            #name: <#ty>::from_binary(ty, values[#x])?
        }
    });

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

    gen.into()
}
