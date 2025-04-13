use super::SymbolRegistryInterner;
use crate::symbol::Symbol;

#[test]
fn test_symbol_registry_interner() {
    let mut sri = SymbolRegistryInterner::new();

    assert_eq!(Symbol(0), sri.intern("while".to_string()));
    assert_eq!(Symbol(1), sri.intern("if".to_string()));
    assert_eq!(Symbol(0), sri.intern("while".to_string()));
    assert_eq!(Symbol(2), sri.intern("int".to_string()));
    assert_eq!(Symbol(1), sri.intern("if".to_string()));
}
