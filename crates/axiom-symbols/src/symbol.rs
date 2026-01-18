// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Symbol types.

use axiom_core::Location;
use serde::{Deserialize, Serialize};

/// Kind of symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    /// Function definition.
    Function,
    /// Variable declaration.
    Variable,
    /// Type definition (struct, class, enum, typedef).
    Type,
    /// Macro definition.
    Macro,
    /// Constant.
    Constant,
    /// Field/member of a struct or class.
    Field,
    /// Enum variant.
    EnumVariant,
    /// Parameter.
    Parameter,
}

impl SymbolKind {
    /// Get display order for deterministic sorting.
    /// Types first, then functions, then variables, etc.
    pub fn sort_order(&self) -> u8 {
        match self {
            SymbolKind::Type => 0,
            SymbolKind::Function => 1,
            SymbolKind::Macro => 2,
            SymbolKind::Constant => 3,
            SymbolKind::EnumVariant => 4,
            SymbolKind::Variable => 5,
            SymbolKind::Field => 6,
            SymbolKind::Parameter => 7,
        }
    }
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Function => write!(f, "function"),
            SymbolKind::Variable => write!(f, "variable"),
            SymbolKind::Type => write!(f, "type"),
            SymbolKind::Macro => write!(f, "macro"),
            SymbolKind::Constant => write!(f, "constant"),
            SymbolKind::Field => write!(f, "field"),
            SymbolKind::EnumVariant => write!(f, "enum"),
            SymbolKind::Parameter => write!(f, "parameter"),
        }
    }
}

/// A symbol in the codebase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    /// Symbol name.
    pub name: String,
    /// Kind of symbol.
    pub kind: SymbolKind,
    /// Location where defined.
    pub location: Location,
    /// Scope path (e.g., ["file.c", "main", "inner_block"]).
    pub scope: Vec<String>,
    /// Optional type signature.
    pub signature: Option<String>,
}

impl Symbol {
    /// Create a new symbol.
    pub fn new(name: String, kind: SymbolKind, location: Location) -> Self {
        Self {
            name,
            kind,
            location,
            scope: Vec::new(),
            signature: None,
        }
    }

    /// Add scope path.
    pub fn with_scope(mut self, scope: Vec<String>) -> Self {
        self.scope = scope;
        self
    }

    /// Add signature.
    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Symbol {
    /// Deterministic ordering: by kind, then by name alphabetically.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind
            .sort_order()
            .cmp(&other.kind.sort_order())
            .then_with(|| self.name.cmp(&other.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_core::{Position, Range};
    use std::path::PathBuf;

    fn test_location() -> Location {
        Location::new(
            PathBuf::from("test.c"),
            Range::new(Position::new(0, 0), Position::new(0, 10)),
        )
    }

    #[test]
    fn test_symbol_ordering() {
        let mut symbols = vec![
            Symbol::new("zeta".to_string(), SymbolKind::Variable, test_location()),
            Symbol::new("alpha".to_string(), SymbolKind::Function, test_location()),
            Symbol::new("MyType".to_string(), SymbolKind::Type, test_location()),
            Symbol::new("beta".to_string(), SymbolKind::Function, test_location()),
        ];

        symbols.sort();

        // Types first, then functions (alphabetically), then variables
        assert_eq!(symbols[0].name, "MyType");
        assert_eq!(symbols[1].name, "alpha");
        assert_eq!(symbols[2].name, "beta");
        assert_eq!(symbols[3].name, "zeta");
    }

    #[test]
    fn test_deterministic_ordering() {
        // Run multiple times to ensure determinism
        for _ in 0..10 {
            let mut symbols1 = vec![
                Symbol::new("c".to_string(), SymbolKind::Function, test_location()),
                Symbol::new("a".to_string(), SymbolKind::Function, test_location()),
                Symbol::new("b".to_string(), SymbolKind::Function, test_location()),
            ];

            let mut symbols2 = symbols1.clone();

            symbols1.sort();
            symbols2.sort();

            assert_eq!(symbols1, symbols2);
            assert_eq!(symbols1[0].name, "a");
            assert_eq!(symbols1[1].name, "b");
            assert_eq!(symbols1[2].name, "c");
        }
    }
}
