extern crate proc_macro;

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let ast = syn::parse(input)
        .unwrap();

    impl_entity_macro(&ast)
}

fn impl_entity_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream
{
    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => unimplemented!(),
    };

    let body = fields.iter()
        .map(|field| {
            use syn::spanned::Spanned;

            let name = &field.ident;
            let ty = &field.ty;

            let default = if is_option(ty) {
                quote::quote! {
                    None
                }
            }
            else {
                quote::quote! {
                    panic!("Unable to find '{}' field", stringify!(#name));
                }
            };

            quote::quote_spanned! {field.span() => #name: {
                if let Some((t, content)) = data.get(stringify!(#name)) {
                    postgres::types::FromSql::from_sql(t, content)
                        .expect(&format!("Unable to convert '{}' field of type '{}' from SQL", stringify!(#name), stringify!(#ty)))
                }
                else {
                    #default
                }
            }}
        });

    let name = &ast.ident;

    let gen = quote::quote! {
        impl romm::Entity for #name
        {
            fn from(data: &std::collections::HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self
            {
                Self {
                    #(#body, )*
                }
            }
        }
    };

    gen.into()
}

fn is_option(ty: &syn::Type) -> bool
{
    let typepath = match ty {
        syn::Type::Path(typepath) => typepath,
        _ => unimplemented!(),
    };

    typepath.path.leading_colon.is_none()
        && typepath.path.segments.len() == 1
        && typepath.path.segments.iter().next().unwrap().ident == "Option"
}
