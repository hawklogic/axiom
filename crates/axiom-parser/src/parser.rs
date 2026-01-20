// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Parser wrapper for tree-sitter.

use crate::AstNode;
use std::path::Path;

/// Supported languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    C,
    Cpp,
}

impl Language {
    /// Detect language from file extension.
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?;
        match ext.to_lowercase().as_str() {
            "c" | "h" => Some(Language::C),
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => Some(Language::Cpp),
            _ => None,
        }
    }
}

/// Parser error.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unsupported language")]
    UnsupportedLanguage,

    #[error("Parse failed")]
    ParseFailed,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Parser for C/C++ files.
pub struct Parser {
    c_parser: tree_sitter::Parser,
    cpp_parser: tree_sitter::Parser,
}

impl Parser {
    /// Create a new parser.
    pub fn new() -> Result<Self, ParseError> {
        let mut c_parser = tree_sitter::Parser::new();
        c_parser
            .set_language(tree_sitter_c::language())
            .map_err(|_| ParseError::UnsupportedLanguage)?;

        let mut cpp_parser = tree_sitter::Parser::new();
        cpp_parser
            .set_language(tree_sitter_cpp::language())
            .map_err(|_| ParseError::UnsupportedLanguage)?;

        Ok(Self {
            c_parser,
            cpp_parser,
        })
    }

    /// Parse source code.
    pub fn parse(&mut self, source: &str, language: Language) -> Result<AstNode, ParseError> {
        let parser = match language {
            Language::C => &mut self.c_parser,
            Language::Cpp => &mut self.cpp_parser,
        };

        let tree = parser.parse(source, None).ok_or(ParseError::ParseFailed)?;
        let root = tree.root_node();
        let source_bytes = source.as_bytes();

        Ok(AstNode::from_ts_node(root, source_bytes))
    }

    /// Parse a file.
    pub fn parse_file(&mut self, path: &Path) -> Result<AstNode, ParseError> {
        let language = Language::from_path(path).ok_or(ParseError::UnsupportedLanguage)?;
        let source = std::fs::read_to_string(path)?;
        self.parse(&source, language)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new().expect("Failed to create parser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_path(Path::new("main.c")), Some(Language::C));
        assert_eq!(
            Language::from_path(Path::new("main.cpp")),
            Some(Language::Cpp)
        );
        assert_eq!(Language::from_path(Path::new("main.h")), Some(Language::C));
        assert_eq!(
            Language::from_path(Path::new("main.hpp")),
            Some(Language::Cpp)
        );
        assert_eq!(Language::from_path(Path::new("main.rs")), None);
    }

    #[test]
    fn test_parse_c() {
        let mut parser = Parser::new().unwrap();
        let source = r#"
int main(void) {
    return 0;
}
        "#;

        let ast = parser.parse(source, Language::C).unwrap();
        assert_eq!(ast.kind, "translation_unit");
        assert!(!ast.is_error);

        let functions = ast.find_by_kind("function_definition");
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn test_parse_cpp() {
        let mut parser = Parser::new().unwrap();
        let source = r#"
class Foo {
public:
    void bar() {}
};
        "#;

        let ast = parser.parse(source, Language::Cpp).unwrap();
        assert_eq!(ast.kind, "translation_unit");

        let classes = ast.find_by_kind("class_specifier");
        assert_eq!(classes.len(), 1);
    }

    #[test]
    fn test_parse_with_error() {
        let mut parser = Parser::new().unwrap();
        let source = r#"
int main( {
    return;
}
        "#;

        let ast = parser.parse(source, Language::C).unwrap();
        let errors = ast.errors();
        assert!(!errors.is_empty());
    }
}
