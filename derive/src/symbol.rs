pub(crate) struct Symbol(&'static str);

pub(crate) const DEFAULT: Symbol = Symbol("default");
pub(crate) const ELEPHANTRY: Symbol = Symbol("elephantry");
pub(crate) const INTERNAL: Symbol = Symbol("internal");

impl PartialEq<Symbol> for syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}
