pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let params = crate::params::Entity::from_ast(ast);

    let elephantry = if params.internal {
        quote::quote! {
            crate
        }
    } else {
        quote::quote! {
            elephantry
        }
    };

    let entity = entity_impl(ast, &elephantry);
    let structure = structure_impl(ast, &params, &elephantry);
    let model = model_impl(ast, &params, &elephantry);

    let gen = quote::quote! {
        #entity
        #structure
        #model
    };

    gen.into()
}

fn entity_impl(ast: &syn::DeriveInput, elephantry: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let from_body = fields.iter().map(|field| {
        let field_params = crate::params::Field::from_ast(field);

        let name = &field.ident;
        let column = field_params.column.unwrap_or(field.ident.as_ref().unwrap().to_string());
        let ty = &field.ty;
        crate::check_type(ty);

        if field_params.default {
            quote::quote! {
                #name: tuple.try_get(#column).unwrap_or_default()
            }
        } else if is_option(ty) {
            quote::quote! {
                #name: tuple.try_get(#column).ok()
            }
        } else {
            quote::quote! {
                #name: tuple.get(#column)
            }
        }
    });

    let get_body = fields.iter().map(|field| {
        let field_params = crate::params::Field::from_ast(field);

        let name = &field.ident;
        let column = field_params.column.unwrap_or(field.ident.as_ref().unwrap().to_string());
        let ty = &field.ty;

        if is_option(ty) {
            quote::quote! {
                #column => match self.#name {
                    Some(ref value) => Some(value),
                    None => None,
                }
            }
        } else {
            quote::quote! {
                #column => Some(&self.#name)
            }
        }
    });

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote::quote! {
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
    }
}

fn structure_impl(ast: &syn::DeriveInput, params: &crate::params::Entity, elephantry: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let name = match &params.structure {
        Some(name) => name,
        None => return proc_macro2::TokenStream::new(),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let relation = params.relation.clone()
        .unwrap_or(ast.ident.to_string().to_lowercase());

    let primary_key = fields.iter()
        .filter(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params.pk
        })
        .map(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params.column.unwrap_or(field.ident.as_ref().unwrap().to_string())
        });

    let columns = fields.iter()
        .map(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params.column.unwrap_or(field.ident.as_ref().unwrap().to_string())
        });

    quote::quote! {
        struct #name;

        #[automatically_derived]
        impl #elephantry::Structure for #name {
            fn relation() -> &'static str {
                #relation
            }

            fn primary_key() -> &'static [&'static str] {
                &[
                    #(#primary_key, )*
                ]
            }

            fn columns() -> &'static [&'static str] {
                &[
                    #(#columns, )*
                ]
            }
        }
    }
}

fn model_impl(ast: &syn::DeriveInput, params: &crate::params::Entity, elephantry: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let name = match &params.model {
        Some(name) => name,
        None => return proc_macro2::TokenStream::new(),
    };

    let structure = match &params.structure {
        Some(structure) => structure,
        None => panic!("Model requires structure"),
    };

    let entity = &ast.ident;

    quote::quote! {
        struct #name<'a> {
            connection: &'a #elephantry::Connection,
        }

        #[automatically_derived]
        impl<'a> #elephantry::Model<'a> for Model<'a> {
            type Entity = #entity;
            type Structure = #structure;

            fn new(connection: &'a #elephantry::Connection) -> Self {
                Self {
                    connection,
                }
            }
        }
    }
}

fn is_option(ty: &syn::Type) -> bool {
    let typepath = match ty {
        syn::Type::Path(typepath) => typepath,
        _ => unimplemented!(),
    };

    typepath.path.leading_colon.is_none()
        && typepath.path.segments.len() == 1
        && typepath
            .path
            .segments
            .iter()
            .next()
            .map(|x| x.ident.to_string())
            == Some("Option".to_string())
}
