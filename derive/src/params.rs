#[derive(Clone, Default, Debug)]
pub(crate) struct Container {
    pub internal: bool,
}

impl syn::parse::Parse for Container {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let internal = match content.parse::<syn::Ident>() {
            Ok(internal) => internal == "internal",
            Err(_) => false,
        };

        Ok(Container { internal })
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Field {
    pub default: bool,
}

impl syn::parse::Parse for Field {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let default = match content.parse::<syn::Ident>() {
            Ok(default) => default == "default",
            Err(_) => false,
        };

        Ok(Field { default })
    }
}
