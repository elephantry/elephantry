#![warn(rust_2018_idioms)]

mod composite;
mod entity;
mod r#enum;

#[derive(Clone, Debug)]
struct Params {
    internal: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            internal: false,
        }
    }
}

impl syn::parse::Parse for Params {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let internal = match content.parse::<syn::Ident>() {
            Ok(internal) => internal == "internal",
            Err(_) => false,
        };

        Ok(Params {
            internal,
        })
    }
}

#[derive(Clone, Debug)]
struct FieldParams {
    default: bool,
}

impl Default for FieldParams {
    fn default() -> Self {
        Self {
            default: false,
        }
    }
}

impl syn::parse::Parse for FieldParams {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let default = match content.parse::<syn::Ident>() {
            Ok(default) => default == "default",
            Err(_) => false,
        };

        Ok(FieldParams {
            default,
        })
    }
}

/**
 * Impl [`Composite`] trait.
 *
 * [`Composite`]: trait.Composite.html
 */
#[proc_macro_derive(Composite, attributes(composite))]
pub fn composite_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    composite::impl_macro(&ast)
}

/**
 * Impl [`Entity`] trait.
 *
 * [`Entity`]: trait.Entity.html
 */
#[proc_macro_derive(Entity, attributes(entity, elephantry))]
pub fn entity_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    entity::impl_macro(&ast)
}

/**
 * Impl [`Enum`] trait.
 *
 * [`Enum`]: trait.Enum.html
 */
#[proc_macro_derive(Enum, attributes(r#enum))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    r#enum::impl_macro(&ast)
}

pub(crate) fn check_type(ty: &syn::Type) {
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
        if !features.contains(feature)
            && ty == &syn::parse_str(feature_ty).unwrap()
        {
            panic!(
                "Enable '{}' feature to use the type `{}` in this entity",
                feature, feature_ty
            );
        }
    }
}
