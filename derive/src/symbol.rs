#[derive(Clone, Copy)]
pub(crate) struct Symbol(&'static str);

pub(crate) const COLUMN: Symbol = Symbol("column");
pub(crate) const DEFAULT: Symbol = Symbol("default");
pub(crate) const ELEPHANTRY: Symbol = Symbol("elephantry");
pub(crate) const MODEL: Symbol = Symbol("model");
pub(crate) const PK: Symbol = Symbol("pk");
pub(crate) const RELATION: Symbol = Symbol("relation");
pub(crate) const STRUCTURE: Symbol = Symbol("structure");
pub(crate) const VIRTUAL: Symbol = Symbol("virtual");

impl PartialEq<Symbol> for syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl PartialEq<Symbol> for &syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
