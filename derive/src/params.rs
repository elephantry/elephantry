#[derive(Clone, Default, Debug)]
pub(crate) struct Container {
    pub internal: bool,
}

impl Container {
    pub fn from_ast(ast: &syn::DeriveInput) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&ast.attrs)? {
            // Parse #[elephantry(internal)]
            if item.0 == crate::symbol::INTERNAL {
                param.internal = true;
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
            // Parse #[elephantry(internal)]
            if item.0 == crate::symbol::INTERNAL {
                param.internal = true;
            // Parse #[elephantry(model = "")]
            } else if item.0 == crate::symbol::MODEL {
                if let Some(model) = item.1 {
                    param.model = Some(syn::parse_str(&model)?);
                } else {
                    return crate::error(
                        &item.0,
                        &format!("expected elephantry {attr_name:?} attribute to be a string: `{attr_name:?} = \"...\"`", attr_name = item.0),
                    );
                }
            // Parse #[elephantry(structure = "")]
            } else if item.0 == crate::symbol::STRUCTURE {
                if let Some(structure) = item.1 {
                    param.structure = Some(syn::parse_str(&structure)?);
                } else {
                    return crate::error(&item.0, "expected #[elephantry(structure = \"\")]");
                }
            // Parse #[elephantry(relation = "")]
            } else if item.0 == crate::symbol::RELATION {
                if let Some(relation) = item.1 {
                    param.relation = Some(relation);
                } else {
                    return crate::error(&item.0, "expected #[elephantry(relation = \"\")]");
                }
            } else {
                return crate::error(&item.0, "Unknow elephantry container attribute");
            }
        }

        Ok(param)
    }
}

fn get_lit(meta: &syn::meta::ParseNestedMeta) -> syn::Result<String> {
    let expr: syn::Expr = meta.value()?.parse()?;
    let mut value = &expr;
    while let syn::Expr::Group(e) = value {
        value = &e.expr;
    }

    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit),
        ..
    }) = value
    {
        let suffix = lit.suffix();
        if !suffix.is_empty() {
            return crate::error(
                &meta.path,
                &format!("unexpected suffix `{}` on string literal", suffix),
            );
        }
        Ok(lit.value())
    } else {
        crate::error(
            &meta.path,
            &format!(
                "expected elephantry {attr_name:?} attribute to be a string: `{attr_name:?} = \"...\"`",
                attr_name = meta.path,
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
            // Parse #[elephantry(default)]
            if item.0 == crate::symbol::DEFAULT {
                param.default = true;
            // Parse #[elephantry(pk)]
            } else if item.0 == crate::symbol::PK {
                param.pk = true;
            // Parse #[elephantry(virtual)] and #[elephantry(virtual = "")]
            } else if item.0 == crate::symbol::VIRTUAL {
                param.r#virtual = true;
                param.projection = item.1;
            // Parse #[elephantry(column = "")]
            } else if item.0 == crate::symbol::COLUMN {
                if let Some(column) = item.1 {
                    param.column = Some(column);
                } else {
                    return crate::error(&item.0, "expected #[elephantry(columun = \"\")]");
                }
            } else {
                return crate::error(&item.0, "Unknow elephantry field attribute");
            }
        }

        Ok(param)
    }
}

fn flat_map(attrs: &[syn::Attribute]) -> syn::Result<Vec<(syn::Path, Option<String>)>> {
    let mut items = Vec::new();

    for attr in attrs {
        items.append(&mut meta_items(attr)?);
    }

    Ok(items)
}

fn meta_items(attr: &syn::Attribute) -> syn::Result<Vec<(syn::Path, Option<String>)>> {
    let mut items = Vec::new();

    if attr.path() != crate::symbol::ELEPHANTRY {
        return Ok(items);
    }

    attr.parse_nested_meta(|meta| {
        let lit = if meta.input.peek(syn::Token![=]) {
            Some(get_lit(&meta)?)
        } else {
            None
        };

        items.push((meta.path, lit));

        Ok(())
    })?;

    Ok(items)
}
