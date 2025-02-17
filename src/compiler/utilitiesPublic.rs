use oxc_ast::AstKind;

use super::types::{SignatureDeclaration, TextSpan};

// region: 430
pub fn createTextSpan(start: u32, length: u32) -> TextSpan { TextSpan { start, length } }

pub fn createTextSpanFromBounds(start: u32, end: u32) -> TextSpan { createTextSpan(start, end - start) }

// endregion: 444

// region: 1653
// Functions

pub fn isFunctionLike(node: Option<&AstKind>) -> bool {
    if let Some(node) = node {
        return SignatureDeclaration::from_ast_kind(node).is_some();
    }
    false
}
// endregion: 1657
