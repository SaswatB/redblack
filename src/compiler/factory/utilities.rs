use oxc_ast::{AstKind, GetChildren};

use crate::compiler::types::OuterExpressionKinds;

// region: 622
/** @internal */
/** @internal */
pub fn isOuterExpression(node: &AstKind, kinds: OuterExpressionKinds) -> bool {
    match node {
        AstKind::ParenthesizedExpression(_) => {
            // !rb skipping isJSDocTypeAssertion check
            // if (kinds.contains(OuterExpressionKinds::ExcludeJSDocTypeAssertion) && isJSDocTypeAssertion(node)) {
            //     return false;
            // }
            kinds.contains(OuterExpressionKinds::Parentheses)
        }
        AstKind::TSTypeAssertion(_) | AstKind::TSAsExpression(_) | AstKind::TSSatisfiesExpression(_) => kinds.contains(OuterExpressionKinds::TypeAssertions),
        AstKind::ExpressionWithTypeArguments(_) => kinds.contains(OuterExpressionKinds::ExpressionsWithTypeArguments),
        AstKind::TSNonNullExpression(_) => kinds.contains(OuterExpressionKinds::NonNullAssertions),
        // !rb skipping PartiallyEmittedExpression case
        // AstKind::PartiallyEmittedExpression(_) => {
        //     kinds.contains(OuterExpressionKinds::PartiallyEmittedExpressions)
        // }
        _ => false,
    }
}

/** @internal */
pub fn skipOuterExpressions<'a>(node: AstKind<'a>, kinds: OuterExpressionKinds) -> AstKind<'a> {
    let mut node = node;
    while isOuterExpression(&node, kinds) {
        match node {
            AstKind::ParenthesizedExpression(parenthesized) => {
                node = parenthesized.expression.to_ast_kind();
            }
            AstKind::TSTypeAssertion(type_assertion) => {
                node = type_assertion.expression.to_ast_kind();
            }
            AstKind::TSSatisfiesExpression(satisfies) => {
                node = satisfies.expression.to_ast_kind();
            }
            AstKind::TSAsExpression(as_expression) => {
                node = as_expression.expression.to_ast_kind();
            }
            AstKind::TSNonNullExpression(non_null) => {
                node = non_null.expression.to_ast_kind();
            }
            _ => {
                panic!("Unsupported outer expression kind: {:?}", node);
            }
        }
    }
    node
}
// endregion: 658
