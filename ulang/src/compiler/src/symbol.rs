mod fxset;

use std::cell::LazyCell;

use fxset::FxIndexSet;

thread_local! { pub static SYMBOL_REGISTRY: LazyCell<SymbolRegistryInterner> = LazyCell::new(|| SymbolRegistryInterner::new())}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(u16);

struct SymbolRegistryInterner {
    symbols: FxIndexSet<String>,
}

impl SymbolRegistryInterner {
    pub fn new() -> Self {
        Self {
            symbols: FxIndexSet::default(),
        }
    }

    pub fn intern(&mut self) -> Symbol {}
}
