pub(crate) fn expand(params: crate::params::Test, input: syn::ItemFn) -> proc_macro2::TokenStream {
    let elephantry = crate::elephantry();

    let name = &input.sig.ident;
    let args = &input.sig.inputs;
    let attrs = &input.attrs;
    let fixtures = &params.fixtures;

    quote::quote! {
        #(#attrs)*
        #[::core::prelude::v1::test]
        fn #name() -> #elephantry::Result {
            #input

            let f: fn(#args) -> _ = #name;
            #elephantry::testing::Fn::run(f, &[#(#fixtures, )*])
        }
    }
}
