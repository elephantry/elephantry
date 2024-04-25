#[derive(Clone, Debug, darling::FromDeriveInput)]
#[darling(attributes(elephantry), supports(struct_named))]
pub(crate) struct Entity {
    pub model: Option<syn::Ident>,
    pub relation: Option<String>,
    pub structure: Option<syn::Ident>,
}

#[derive(Clone, Debug, darling::FromField)]
#[darling(attributes(elephantry))]
pub(crate) struct Field {
    pub column: Option<String>,
    #[darling(default)]
    pub default: bool,
    #[darling(default)]
    pub pk: bool,
    #[darling(default, rename = "virtual")]
    pub r#virtual: Option<darling::util::Override<String>>,
}
