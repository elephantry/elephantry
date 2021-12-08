pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let params = crate::params::Entity::from_ast(ast)?;

    let elephantry = if params.internal {
        quote::quote! {
            crate
        }
    } else {
        quote::quote! {
            elephantry
        }
    };

    let public = if is_public(ast) {
        quote::quote!(pub)
    } else {
        proc_macro2::TokenStream::new()
    };

    let entity = entity_impl(ast, &elephantry)?;
    let structure = structure_impl(ast, &params, &elephantry, &public)?;
    let model = model_impl(ast, &params, &elephantry, &public)?;

    let gen = quote::quote! {
        #entity
        #structure
        #model
    };

    Ok(gen)
}

fn entity_impl(
    ast: &syn::DeriveInput,
    elephantry: &proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => return crate::error(ast, "this derive macro only works on structs"),
    };

    if matches!(fields, syn::Fields::Unnamed(_)) {
        return crate::error(
            ast,
            "this derive macro only works on structs with named field",
        );
    }

    let mut from_body = Vec::new();
    let mut get_body = Vec::new();

    for field in fields {
        let field_params = crate::params::Field::from_ast(field)?;

        let name = &field.ident;
        let column = field_params
            .column
            .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());
        let ty = &field.ty;
        crate::check_type(ty)?;

        let from_part = if field_params.default {
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
        };

        from_body.push(from_part);

        let get_part = if is_option(ty) {
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
        };

        get_body.push(get_part);
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let entity = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #elephantry::Entity for #name #ty_generics #where_clause
        {
            fn from(tuple: &#elephantry::Tuple<'_>) -> Self
            {
                Self {
                    #(#from_body, )*
                }
            }

            fn get(&self, field: &str) -> ::std::option::Option<&dyn #elephantry::ToSql> {
                match field {
                    #(#get_body, )*
                    _ => None,
                }
            }
        }
    };

    Ok(entity)
}

fn structure_impl(
    ast: &syn::DeriveInput,
    params: &crate::params::Entity,
    elephantry: &proc_macro2::TokenStream,
    public: &proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = match &params.structure {
        Some(name) => name,
        None => return Ok(proc_macro2::TokenStream::new()),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unreachable!(),
    };

    let relation = params
        .relation
        .clone()
        .unwrap_or_else(|| ast.ident.to_string().to_lowercase());

    let mut primary_key = Vec::new();
    let mut columns = Vec::new();

    for field in fields {
        let field_params = crate::params::Field::from_ast(field)?;
        let column = field_params
            .column
            .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());

        if field_params.pk {
            primary_key.push(column.clone());
        }

        if !field_params.r#virtual {
            columns.push(column);
        }
    }

    let structure_impl = quote::quote! {
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
    };

    Ok(structure_impl)
}

fn model_impl(
    ast: &syn::DeriveInput,
    params: &crate::params::Entity,
    elephantry: &proc_macro2::TokenStream,
    public: &proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = match &params.model {
        Some(name) => name,
        None => return Ok(proc_macro2::TokenStream::new()),
    };

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unreachable!(),
    };

    let structure = match &params.structure {
        Some(structure) => structure,
        None => return crate::error(ast, "Model requires structure attribute"),
    };

    let entity = &ast.ident;

    let mut projection_body = Vec::new();

    for field in fields {
        let field_params = crate::params::Field::from_ast(field)?;

        if let Some(projection) = field_params.projection {
            let name = &field.ident;
            let projection_part = quote::quote!(
                .add_field(stringify!(#name), #projection)
            );

            projection_body.push(projection_part);
        }
    }

    let create_projection = if projection_body.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        quote::quote! {
            fn create_projection() -> #elephantry::Projection {
                Self::default_projection()
                    #(#projection_body)*
            }
        }
    };

    let model_impl = quote::quote! {
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
    };

    Ok(model_impl)
}

fn is_public(ast: &syn::DeriveInput) -> bool {
    matches!(ast.vis, syn::Visibility::Public(_))
}

fn is_option(ty: &syn::Type) -> bool {
    let typepath = match ty {
        syn::Type::Path(typepath) => typepath,
        _ => return false,
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
