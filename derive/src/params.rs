#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) struct Container {
    #[darling(default)]
    pub internal: bool,
}

impl Container {
    pub fn elephantry(&self) -> proc_macro2::TokenStream {
        if self.internal {
            quote::quote! {
                crate
            }
        } else {
            quote::quote! {
                elephantry
            }
        }
    }
}

#[derive(Clone, Debug, darling::FromDeriveInput)]
#[darling(attributes(elephantry), supports(struct_any))]
pub(crate) struct Composite {
    #[darling(flatten)]
    container: Container,
    #[darling(rename = "model")]
    _model: Option<syn::Ident>,
    #[darling(rename = "relation")]
    _relation: Option<String>,
    #[darling(rename = "structure")]
    _structure: Option<syn::Ident>,
}

impl std::ops::Deref for Composite {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

#[derive(Clone, Debug, darling::FromDeriveInput)]
#[darling(attributes(elephantry), supports(struct_named))]
pub(crate) struct Entity {
    #[darling(flatten)]
    container: Container,
    pub model: Option<syn::Ident>,
    pub relation: Option<String>,
    pub structure: Option<syn::Ident>,
}

impl std::ops::Deref for Entity {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

#[derive(Clone, Debug, darling::FromDeriveInput)]
#[darling(attributes(elephantry), supports(enum_unit))]
pub(crate) struct Enum {
    #[darling(flatten)]
    container: Container,
}

impl std::ops::Deref for Enum {
    type Target = Container;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

#[derive(Clone, Debug, darling::FromField)]
#[darling(attributes(elephantry))]
pub(crate) struct Field {
    pub column: Option<String>,
    #[darling(default)]
    pub default: bool,
    #[darling(default)]
    pub pk: bool,
    #[darling(default)]
    pub r#virtual: Option<darling::util::Override<String>>,
}
