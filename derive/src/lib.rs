#![warn(rust_2018_idioms)]

mod composite;
mod entity;
mod r#enum;
mod params;
mod symbol;

/**
 * Impl [`FromSql`]/[`ToSql`] traits for [composite
 * type](https://www.postgresql.org/docs/current/rowtypes.html).
 *
 * [`FromSql`]: trait.FromSql.html
 * [`ToSql`]: trait.ToSql.html
 */
#[proc_macro_derive(Composite, attributes(elephantry))]
pub fn composite_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    composite::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/**
 * Impl [`Entity`] trait.
 *
 * [`Entity`]: trait.Entity.html
 */
#[proc_macro_derive(Entity, attributes(elephantry))]
pub fn entity_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    entity::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/**
 * Impl [`FromSql`]/[`ToSql`] traits for [enum
 * type](https://www.postgresql.org/docs/current/datatype-enum.html).
 *
 * [`FromSql`]: trait.FromSql.html
 * [`ToSql`]: trait.ToSql.html
 */
#[proc_macro_derive(Enum, attributes(elephantry))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    r#enum::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

pub(crate) fn check_type(ty: &syn::Type) -> syn::Result<()> {
    let features = vec![
        #[cfg(feature = "bit")]
        "bit",
        #[cfg(feature = "date")]
        "date",
        #[cfg(feature = "geo")]
        "geo",
        #[cfg(feature = "json")]
        "json",
        #[cfg(feature = "net")]
        "net",
        #[cfg(feature = "numeric")]
        "numeric",
        #[cfg(feature = "time")]
        "time",
        #[cfg(feature = "uuid")]
        "uuid",
        #[cfg(feature = "xml")]
        "xml",
    ];

    let types = [
        ("bit", "bit_vec::BitVec"),
        ("bit", "u8"),
        ("date", "chrono::DateTime"),
        ("date", "chrono::NaiveDate"),
        ("date", "chrono::NaiveDateTime"),
        ("geo", "elephantry::Box"),
        ("geo", "elephantry::Circle"),
        ("geo", "elephantry::Line"),
        ("geo", "elephantry::Path"),
        ("geo", "elephantry::Point"),
        ("geo", "elephantry::Polygon"),
        ("geo", "elephantry::Segment"),
        ("json", "serde_json::value::Value"),
        ("net", "ipnetwork::IpNetwork"),
        ("net", "macaddr::MacAddr6"),
        ("net", "macaddr::MacAddr8"),
        ("net", "std::net::IpAddr"),
        ("numeric", "bigdecimal::BigDecimal"),
        ("time", "elephantry::Time"),
        ("time", "elephantry::TimeTz"),
        ("uuid", "uuid::Uuid"),
        ("xml", "xmltree::Element"),
    ];

    for (feature, feature_ty) in &types {
        if !features.contains(feature) && ty == &syn::parse_str(feature_ty).unwrap() {
            return error(
                ty,
                &format!(
                    "Enable '{}' feature to use the type `{}` in this entity",
                    feature, feature_ty
                ),
            );
        }
    }

    Ok(())
}

pub(crate) fn error<R>(ast: &dyn quote::ToTokens, message: &str) -> syn::Result<R> {
    Err(syn::Error::new_spanned(ast, message))
}
