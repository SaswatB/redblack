// region: 1653
// Functions

use oxc_ast::AstKind;

use super::types::SignatureDeclaration;

pub fn isFunctionLike(node: Option<&AstKind>) -> bool {
    if let Some(node) = node {
        return SignatureDeclaration::from_ast_kind(node).is_some();
    }
    false
}
// endregion: 1657
