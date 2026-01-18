// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Symbol index for autocomplete.

use crate::{Symbol, SymbolKind};
use axiom_core::{Location, Position, Range};
use axiom_parser::AstNode;
use std::collections::HashMap;
use std::path::PathBuf;

/// Symbol index for a codebase.
#[derive(Debug, Default)]
pub struct SymbolIndex {
    /// All symbols, keyed by file path.
    symbols: HashMap<PathBuf, Vec<Symbol>>,
}

impl SymbolIndex {
    /// Create a new empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Index symbols from an AST.
    pub fn index_file(&mut self, path: PathBuf, ast: &AstNode) {
        let mut symbols = Vec::new();
        self.extract_symbols(ast, &path, &mut Vec::new(), &mut symbols);
        symbols.sort(); // Deterministic ordering
        self.symbols.insert(path, symbols);
    }

    /// Remove a file from the index.
    pub fn remove_file(&mut self, path: &PathBuf) {
        self.symbols.remove(path);
    }

    /// Get completions for a prefix.
    ///
    /// Returns symbols matching the prefix, sorted deterministically:
    /// by kind (types first), then alphabetically within kind.
    pub fn complete(&self, prefix: &str) -> Vec<&Symbol> {
        let prefix_lower = prefix.to_lowercase();

        let mut matches: Vec<&Symbol> = self
            .symbols
            .values()
            .flatten()
            .filter(|s| s.name.to_lowercase().starts_with(&prefix_lower))
            .collect();

        // Deterministic sort: by kind order, then alphabetically
        matches.sort_by(|a, b| a.cmp(b));

        matches
    }

    /// Get all symbols in a file.
    pub fn symbols_in_file(&self, path: &PathBuf) -> Option<&[Symbol]> {
        self.symbols.get(path).map(|v| v.as_slice())
    }

    /// Get total symbol count.
    pub fn len(&self) -> usize {
        self.symbols.values().map(|v| v.len()).sum()
    }

    /// Check if index is empty.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Extract symbols from an AST node.
    fn extract_symbols(
        &self,
        node: &AstNode,
        path: &PathBuf,
        scope: &mut Vec<String>,
        symbols: &mut Vec<Symbol>,
    ) {
        match node.kind.as_str() {
            "function_definition" | "function_declarator" => {
                if let Some(name) = self.find_identifier(node) {
                    let location = Location::new(path.clone(), node.range);
                    let symbol = Symbol::new(name, SymbolKind::Function, location)
                        .with_scope(scope.clone());
                    symbols.push(symbol);
                }
            }
            "declaration" => {
                if let Some(name) = self.find_identifier(node) {
                    let kind = if self.is_type_declaration(node) {
                        SymbolKind::Type
                    } else {
                        SymbolKind::Variable
                    };
                    let location = Location::new(path.clone(), node.range);
                    let symbol = Symbol::new(name, kind, location).with_scope(scope.clone());
                    symbols.push(symbol);
                }
            }
            "struct_specifier" | "class_specifier" | "enum_specifier" => {
                if let Some(name) = self.find_type_name(node) {
                    let location = Location::new(path.clone(), node.range);
                    let symbol = Symbol::new(name.clone(), SymbolKind::Type, location)
                        .with_scope(scope.clone());
                    symbols.push(symbol);

                    // Enter scope for nested symbols
                    scope.push(name);
                    for child in &node.children {
                        self.extract_symbols(child, path, scope, symbols);
                    }
                    scope.pop();
                    return; // Don't process children again
                }
            }
            "preproc_def" | "preproc_function_def" => {
                if let Some(name) = self.find_macro_name(node) {
                    let location = Location::new(path.clone(), node.range);
                    let symbol = Symbol::new(name, SymbolKind::Macro, location)
                        .with_scope(scope.clone());
                    symbols.push(symbol);
                }
            }
            "field_declaration" => {
                if let Some(name) = self.find_identifier(node) {
                    let location = Location::new(path.clone(), node.range);
                    let symbol = Symbol::new(name, SymbolKind::Field, location)
                        .with_scope(scope.clone());
                    symbols.push(symbol);
                }
            }
            "enumerator" => {
                if let Some(name) = self.find_identifier(node) {
                    let location = Location::new(path.clone(), node.range);
                    let symbol = Symbol::new(name, SymbolKind::EnumVariant, location)
                        .with_scope(scope.clone());
                    symbols.push(symbol);
                }
            }
            _ => {}
        }

        // Recurse into children
        for child in &node.children {
            self.extract_symbols(child, path, scope, symbols);
        }
    }

    /// Find identifier in a node.
    fn find_identifier(&self, node: &AstNode) -> Option<String> {
        if node.kind == "identifier" {
            return node.text.clone();
        }
        for child in &node.children {
            if let Some(id) = self.find_identifier(child) {
                return Some(id);
            }
        }
        None
    }

    /// Find type name in a struct/class/enum node.
    fn find_type_name(&self, node: &AstNode) -> Option<String> {
        for child in &node.children {
            if child.kind == "type_identifier" || child.kind == "identifier" {
                return child.text.clone();
            }
        }
        None
    }

    /// Find macro name.
    fn find_macro_name(&self, node: &AstNode) -> Option<String> {
        for child in &node.children {
            if child.kind == "identifier" {
                return child.text.clone();
            }
        }
        None
    }

    /// Check if a declaration is a type declaration.
    fn is_type_declaration(&self, node: &AstNode) -> bool {
        for child in &node.children {
            if child.kind == "type_definition" {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_determinism() {
        let mut index = SymbolIndex::new();

        // Create test symbols
        let loc = Location::new(
            PathBuf::from("test.c"),
            Range::new(Position::new(0, 0), Position::new(0, 10)),
        );

        let symbols = vec![
            Symbol::new("func_c".to_string(), SymbolKind::Function, loc.clone()),
            Symbol::new("func_a".to_string(), SymbolKind::Function, loc.clone()),
            Symbol::new("func_b".to_string(), SymbolKind::Function, loc.clone()),
            Symbol::new("TypeA".to_string(), SymbolKind::Type, loc.clone()),
        ];

        index.symbols.insert(PathBuf::from("test.c"), symbols);

        // Run multiple times to verify determinism
        for _ in 0..10 {
            let completions = index.complete("func_");
            assert_eq!(completions.len(), 3);
            // Alphabetical order within kind
            assert_eq!(completions[0].name, "func_a");
            assert_eq!(completions[1].name, "func_b");
            assert_eq!(completions[2].name, "func_c");
        }
    }

    #[test]
    fn test_completion_kind_ordering() {
        let mut index = SymbolIndex::new();

        let loc = Location::new(
            PathBuf::from("test.c"),
            Range::new(Position::new(0, 0), Position::new(0, 10)),
        );

        let symbols = vec![
            Symbol::new("abc_var".to_string(), SymbolKind::Variable, loc.clone()),
            Symbol::new("abc_func".to_string(), SymbolKind::Function, loc.clone()),
            Symbol::new("AbcType".to_string(), SymbolKind::Type, loc.clone()),
        ];

        index.symbols.insert(PathBuf::from("test.c"), symbols);

        let completions = index.complete("abc");
        assert_eq!(completions.len(), 3);
        // Types first, then functions, then variables
        assert_eq!(completions[0].kind, SymbolKind::Type);
        assert_eq!(completions[1].kind, SymbolKind::Function);
        assert_eq!(completions[2].kind, SymbolKind::Variable);
    }
}
