// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! AST node representation.

use axiom_core::{Position, Range};
use serde::{Deserialize, Serialize};

/// A simplified AST node for serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNode {
    /// Node kind (e.g., "function_definition", "if_statement").
    pub kind: String,
    /// Range in source.
    pub range: Range,
    /// Whether this node represents an error.
    pub is_error: bool,
    /// Whether this node is named (vs anonymous).
    pub is_named: bool,
    /// Child nodes.
    pub children: Vec<AstNode>,
    /// Node text (for leaf nodes).
    pub text: Option<String>,
}

impl AstNode {
    /// Create a new AST node from a tree-sitter node.
    pub fn from_ts_node(node: tree_sitter::Node, source: &[u8]) -> Self {
        let start = node.start_position();
        let end = node.end_position();

        let range = Range::new(
            Position::new(start.row as u32, start.column as u32),
            Position::new(end.row as u32, end.column as u32),
        );

        let text = if node.child_count() == 0 {
            node.utf8_text(source).ok().map(|s| s.to_string())
        } else {
            None
        };

        let children: Vec<AstNode> = (0..node.child_count())
            .filter_map(|i| node.child(i))
            .filter(|c| c.is_named())
            .map(|c| AstNode::from_ts_node(c, source))
            .collect();

        Self {
            kind: node.kind().to_string(),
            range,
            is_error: node.is_error(),
            is_named: node.is_named(),
            children,
            text,
        }
    }

    /// Get all error nodes in this tree.
    pub fn errors(&self) -> Vec<&AstNode> {
        let mut errors = Vec::new();
        self.collect_errors(&mut errors);
        errors
    }

    fn collect_errors<'a>(&'a self, errors: &mut Vec<&'a AstNode>) {
        if self.is_error {
            errors.push(self);
        }
        for child in &self.children {
            child.collect_errors(errors);
        }
    }

    /// Find nodes of a specific kind.
    pub fn find_by_kind(&self, kind: &str) -> Vec<&AstNode> {
        let mut results = Vec::new();
        self.collect_by_kind(kind, &mut results);
        results
    }

    fn collect_by_kind<'a>(&'a self, kind: &str, results: &mut Vec<&'a AstNode>) {
        if self.kind == kind {
            results.push(self);
        }
        for child in &self.children {
            child.collect_by_kind(kind, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_node_errors() {
        let mut node = AstNode {
            kind: "root".to_string(),
            range: Range::new(Position::new(0, 0), Position::new(10, 0)),
            is_error: false,
            is_named: true,
            children: vec![AstNode {
                kind: "ERROR".to_string(),
                range: Range::new(Position::new(1, 0), Position::new(1, 10)),
                is_error: true,
                is_named: true,
                children: vec![],
                text: Some("bad code".to_string()),
            }],
            text: None,
        };

        let errors = node.errors();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, "ERROR");
    }
}
