use oxc_ast::{
    ast::{CallExpression, ElementAccessExpression},
    AstKind,
};

use crate::define_subset_enum;

use super::{
    factory::nodeTests::isElementAccessExpression,
    rb_extra::AstKindExt,
    types::{AstKindExpression, HasLocals, PropertyAccessExpression, SignatureDeclaration, TextSpan},
    utilities::{isInJSFile, isLiteralLikeElementAccess},
};

// region: 430
pub fn createTextSpan(start: u32, length: u32) -> TextSpan { TextSpan { start, length } }

pub fn createTextSpanFromBounds(start: u32, end: u32) -> TextSpan { createTextSpan(start, end - start) }

// endregion: 444

// region: 825
/** Add an extra underscore to identifiers that start with two underscores to avoid issues with magic names like '__proto__' */
pub fn escapeLeadingUnderscores(identifier: &str) -> String {
    if identifier.len() >= 2 && identifier.as_bytes()[0] == b'_' && identifier.as_bytes()[1] == b'_' {
        format!("_{}", identifier)
    } else {
        identifier.to_string()
    }
}
// endregion: 830

// region: 1187
/** Gets the JSDoc type tag for the node if present and valid */
pub fn getJSDocTypeTag<'a>(node: &'a AstKind) -> Option<()> {
    // ! rb skipping jsdoc
    // // We should have already issued an error if there were multiple type jsdocs, so just use the first one.
    // let tag = getFirstJSDocTag(node, isJSDocTypeTag);
    // if let Some(tag) = tag {
    //     if let Some(type_expr) = &tag.type_expression {
    //         if type_expr.type_.is_some() {
    //             return Some(tag);
    //         }
    //     }
    // }
    None
}
// endregion: 1197

// region: 1364
define_subset_enum!(isOptionalChainResult from AstKind {
    Sub(PropertyAccessExpression),
    ElementAccessExpression,
    CallExpression,
});
pub fn isOptionalChain<'a>(node: AstKind<'a>) -> Option<isOptionalChainResult<'a>> {
    // ! rb typescript also covers NonNullExpression, but I don't see how that's valid
    match node {
        // PropertyAccessExpression
        AstKind::StaticMemberExpression(n) if n.optional => Some(isOptionalChainResult::PropertyAccessExpression(PropertyAccessExpression::StaticMemberExpression(n))),
        AstKind::PrivateFieldExpression(n) if n.optional => Some(isOptionalChainResult::PropertyAccessExpression(PropertyAccessExpression::PrivateFieldExpression(n))),
        // end PropertyAccessExpression
        AstKind::ElementAccessExpression(n) if n.optional => Some(isOptionalChainResult::ElementAccessExpression(n)),
        AstKind::CallExpression(n) if n.optional => Some(isOptionalChainResult::CallExpression(n)),
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
        AstKind::StaticMemberExpression(_) |
        AstKind::PrivateFieldExpression(_) |
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
        AstKind::PrivateIdentifier(_) | // technically this is only an Expression if it's in a `#field in expr` BinaryExpression
        // end Identifier
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

// region: 2182
/** @internal */
/** @internal */
pub fn canHaveSymbol(node: &AstKind) -> bool {
    // NOTE: This should cover all possible declarations except MissingDeclaration and SemicolonClassElement
    //       since they aren't actually declarations and can't have a symbol.
    matches!(
        node,
        AstKind::Function(_) | // ArrowFunction, FunctionDeclaration, FunctionExpression
        // BinaryExpression
        AstKind::GeneralBinaryExpression(_) | 
        AstKind::AssignmentExpression(_) | 
        AstKind::LogicalExpression(_) | 
        AstKind::PrivateInExpression(_) | 
        AstKind::SequenceExpression(_) | 
        // end BinaryExpression
        // BindingElement
        AstKind::BindingProperty(_) |
        AstKind::ArrayPatternElement(_) |
        AstKind::BindingRestElement(_) |
        // end BindingElement
        AstKind::CallExpression(_) |
        AstKind::TSCallSignatureDeclaration(_) | // CallSignature
        AstKind::Class(_) | // ClassDeclaration, ClassExpression
        AstKind::StaticBlock(_) | // ClassStaticBlockDeclaration
        AstKind::TSConstructorType(_) | // ConstructorType
        AstKind::TSConstructSignatureDeclaration(_) | // ConstructSignature
        AstKind::ElementAccessExpression(_) |
        AstKind::TSEnumDeclaration(_) | // EnumDeclaration
        AstKind::TSEnumMember(_) | // EnumMember
        AstKind::ExportDefaultDeclaration(_) | // ExportAssignment
        AstKind::ExportNamedDeclaration(_) | // ExportDeclaration
        AstKind::ExportSpecifier(_) |
        AstKind::TSFunctionType(_) | // FunctionType
        AstKind::MethodDefinition(_) | // Constructor, GetAccessor, SetAccessor
        AstKind::IdentifierName(_) | AstKind::IdentifierReference(_) | AstKind::BindingIdentifier(_) | // Identifier
        // ImportClause
        AstKind::ImportSpecifier(_) |
        AstKind::ImportDefaultSpecifier(_) |
        AstKind::ImportNamespaceSpecifier(_) | // NamespaceImport
        // end ImportClause
        AstKind::TSImportEqualsDeclaration(_) | // ImportEqualsDeclaration
        AstKind::TSIndexSignature(_) | // IndexSignature
        AstKind::TSInterfaceDeclaration(_) | // InterfaceDeclaration
        // !rb skipping JSDoc
        // case SyntaxKind.JSDocCallbackTag:
        // case SyntaxKind.JSDocEnumTag:
        // case SyntaxKind.JSDocFunctionType:
        // case SyntaxKind.JSDocParameterTag:
        // case SyntaxKind.JSDocPropertyTag:
        // case SyntaxKind.JSDocSignature:
        // case SyntaxKind.JSDocTypedefTag:
        // case SyntaxKind.JSDocTypeLiteral:
        AstKind::JSXAttribute(_) |
        AstKind::JSXSpreadAttribute(_) |
        AstKind::TSMappedType(_) | // MappedType
        AstKind::TSModuleDeclaration(_) | // ModuleDeclaration
        AstKind::TSNamedTupleMember(_) | // NamedTupleMember
        // AstKind::TSNamespaceExport(_) | // NamespaceExport
        AstKind::TSNamespaceExportDeclaration(_) | // NamespaceExportDeclaration
        AstKind::NewExpression(_) |
        AstKind::TemplateLiteral(_) | // NoSubstitutionTemplateLiteral
        AstKind::NumericLiteral(_) |
        AstKind::ObjectExpression(_) | // ObjectLiteralExpression
        AstKind::FormalParameter(_) | // Parameter
        // PropertyAccessExpression
        AstKind::StaticMemberExpression(_) |
        AstKind::PrivateFieldExpression(_) |
        // end PropertyAccessExpression
        AstKind::PropertyDefinition(_) | // PropertyAssignment
        AstKind::TSPropertySignature(_) | // PropertySignature
        AstKind::ObjectProperty(_) | // ShorthandPropertyAssignment
        AstKind::SourceFile(_) |
        AstKind::SpreadElement(_) | // SpreadAssignment
        AstKind::StringLiteral(_) |
        AstKind::TSTypeAliasDeclaration(_) | // TypeAliasDeclaration
        AstKind::TSTypeLiteral(_) | // TypeLiteral
        AstKind::TSTypeParameter(_) | // TypeParameter
        AstKind::VariableDeclarator(_) // VariableDeclaration
    )
}
// endregion: 2257

// region: 2257
/** @internal */
pub fn canHaveLocals(node: &AstKind) -> bool { HasLocals::from_ast_kind(node).is_some() }
// endregion: 2296

// region: 2593
pub fn isStringLiteralLike(node: &AstKind) -> bool {
    match node {
        AstKind::StringLiteral(_) => true,
        AstKind::TemplateLiteral(n) => n.is_no_substitution_template(),
        _ => false,
    }
}
// endregion: 2597

// region: 4132
define_subset_enum!(isSpecialPropertyDeclarationParam from AstKind {
    Sub(PropertyAccessExpression),
    ElementAccessExpression
});
/** @internal */
pub fn isSpecialPropertyDeclaration(expr: isSpecialPropertyDeclarationParam) -> bool {
    isInJSFile(&expr.to_ast_kind()) && AstKindExpression::from_ast_kind(&expr.to_ast_kind().parent().unwrap()).is_some() && (!isElementAccessExpression(&expr.to_ast_kind()) || isLiteralLikeElementAccess(&expr.to_ast_kind())) && getJSDocTypeTag(&expr.to_ast_kind()).is_some()
}
// endregion: 4140
