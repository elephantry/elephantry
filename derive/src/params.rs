#[derive(Clone, Default, Debug)]
pub(crate) struct Container {
    pub internal: bool,
}

impl Container {
    pub fn from_ast(ast: &syn::DeriveInput) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&ast.attrs)? {
            match &item {
                // Parse #[elephantry(internal)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::INTERNAL => {
                    param.internal = true;
                }
                syn::NestedMeta::Meta(_) => (),
                syn::NestedMeta::Lit(lit) => {
                    return crate::error(lit, "Unexpected literal in elephantry field attribute");
                }
            }
        }

        Ok(param)
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Entity {
    pub internal: bool,
    pub model: Option<proc_macro2::TokenStream>,
    pub relation: Option<String>,
    pub structure: Option<proc_macro2::TokenStream>,
}

impl Entity {
    pub fn from_ast(ast: &syn::DeriveInput) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&ast.attrs)? {
            match &item {
                // Parse #[elephantry(internal)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::INTERNAL => {
                    param.internal = true;
                }
                // Parse #[elephantry(model = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::MODEL =>
                {
                    let model = get_lit(crate::symbol::MODEL, &m.lit)?;
                    param.model = Some(model);
                }
                // Parse #[elephantry(relation = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::RELATION =>
                {
                    let relation = get_lit_str(crate::symbol::STRUCTURE, &m.lit)?;
                    param.relation = Some(relation);
                }
                // Parse #[elephantry(structure = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::STRUCTURE =>
                {
                    let structure = get_lit(crate::symbol::STRUCTURE, &m.lit)?;
                    param.structure = Some(structure);
                }
                syn::NestedMeta::Meta(meta) => {
                    return crate::error(meta.path(), "Unknow elephantry container attribute");
                }
                syn::NestedMeta::Lit(lit) => {
                    return crate::error(lit, "Unexpected literal in elephantry field attribute");
                }
            }
        }

        Ok(param)
    }
}

fn get_lit(
    attr_name: crate::symbol::Symbol,
    lit: &syn::Lit,
) -> syn::Result<proc_macro2::TokenStream> {
    let lit = get_lit_str(attr_name, lit)?;
    syn::parse_str(&lit)
}

fn get_lit_str(attr_name: crate::symbol::Symbol, lit: &syn::Lit) -> syn::Result<String> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit.value())
    } else {
        crate::error(
            lit,
            &format!(
                "expected elephantry {attr_name} attribute to be a string: `{attr_name} = \"...\"`"
            ),
        )
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Field {
    pub column: Option<String>,
    pub default: bool,
    pub pk: bool,
    pub projection: Option<String>,
    pub r#virtual: bool,
}

impl Field {
    pub fn from_ast(field: &syn::Field) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&field.attrs)? {
            match &item {
                // Parse #[elephantry(default)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::DEFAULT => {
                    param.default = true;
                }
                // Parse #[elephantry(pk)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::PK => {
                    param.pk = true;
                }
                // Parse #[elephantry(column = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::COLUMN =>
                {
                    let column = get_lit_str(crate::symbol::COLUMN, &m.lit)?;
                    param.column = Some(column);
                }
                // Parse #[elephantry(virtual)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::VIRTUAL => {
                    param.r#virtual = true;
                }
                // Parse #[elephantry(virtual = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::VIRTUAL =>
                {
                    let projection = get_lit_str(crate::symbol::VIRTUAL, &m.lit)?;
                    param.r#virtual = true;
                    param.projection = Some(projection);
                }
                syn::NestedMeta::Meta(meta) => {
                    return crate::error(meta.path(), "Unknow elephantry field attribute");
                }
                syn::NestedMeta::Lit(lit) => {
                    return crate::error(lit, "Unexpected literal in elephantry field attribute");
                }
            }
        }

        Ok(param)
    }
}

fn flat_map(attrs: &[syn::Attribute]) -> syn::Result<Vec<syn::NestedMeta>> {
    let mut items = Vec::new();

    for attr in attrs {
        items.append(&mut meta_items(attr)?);
    }

    Ok(items)
}

fn meta_items(attr: &syn::Attribute) -> syn::Result<Vec<syn::NestedMeta>> {
    if attr.path != crate::symbol::ELEPHANTRY {
        return Ok(Vec::new());
    }

    match attr.parse_meta()? {
        syn::Meta::List(meta) => Ok(meta.nested.into_iter().collect()),
        _ => crate::error(attr, "expected #[elephantry(...)]"),
    }
}
