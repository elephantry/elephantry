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

    let public = if is_public(&ast) {
        quote::quote!(pub)
    } else {
        proc_macro2::TokenStream::new()
    };

    let entity = entity_impl(ast, &elephantry);
    let structure = structure_impl(ast, &params, &elephantry, &public);
    let model = model_impl(ast, &params, &elephantry, &public);

    let gen = quote::quote! {
        #entity
        #structure
        #model
    };

    gen.into()
}

fn entity_impl(
    ast: &syn::DeriveInput,
    elephantry: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let from_body = fields.iter().map(|field| {
        let field_params = crate::params::Field::from_ast(field);

        let name = &field.ident;
        let column = field_params
            .column
            .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());
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
        let column = field_params
            .column
            .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());
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

fn structure_impl(
    ast: &syn::DeriveInput,
    params: &crate::params::Entity,
    elephantry: &proc_macro2::TokenStream,
    public: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let name = match &params.structure {
        Some(name) => name,
        None => return proc_macro2::TokenStream::new(),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let relation = params
        .relation
        .clone()
        .unwrap_or_else(|| ast.ident.to_string().to_lowercase());

    let primary_key = fields
        .iter()
        .filter(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params.pk
        })
        .map(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params
                .column
                .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
        });

    let columns = fields
        .iter()
        .filter(|field| {
            let field_params = crate::params::Field::from_ast(field);

            !field_params.r#virtual
        })
        .map(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params
                .column
                .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
        });

    quote::quote! {
        #public struct #name;

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

fn model_impl(
    ast: &syn::DeriveInput,
    params: &crate::params::Entity,
    elephantry: &proc_macro2::TokenStream,
    public: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let name = match &params.model {
        Some(name) => name,
        None => return proc_macro2::TokenStream::new(),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let structure = match &params.structure {
        Some(structure) => structure,
        None => panic!("Model requires structure"),
    };

    let entity = &ast.ident;

    let projection = fields
        .iter()
        .filter(|field| {
            let field_params = crate::params::Field::from_ast(field);

            field_params.projection.is_some()
        })
        .map(|field| {
            let field_params = crate::params::Field::from_ast(field);
            let name = &field.ident;
            let projection = field_params.projection.unwrap();

            quote::quote!(
                .add_field(stringify!(#name), #projection)
            )
        })
        .collect::<Vec<_>>();

    let create_projection = if projection.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        quote::quote! {
            fn create_projection() -> #elephantry::Projection {
                Self::default_projection()
                    #(#projection)*
            }
        }
    };

    quote::quote! {
        #public struct #name<'a> {
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

            #create_projection
        }
    }
}

fn is_public(ast: &syn::DeriveInput) -> bool {
    matches!(ast.vis, syn::Visibility::Public(_))
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
