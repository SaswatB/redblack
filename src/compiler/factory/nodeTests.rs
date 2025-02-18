use oxc_ast::{
    ast::{UnaryExpression, UnaryOperator},
    AstKind,
};

use crate::compiler::types::Identifier;

// region: 234
pub fn isNumericLiteral(node: &AstKind) -> bool { matches!(node, AstKind::NumericLiteral(_)) }
// endregion: 238

// region: 315
// Identifiers

pub fn isIdentifier(node: &AstKind) -> bool { Identifier::from_ast_kind(node).is_some() }

pub fn isPrivateIdentifier(node: &AstKind) -> bool { matches!(node, AstKind::PrivateIdentifier(_)) }
// endregion: 325

// region: 434
pub fn isClassStaticBlockDeclaration(node: Option<&AstKind>) -> bool { matches!(node, Some(AstKind::StaticBlock(_))) }
// endregion: 436

// region: 583
pub fn isPropertyAccessExpression(node: &AstKind) -> bool { matches!(node, AstKind::PropertyAccessExpression(_)) }
// endregion: 587

// region: 623
pub fn isTypeOfExpression<'a>(node: &AstKind<'a>) -> Option<&'a UnaryExpression<'a>> {
    let AstKind::UnaryExpression(unary) = node else { return None };
    if unary.operator == UnaryOperator::Typeof {
        Some(unary)
    } else {
        None
    }
}
// endregion: 627
