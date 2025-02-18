use oxc_ast::{
    ast::{CallExpression, ElementAccessExpression, PrivateFieldExpression, PropertyAccessExpression},
    AstKind,
};

use crate::define_subset_enum;

use super::types::{SignatureDeclaration, TextSpan};

// region: 430
pub fn createTextSpan(start: u32, length: u32) -> TextSpan { TextSpan { start, length } }

pub fn createTextSpanFromBounds(start: u32, end: u32) -> TextSpan { createTextSpan(start, end - start) }

// endregion: 444

// region: 1364
define_subset_enum!(isOptionalChainResult from AstKind {
    PropertyAccessExpression,
    ElementAccessExpression,
    CallExpression,
    PrivateFieldExpression
});
pub fn isOptionalChain<'a>(node: AstKind<'a>) -> Option<isOptionalChainResult<'a>> {
    // ! rb typescript also covers NonNullExpression, but I don't see how that's valid
    match node {
        AstKind::PropertyAccessExpression(n) if n.optional => Some(isOptionalChainResult::PropertyAccessExpression(n)),
        AstKind::ElementAccessExpression(n) if n.optional => Some(isOptionalChainResult::ElementAccessExpression(n)),
        AstKind::CallExpression(n) if n.optional => Some(isOptionalChainResult::CallExpression(n)),
        AstKind::PrivateFieldExpression(n) if n.optional => Some(isOptionalChainResult::PrivateFieldExpression(n)),
        _ => None,
    }
}
// endregion: 1373

// region: 1415
pub fn skipPartiallyEmittedExpressions<'a>(node: &'a AstKind) -> &'a AstKind<'a> {
    // ! rb skipping PartiallyEmittedExpressions
    // skipOuterExpressions(node, OuterExpressionKinds.PartiallyEmittedExpressions);
    node
}
// endregion: 1421

// region: 1653
// Functions

pub fn isFunctionLike(node: Option<&AstKind>) -> bool {
    if let Some(node) = node {
        return SignatureDeclaration::from_ast_kind(node).is_some();
    }
    false
}
// endregion: 1657

// region: 1668
/** @internal */
pub fn isBooleanLiteral(node: &AstKind) -> bool { matches!(node, AstKind::BooleanLiteral(_)) }
// endregion: 1673

// region: 1970
pub fn isLeftHandSideExpression(node: &AstKind) -> bool { isLeftHandSideExpressionKind(skipPartiallyEmittedExpressions(node)) }

pub fn isLeftHandSideExpressionKind(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::PropertyAccessExpression(_) |
        AstKind::ElementAccessExpression(_) |
        AstKind::NewExpression(_) |
        AstKind::CallExpression(_) |
        AstKind::JSXElement(_) |
        // case SyntaxKind.JsxSelfClosingElement:
        AstKind::JSXFragment(_) |
        AstKind::TaggedTemplateExpression(_) |
        AstKind::ArrayExpression(_) |
        AstKind::ParenthesizedExpression(_) |
        AstKind::ObjectExpression(_) |
        AstKind::Class(_) |
        AstKind::Function(_) |
        // Identifier
        AstKind::IdentifierName(_) |
        AstKind::BindingIdentifier(_) |
        AstKind::IdentifierReference(_) |
        // end Identifier
        AstKind::PrivateIdentifier(_) | // technically this is only an Expression if it's in a `#field in expr` BinaryExpression
        AstKind::RegExpLiteral(_) |
        AstKind::NumericLiteral(_) |
        AstKind::BigIntLiteral(_) |
        AstKind::StringLiteral(_) |
        AstKind::TemplateLiteral(_) |
        AstKind::BooleanLiteral(_) |
        AstKind::NullLiteral(_) |
        AstKind::ThisExpression(_) |
        AstKind::Super(_) |
        AstKind::TSNonNullExpression(_) |
        AstKind::TSTypeParameterInstantiation(_) |
        AstKind::MetaProperty(_) |
        AstKind::ImportExpression(_) // technically this is only an Expression if it's in a CallExpression
                                     // case SyntaxKind.MissingDeclaration:
    )
}
// endregion: 2013

// region: 2593
pub fn isStringLiteralLike(node: &AstKind) -> bool {
    match node {
        AstKind::StringLiteral(_) => true,
        AstKind::TemplateLiteral(n) => n.is_no_substitution_template(),
        _ => false,
    }
}
// endregion: 2597
