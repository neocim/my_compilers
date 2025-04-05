mod fxset;
#[cfg(test)]
mod tests;

use std::cell::RefCell;

use fxset::FxIndexSet;

type SymbolRegistry = RefCell<SymbolRegistryInterner>;

thread_local! {
    static SYMBOL_REGISTRY: SymbolRegistry = RefCell::new(SymbolRegistryInterner::new())
}

impl Symbol {
    pub fn new(idx: u16) -> Self {
        Self(idx)
    }

    pub fn intern(to_intern: String) -> Symbol {
        with_mut_symbol_registry(|symregi| symregi.intern(to_intern))
    }
}

/// Interner for all strings that we are meet during compilation. Here i use `String` instead of
/// `&'static str` because I'm too lazy to write some kind of arena where these variables
/// will live the entire program or look for some other way to make these strings with
/// `'static` lifetime.
struct SymbolRegistryInterner {
    symbols: FxIndexSet<String>,
}

impl SymbolRegistryInterner {
    fn new() -> Self {
        Self {
            symbols: FxIndexSet::default(),
        }
    }

    fn intern(&mut self, to_intern: String) -> Symbol {
        if let Some(idx) = self.symbols.get_index_of(&to_intern) {
            return Symbol::new(idx as u16);
        }

        let (idx, _) = self.symbols.insert_full(to_intern);

        Symbol::new(idx as u16)
    }
}

fn with_mut_symbol_registry<F, R>(func: F) -> R
where
    F: FnOnce(&mut SymbolRegistryInterner) -> R,
{
    SYMBOL_REGISTRY.with_borrow_mut(func)
}

/// I use `u16` instead of `u32` because I have a small compiler and I
/// doubt that this table will ever be able to fill up with 65535 (max u16 size) symbols.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(u16);
