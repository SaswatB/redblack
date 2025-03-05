use oxc_ast::{
    ast::{AwaitExpression, MethodDefinitionKind, UnaryExpression, UnaryOperator, UpdateExpression},
    AstKind,
};

use crate::compiler::types::{BindingElement, Identifier, PropertyAccessExpression};

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

pub fn isConstructorDeclaration(node: &AstKind) -> bool { matches!(node, AstKind::MethodDefinition(method_definition) if method_definition.kind == MethodDefinitionKind::Constructor) }
// endregion: 441

// region: 569
pub fn isBindingElement(node: &AstKind) -> bool { BindingElement::from_ast_kind(node).is_some() }
// endregion: 573

// region: 579
pub fn isObjectLiteralExpression(node: &AstKind) -> bool { matches!(node, AstKind::ObjectExpression(_)) }

pub fn isPropertyAccessExpression(node: &AstKind) -> bool { PropertyAccessExpression::from_ast_kind(node).is_some() }

pub fn isElementAccessExpression(node: &AstKind) -> bool { matches!(node, AstKind::ElementAccessExpression(_)) }

pub fn isCallExpression(node: &AstKind) -> bool { matches!(node, AstKind::CallExpression(_)) }
// endregion: 595

// region: 611
pub fn isFunctionExpression(node: &AstKind) -> bool { matches!(node, AstKind::Function(_)) }

pub fn isArrowFunction(node: &AstKind) -> bool { matches!(node, AstKind::ArrowFunctionExpression(_)) }

pub fn isDeleteExpression<'a>(node: &AstKind<'a>) -> Option<&'a UnaryExpression<'a>> {
    let AstKind::UnaryExpression(unary) = node else { return None };
    if unary.operator == UnaryOperator::Delete {
        Some(unary)
    } else {
        None
    }
}

pub fn isTypeOfExpression<'a>(node: &AstKind<'a>) -> Option<&'a UnaryExpression<'a>> {
    let AstKind::UnaryExpression(unary) = node else { return None };
    if unary.operator == UnaryOperator::Typeof {
        Some(unary)
    } else {
        None
    }
}

pub fn isVoidExpression<'a>(node: &AstKind<'a>) -> Option<&'a UnaryExpression<'a>> {
    let AstKind::UnaryExpression(unary) = node else { return None };
    if unary.operator == UnaryOperator::Void {
        Some(unary)
    } else {
        None
    }
}

pub fn isAwaitExpression<'a>(node: &AstKind<'a>) -> Option<&'a AwaitExpression<'a>> {
    let AstKind::AwaitExpression(a) = node else { return None };
    Some(a)
}

pub fn isPrefixUnaryExpression<'a>(node: &AstKind<'a>) -> bool {
    match node {
        AstKind::UpdateExpression(update) if update.prefix => true,
        AstKind::UnaryExpression(unary) if matches!(unary.operator, UnaryOperator::UnaryPlus | UnaryOperator::UnaryNegation | UnaryOperator::BitwiseNot | UnaryOperator::LogicalNot) => true,
        _ => false,
    }
}

pub fn isPostfixUnaryExpression<'a>(node: &AstKind<'a>) -> Option<&'a UpdateExpression<'a>> {
    let AstKind::UpdateExpression(update) = node else { return None };
    if !update.prefix {
        Some(update)
    } else {
        None
    }
}
// endregion: 643

// region: 663
pub fn isClassExpression(node: &AstKind) -> bool { matches!(node, AstKind::Class(_)) }
// endregion: 667

// region: 819
pub fn isModuleDeclaration(node: &AstKind) -> bool { matches!(node, AstKind::TSModuleDeclaration(_)) }
// endregion: 823

// region: 970
pub fn isJsxNamespacedName(node: &AstKind) -> bool { matches!(node, AstKind::JSXNamespacedName(_)) }
// endregion: 974

// region: 992
// Property assignments

pub fn isPropertyAssignment(node: &AstKind) -> bool { matches!(node, AstKind::ObjectProperty(n) if !n.shorthand) }
// endregion: 998
