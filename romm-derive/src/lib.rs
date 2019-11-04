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

            quote::quote_spanned! {field.span() => #name: {
                let (t, content) = data.get(stringify!(#name))
                    .expect(&format!("Unable to find '{}' field", stringify!(#name)));
                #ty::from_sql(t, content)
                    .expect(&format!("Unable to convert '{}' field of type '{}' from SQL", stringify!(#name), stringify!(#ty)))
            }}
        });

    let name = &ast.ident;

    let gen = quote::quote! {
        impl romm::Entity for #name
        {
            fn from(data: &std::collections::HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self
            {
                use postgres::types::FromSql;

                Self {
                    #(#body, )*
                }
            }
        }
    };

    gen.into()
}
