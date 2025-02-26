use std::fmt::Display;

use oxc_ast::{
    ast::{PrivateIdentifier, TSQualifiedName},
    AstKind,
};

use crate::define_subset_enum;

use super::types::{DeclarationName, DiagnosticMessageChain, EntityNameExpression, IsContainer, PropertyNameLiteral};

define_subset_enum!(IsContainerOrEntityNameExpression from AstKind {
    Sub(IsContainer),
    Sub(EntityNameExpression),
});

#[derive(Debug, Clone)]
pub enum StringOrNumber {
    String(String),
    Number(u64),
}
impl From<String> for StringOrNumber {
    fn from(s: String) -> Self { StringOrNumber::String(s) }
}
impl From<u64> for StringOrNumber {
    fn from(n: u64) -> Self { StringOrNumber::Number(n) }
}
pub fn strings_to_string_or_numbers(strings: &[String]) -> Vec<StringOrNumber> { strings.iter().map(|s| StringOrNumber::String(s.clone())).collect() }

impl Display for StringOrNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrNumber::String(s) => write!(f, "{}", s),
            StringOrNumber::Number(n) => write!(f, "{}", n),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringOrDiagnosticMessageChain {
    String(String),
    DiagnosticMessageChain(DiagnosticMessageChain),
}

define_subset_enum!(DeclarationNameOrQualifiedName from AstKind {
    Sub(DeclarationName),
    TSQualifiedName,
});

pub trait StrName {
    fn str_name(&self) -> &str;
}
pub trait StrText {
    fn str_text(&self) -> &str;
}

define_subset_enum!(PropertyNameLiteralOrPrivateIdentifier from AstKind {
    Sub(PropertyNameLiteral),
    PrivateIdentifier,
});
