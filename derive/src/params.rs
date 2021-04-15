#[derive(Clone, Default, Debug)]
pub(crate) struct Container {
    pub internal: bool,
}

impl Container {
    pub fn from_ast(ast: &syn::DeriveInput) -> Self {
        let mut param = Self::default();

        for item in ast.attrs.iter().flat_map(|attr| meta_items(attr)).flatten() {
            match &item {
                // Parse #[elephantry(internal)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::INTERNAL => {
                    param.internal = true;
                }
                syn::NestedMeta::Meta(meta) => {
                    let ident = meta.path().get_ident().unwrap();
                    panic!("Unknow elephantry container attribute: '{}'", ident);
                }
                syn::NestedMeta::Lit(_) => {
                    panic!("Unexpected literal in elephantry container attribute");
                }
            }
        }

        param
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Field {
    pub default: bool,
}

impl Field {
    pub fn from_ast(field: &syn::Field) -> Self {
        let mut param = Self::default();

        for item in field.attrs.iter().flat_map(|attr| meta_items(attr)).flatten() {
            match &item {
                // Parse #[elephantry(default)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::DEFAULT => {
                    param.default = true;
                }
                syn::NestedMeta::Meta(meta) => {
                    let ident = meta.path().get_ident().unwrap();
                    panic!("Unknow elephantry field attribute: '{}'", ident);
                }
                syn::NestedMeta::Lit(_) => {
                    panic!("Unexpected literal in elephantry field attribute");
                }
            }
        }

        param
    }
}

fn meta_items(attr: &syn::Attribute) -> Result<Vec<syn::NestedMeta>, ()> {
    if attr.path != crate::symbol::ELEPHANTRY {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(syn::Meta::List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(_) => {
            panic!("expected #[elephantry(...)]");
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}
