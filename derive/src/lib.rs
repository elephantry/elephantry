#![warn(warnings)]

mod composite;
mod entity;
mod r#enum;
mod params;

/**
 * Impl [`FromSql`]/[`ToSql`] traits for [composite
 * type](https://www.postgresql.org/docs/current/rowtypes.html).
 *
 * See the `08-composite.rs` example.
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
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    entity::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/**
 * Impl [`FromText`]/[`ToText`] traits for [enum
 * type](https://www.postgresql.org/docs/current/datatype-enum.html).
 *
 * See the `13-enum.rs` example.
 *
 * [`FromText`]: trait.FromText.html
 * [`ToText`]: trait.ToText.html
 */
#[proc_macro_derive(Enum, attributes(elephantry))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    r#enum::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

pub(crate) fn check_type(ty: &syn::Type) -> syn::Result<()> {
    let features = [
        #[cfg(feature = "bit")]
        "bit",
        #[cfg(feature = "chrono")]
        "chrono",
        #[cfg(feature = "date")]
        "date",
        #[cfg(feature = "geo")]
        "geo",
        #[cfg(feature = "ltree")]
        "ltree",
        #[cfg(feature = "jiff")]
        "jiff",
        #[cfg(feature = "json")]
        "json",
        #[cfg(feature = "multirange")]
        "multirange",
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
        ("bit", "elephantry::Bits"),
        ("bit", "u8"),
        ("chrono", "chrono::DateTime"),
        ("chrono", "chrono::NaiveDate"),
        ("chrono", "chrono::NaiveDateTime"),
        ("chrono", "chrono::NaiveTime"),
        ("date", "elephantry::Date"),
        ("date", "elephantry::TimestampTz"),
        ("date", "elephantry::Timestamp"),
        ("date", "elephantry::Interval"),
        ("geo", "elephantry::Box"),
        ("geo", "elephantry::Circle"),
        ("geo", "elephantry::Line"),
        ("geo", "elephantry::Path"),
        ("geo", "elephantry::Point"),
        ("geo", "elephantry::Polygon"),
        ("geo", "elephantry::Segment"),
        ("jiff", "jiff::civil::Date"),
        ("jiff", "jiff::civil::DateTime"),
        ("jiff", "jiff::civil::Time"),
        ("jiff", "jiff::Zoned"),
        ("json", "serde_json::value::Value"),
        ("json", "elephantry::Json"),
        ("ltree", "elephantry::Lquery"),
        ("ltree", "elephantry::Ltree"),
        ("ltree", "elephantry::Ltxtquery"),
        ("multirange", "elephantry::Multirange"),
        ("net", "ipnetwork::IpNetwork"),
        ("net", "elephantry::Cidr"),
        ("net", "macaddr::MacAddr6"),
        ("net", "elephantry::MacAddr"),
        ("net", "macaddr::MacAddr8"),
        ("net", "elephantry::MacAddr8"),
        ("net", "std::net::IpAddr"),
        ("numeric", "bigdecimal::BigDecimal"),
        ("numeric", "elephantry::Numeric"),
        ("time", "elephantry::Time"),
        ("time", "elephantry::TimeTz"),
        ("uuid", "uuid::Uuid"),
        ("uuid", "elephantry::Uuid"),
        ("xml", "xmltree::Element"),
        ("xml", "elephantry::Xml"),
    ];

    for (feature, feature_ty) in &types {
        if !features.contains(feature) && ty == &syn::parse_str(feature_ty).unwrap() {
            return error(
                ty,
                &format!(
                    "Enable '{feature}' feature to use the type `{feature_ty}` in this entity"
                ),
            );
        }
    }

    check_u8_array(ty)
}

#[cfg(not(feature = "bit"))]
fn check_u8_array(ty: &syn::Type) -> syn::Result<()> {
    if let syn::Type::Array(array) = ty
        && array.elem == syn::parse_str("u8")?
    {
        return error(
            ty,
            "Enable 'bit' feature to use the type `[u8]` in this entity",
        );
    }

    Ok(())
}

#[cfg(feature = "bit")]
fn check_u8_array(_: &syn::Type) -> syn::Result<()> {
    Ok(())
}

pub(crate) fn error<R>(ast: &dyn quote::ToTokens, message: &str) -> syn::Result<R> {
    Err(syn::Error::new_spanned(ast, message))
}

pub(crate) fn elephantry() -> proc_macro2::TokenStream {
    match (
        proc_macro_crate::crate_name("elephantry"),
        std::env::var("CARGO_CRATE_NAME").as_deref(),
    ) {
        (Ok(proc_macro_crate::FoundCrate::Itself), Ok("elephantry")) => quote::quote!(crate),
        (Ok(proc_macro_crate::FoundCrate::Name(name)), _) => {
            let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
            quote::quote!(::#ident)
        }
        _ => quote::quote!(::elephantry),
    }
}
