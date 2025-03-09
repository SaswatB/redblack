use std::collections::HashMap;
use std::fmt::Debug;

use super::core::startsWith;
use super::factory::nodeTests::*;
use super::factory::utilities::skipOuterExpressions;
use super::factory::utilitiesPublic::canHaveModifiers;
use super::rb_extra::SourceFileExt;
use super::rb_unions::strings_to_string_or_numbers;
use super::rb_unions::DeclarationNameOrQualifiedName;
use super::rb_unions::PropertyNameLiteralOrPrivateIdentifier;
use super::rb_unions::StrName;
use super::rb_unions::StrText;
use super::rb_unions::StringOrDiagnosticMessageChain;
use super::scanner::skipTrivia;
use super::utilitiesPublic::*;
use crate::compiler::checker::getSymbolId;
use crate::compiler::factory::nodeTests::isObjectLiteralExpression;
use crate::compiler::parser::*;
use crate::compiler::path::*;
use crate::compiler::program::*;
use crate::compiler::rb_extra::AstKindExt;
use crate::compiler::rb_unions::EscapedText;
use crate::compiler::types::*;
use crate::compiler::utilitiesPublic::createTextSpan;
use crate::rc_cell;
use oxc_ast::ast::*;
use oxc_ast::GetChildren;
use oxc_ast::{ast::SourceFile, AstKind, Visit};
use oxc_span::GetSpan;
use oxc_span::Span;

// region: 774
/** @internal */
pub fn getFullWidth(node: &AstKind) -> u32 { node.span().end - node.span().start }
// endregion: 779

// region: 957
pub fn getSourceFileOfNode<'a>(node: Option<&AstKind<'a>>) -> Option<&'a SourceFile<'a>> {
    let mut current = *node?;
    loop {
        if let AstKind::SourceFile(source_file) = current {
            return Some(source_file);
        }
        current = current.parent()?;
    }
}
// endregion: 967

// region: 1043
// Returns true if this node is missing from the actual source code. A 'missing' node is different
// from 'undefined/defined'. When a node is undefined (which can happen for optional nodes
// in the tree), it is definitely missing. However, a node may be defined, but still be
// missing.  This happens whenever the parser knows it needs to parse something, but can't
// get anything in the source code that it expects at that location. For example:
//
//          let a: ;
//
// Here, the Type in the Type-Annotation is not-optional (as there is a colon in the source
// code). So the parser will attempt to parse out a type, and will create an actual node.
// However, this node will be 'missing' in the sense that no actual source-code/tokens are
// contained within it.
/** @internal */
pub fn nodeIsMissing(node: Option<&AstKind>) -> bool {
    if node.is_none() {
        return true;
    }

    let node = node.unwrap();
    return node.span().start == node.span().end && node.span().start >= 0; // && node.kind !== SyntaxKind.EndOfFileToken;
}

/** @internal */
pub fn nodeIsPresent(node: Option<&AstKind>) -> bool { !nodeIsMissing(node) }
// endregion: 1069

// region: 1272
/** @internal */
pub fn getSourceTextOfNodeFromSourceFile(source_file: &SourceFile, node: &AstKind, include_trivia: Option<bool>) -> String { getTextOfNodeFromSourceText(source_file.source_text, node, include_trivia) }
// endregion: 1277

// region: 1296
/**
 * Equality checks against a keyword without underscores don't need to bother
 * to turn "__" into "___" or vice versa, since they will never be equal in
 * either case. So we can ignore those cases to improve performance.
 *
 * @internal
 */
pub fn moduleExportNameIsDefault(node: &ModuleExportName) -> bool {
    let name = match node {
        ModuleExportName::StringLiteral(n) => n.raw.map(|s| s.as_str()).unwrap_or(""),
        ModuleExportName::IdentifierName(n) => n.name.as_str(),
        ModuleExportName::IdentifierReference(n) => n.name.as_str(),
    };
    name == InternalSymbolName::Default.as_str()
}

/** @internal */
pub fn getTextOfNodeFromSourceText(source_text: &str, node: &AstKind, include_trivia: Option<bool>) -> String {
    if node.span().start == node.span().end {
        return "".to_string();
    }

    let start = if include_trivia.unwrap_or(false) { node.span().start } else { skipTrivia(source_text, node.span().start, None, None, None) };

    let text = source_text[start as usize..node.span().end as usize].to_string();

    // ! skipping jsdoc
    // if (isJSDocTypeExpressionOrChild(node)) {
    //     // strip space + asterisk at line start
    //     text = text.split(/\r\n|\n|\r/).map(line => line.replace(/^\s*\*/, "").trimStart()).join("\n");
    // }

    text
}

/** @internal */
pub fn getTextOfNode(node: &AstKind, include_trivia: Option<bool>) -> String { getSourceTextOfNodeFromSourceFile(getSourceFileOfNode(Some(node)).unwrap(), node, include_trivia) }
// endregion: 1328

// region: 1926
/** @internal */
/** @internal */
pub fn isAmbientModule(node: &AstKind) -> bool {
    let AstKind::TSModuleDeclaration(n) = node else { return false };
    matches!(n.id.to_ast_kind(), AstKind::StringLiteral(_)) || isGlobalScopeAugmentation(n)
}
// endregion: 1931

// region: 1941
/**
 * An effective module (namespace) declaration is either
 * 1. An actual declaration: namespace X { ... }
 * 2. A Javascript declaration, which is:
 *    An identifier in a nested property access expression: Y in `X.Y.Z = { ... }`
 */
pub fn isEffectiveModuleDeclaration(node: &AstKind) -> bool { isModuleDeclaration(node) || isIdentifier(node) }
// endregion: 1951

// region: 1972
/** @internal */
pub fn isGlobalScopeAugmentation(module: &TSModuleDeclaration) -> bool { module.kind.is_global() }
// endregion: 1977

// region: 2194
// Return display name of an identifier
// Computed property names will just be emitted as "[<expr>]", where <expr> is the source
// text of the expression in the computed property.
/** @internal */
pub fn declarationNameToString(name: Option<DeclarationNameOrQualifiedName>) -> String {
    if name.is_none() || getFullWidth(&name.unwrap().to_ast_kind()) == 0 {
        "(Missing)".into()
    } else {
        getTextOfNode(&name.unwrap().to_ast_kind(), None)
    }
}
// endregion: 2202

// region: 2265
/** @internal */
pub fn createDiagnosticForNode<'a>(node: &AstKind<'a>, message: DiagnosticMessage, args: DiagnosticArguments) -> DiagnosticWithLocation<'a> {
    let source_file = getSourceFileOfNode(Some(node));
    createDiagnosticForNodeInSourceFile(source_file, node, message, args)
}

/** @internal */
pub fn createDiagnosticForNodeArray<'a>(source_file: &'a SourceFile<'a>, nodes: &NodeArray<'a>, message: DiagnosticMessage, args: DiagnosticArguments) -> DiagnosticWithLocation<'a> {
    let start = skipTrivia(&source_file.source_text, nodes.pos, None, None, None);
    createFileDiagnostic(source_file, start, nodes.end - start, message, args)
}

/** @internal */
pub fn createDiagnosticForNodeInSourceFile<'a>(source_file: Option<&'a SourceFile<'a>>, node: &AstKind<'a>, message: DiagnosticMessage, args: DiagnosticArguments) -> DiagnosticWithLocation<'a> {
    let span = getErrorSpanForNode(source_file.unwrap(), node);
    createFileDiagnostic(source_file.unwrap(), span.start, span.length, message, args)
}

/** @internal */
pub fn createDiagnosticForNodeFromMessageChain<'a>(source_file: &'a SourceFile<'a>, node: &AstKind<'a>, message_chain: DiagnosticMessageChain, related_information: Option<Vec<DiagnosticRelatedInformation<'a>>>) -> DiagnosticWithLocation<'a> {
    let span = getErrorSpanForNode(source_file, node);
    createFileDiagnosticFromMessageChain(source_file, span.start, span.length, message_chain, related_information)
}

/** @internal */
pub fn createDiagnosticForNodeArrayFromMessageChain<'a>(source_file: &'a SourceFile<'a>, nodes: &NodeArray<'a>, message_chain: DiagnosticMessageChain, related_information: Option<Vec<DiagnosticRelatedInformation<'a>>>) -> DiagnosticWithLocation<'a> {
    let start = skipTrivia(&source_file.source_text, nodes.pos, None, None, None);
    createFileDiagnosticFromMessageChain(source_file, start, nodes.end - start, message_chain, related_information)
}

/** @internal */
pub fn assertDiagnosticLocation(source_text: &str, start: u32, length: u32) {
    assert!(start >= 0);
    assert!(length >= 0);
    assert!(start <= source_text.len() as u32);
    assert!(start + length <= source_text.len() as u32);
}

/** @internal */
pub fn createFileDiagnosticFromMessageChain<'a>(file: &'a SourceFile<'a>, start: u32, length: u32, message_chain: DiagnosticMessageChain, related_information: Option<Vec<DiagnosticRelatedInformation<'a>>>) -> DiagnosticWithLocation<'a> {
    assertDiagnosticLocation(&file.source_text, start, length);
    let messageChain = message_chain.clone();
    DiagnosticWithLocation {
        file: Some(file),
        start: Some(start),
        length: Some(length),
        code: message_chain.code,
        category: message_chain.category,
        messageText: if message_chain.next.is_some() { StringOrDiagnosticMessageChain::DiagnosticMessageChain(message_chain) } else { StringOrDiagnosticMessageChain::String(message_chain.messageText) },
        relatedInformation: related_information,
        canonicalHead: messageChain.canonicalHead,

        fileName: None,
        reportsUnnecessary: None,
        reportsDeprecated: None,
        source: None,
        skippedOn: None,
    }
}

/** @internal */
pub fn createDiagnosticForFileFromMessageChain<'a>(source_file: &'a SourceFile<'a>, message_chain: DiagnosticMessageChain, related_information: Option<Vec<DiagnosticRelatedInformation<'a>>>) -> DiagnosticWithLocation<'a> {
    DiagnosticWithLocation {
        file: Some(source_file),
        start: Some(0),
        length: Some(0),
        code: message_chain.code,
        category: message_chain.category,
        messageText: if message_chain.next.is_some() { StringOrDiagnosticMessageChain::DiagnosticMessageChain(message_chain) } else { StringOrDiagnosticMessageChain::String(message_chain.messageText) },
        relatedInformation: related_information,

        canonicalHead: None,
        fileName: None,
        reportsUnnecessary: None,
        reportsDeprecated: None,
        source: None,
        skippedOn: None,
    }
}

/** @internal */
pub fn createDiagnosticMessageChainFromDiagnostic(diagnostic: DiagnosticRelatedInformation) -> DiagnosticMessageChain {
    match diagnostic.messageText {
        StringOrDiagnosticMessageChain::String(message_text) => DiagnosticMessageChain {
            code: diagnostic.code,
            category: diagnostic.category,
            messageText: message_text,
            next: None, //diagnostic.next,
            canonicalHead: None,
        },
        StringOrDiagnosticMessageChain::DiagnosticMessageChain(message_chain) => message_chain,
    }
}

/** @internal */
pub fn createDiagnosticForRange<'a>(source_file: &'a SourceFile<'a>, range: Span, message: DiagnosticMessage) -> DiagnosticWithLocation<'a> {
    DiagnosticWithLocation {
        file: Some(source_file),
        start: Some(range.start),
        length: Some(range.end - range.start),
        code: message.code,
        category: message.category,
        messageText: StringOrDiagnosticMessageChain::String(message.message),

        fileName: None,
        reportsUnnecessary: None,
        reportsDeprecated: None,
        source: None,
        relatedInformation: None,
        skippedOn: None,
        canonicalHead: None,
    }
}

/** @internal */
pub fn getCanonicalDiagnostic(message: DiagnosticMessage, args: &[String]) -> CanonicalDiagnostic { CanonicalDiagnostic { code: message.code, messageText: formatMessage(&message, &strings_to_string_or_numbers(args)) } }

/** @internal */
pub fn getSpanOfTokenAtPosition<'a>(_source_file: &'a SourceFile<'a>, pos: u32) -> TextSpan {
    // let scanner = createScanner(source_file.language_version, /*skipTrivia*/ true, source_file.language_variant, &source_file.text, /*onError*/ None, pos);
    // scanner.scan();
    // let start = scanner.getTokenStart();
    // createTextSpanFromBounds(start, scanner.getTokenEnd())
    createTextSpanFromBounds(pos, pos + 1) // todo(RB): figure out scanner
}

// /** @internal */
// pub fn scanTokenAtPosition<'a>(source_file: &'a SourceFile<'a>, pos: u32) -> SyntaxKind {
//     let scanner = createScanner(source_file.language_version, /*skipTrivia*/ true, source_file.language_variant, &source_file.text, /*onError*/ None, pos);
//     scanner.scan();
//     scanner.getToken()
// }

pub fn getErrorSpanForArrowFunction<'a>(_source_file: &'a SourceFile<'a>, node: &ArrowFunctionExpression) -> TextSpan {
    // let pos = skipTrivia(&source_file.text, node.span().start, None, None, None);
    // if let Some(body) = &node.body {
    //     if let AstKind::BlockStatement(_) = body {
    //         let start_line = getLineAndCharacterOfPosition(source_file, body.span().start).line;
    //         let end_line = getLineAndCharacterOfPosition(source_file, body.span().end).line;
    //         if start_line < end_line {
    //             // The arrow function spans multiple lines,
    //             // make the error span be the first line, inclusive.
    //             return createTextSpan(pos, getEndLinePosition(start_line, source_file) - pos + 1);
    //         }
    //     }
    // }
    // createTextSpanFromBounds(pos, node.span().end)
    createTextSpanFromBounds(node.span().start, node.span().end) // todo(RB): figure out scanner
}

/** @internal */
pub fn getErrorSpanForNode<'a>(source_file: &'a SourceFile<'a>, node: &AstKind<'a>) -> TextSpan {
    let mut error_node = Some(*node);
    match node {
        AstKind::SourceFile(_) => {
            let pos = skipTrivia(&source_file.source_text, 0, Some(false), None, None);
            if pos == source_file.source_text.len() as u32 {
                // file is empty - return span for the beginning of the file
                return createTextSpan(0, 0);
            }
            return getSpanOfTokenAtPosition(source_file, pos);
        }
        // This list is a work in progress. Add missing node kinds to improve their error
        // spans.
        AstKind::VariableDeclarator(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        // BindingElement
        AstKind::BindingProperty(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::ArrayPatternElement(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::BindingRestElement(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        // end BindingElement
        AstKind::Class(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::TSInterfaceDeclaration(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::TSModuleDeclaration(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::TSEnumDeclaration(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::TSEnumMember(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::Function(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        // MethodDefinition moved below to handle constructor
        AstKind::TSTypeAliasDeclaration(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::PropertyDefinition(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::TSPropertySignature(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::ImportNamespaceSpecifier(n) => {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
        }
        AstKind::ArrowFunctionExpression(arrow) => {
            return getErrorSpanForArrowFunction(source_file, arrow);
        }
        AstKind::SwitchCase(case_clause) => {
            let start = skipTrivia(&source_file.source_text, case_clause.span.start, None, None, None);
            let end = if case_clause.consequent.len() > 0 { case_clause.consequent[0].span().start } else { case_clause.span.end };
            return createTextSpanFromBounds(start, end);
        }
        AstKind::ReturnStatement(_) | AstKind::YieldExpression(_) => {
            let pos = skipTrivia(&source_file.source_text, node.span().start, None, None, None);
            return getSpanOfTokenAtPosition(source_file, pos);
        }
        AstKind::TSSatisfiesExpression(satisfies) => {
            let pos = skipTrivia(&source_file.source_text, satisfies.expression.span().end, None, None, None);
            return getSpanOfTokenAtPosition(source_file, pos);
        }
        // ! skipping jsdoc
        // case SyntaxKind.JSDocSatisfiesTag: {
        //     const pos = skipTrivia(sourceFile.text, (node as JSDocSatisfiesTag).tagName.pos);
        //     return getSpanOfTokenAtPosition(sourceFile, pos);
        // }
        AstKind::MethodDefinition(n) => {
            // if n.kind == MethodDefinitionKind::Constructor {
            //     let start = skipTrivia(&source_file.source_text, n.span.start, None, None, None);
            //     let scanner = createScanner(source_file.language_version, Some(true), source_file.language_variant, &source_file.source_text, None, Some(start));
            //     let mut token = scanner.scan();
            //     while token != SyntaxKind::ConstructorKeyword && token != SyntaxKind::EndOfFileToken {
            //         token = scanner.scan();
            //     }
            //     let end = scanner.get_token_end();
            //     return createTextSpanFromBounds(start, end); // todo(RB): figure out scanner
            // } else {
            error_node = NamedDeclarationTrait::name(*n).map(|n| n.to_ast_kind());
            // }
        }
        _ => {}
    }

    if error_node.is_none() {
        // If we don't have a better node, then just set the error on the first token of
        // construct.
        return getSpanOfTokenAtPosition(source_file, node.span().start);
    }

    // ! skipping jsdoc
    // debug_assert!(!isJSDoc(error_node.unwrap()));

    let error_node = error_node.unwrap();
    let is_missing = nodeIsMissing(Some(&error_node));
    let pos = if is_missing || matches!(node, AstKind::JSXText(_)) { error_node.span().start } else { skipTrivia(&source_file.source_text, error_node.span().start, None, None, None) };

    // These asserts should all be satisfied for a properly constructed `error_node`.
    if is_missing {
        debug_assert!(pos == error_node.span().start, "This failure could trigger https://github.com/Microsoft/TypeScript/issues/20809");
        debug_assert!(pos == error_node.span().end, "This failure could trigger https://github.com/Microsoft/TypeScript/issues/20809");
    } else {
        debug_assert!(pos >= error_node.span().start, "This failure could trigger https://github.com/Microsoft/TypeScript/issues/20809");
        debug_assert!(pos <= error_node.span().end, "This failure could trigger https://github.com/Microsoft/TypeScript/issues/20809");
    }

    createTextSpanFromBounds(pos, error_node.span().end)
}
// endregion: 2481

// region: 2922
/** @internal */
pub fn isObjectLiteralOrClassExpressionMethodOrAccessor(node: &AstKind) -> bool { matches!(node, AstKind::MethodDefinition(_)) && if let Some(parent) = node.parent() { matches!(parent, AstKind::ObjectExpression(_) | AstKind::Class(_)) } else { false } }
// endregion: 2927

// region: 2988
/** @internal */
pub fn getContainingClass<'a>(node: AstKind<'a>) -> Option<&ClassLikeDeclaration<'a>> {
    let a = findAncestor(node.parent(), |n| isClassLike(n).into());
    if let Some(AstKind::Class(a)) = a {
        Some(a)
    } else {
        None
    }
}
// endregion: 2993

// region: 3608
/** @internal */
pub fn isPartOfTypeQuery(node: &AstKind) -> bool {
    let mut current = node.clone();
    while matches!(current, AstKind::TSQualifiedName(_) | AstKind::BindingIdentifier(_) | AstKind::IdentifierReference(_)) {
        let newParent = current.parent();
        if newParent.is_none() {
            break;
        }
        current = newParent.unwrap();
    }
    matches!(current, AstKind::TSTypeQuery(_))
}
// endregion: 3614

// region: 3646
/** @internal */
pub fn isSourceFileJS(file: &SourceFile) -> bool { file.source_type.is_javascript() }

/** @internal */
// ! rb consider making this faster?
pub fn isInJSFile(node: &AstKind) -> bool { isSourceFileJS(getSourceFileOfNode(Some(node)).unwrap()) }
// endregion: 3656

// region: 3755
/** @internal */
pub fn isAssignmentDeclaration(decl: &AstKind) -> bool { isBinaryExpression(decl) || isAccessExpression(decl) || isIdentifier(decl) || isCallExpression(decl) }
// endregion: 3760

// region: 3919
/** @internal */
pub fn getRightMostAssignedExpression<'a>(node: AstKind<'a>) -> AstKind<'a> {
    let mut node = node;
    while isAssignmentExpression(&node, Some(true)) {
        node = BinaryExpression::from_ast_kind(&node).unwrap().right().to_ast_kind();
    }
    node
}

/** @internal */
pub fn isExportsIdentifier(node: &AstKind) -> bool {
    let Some(identifier) = Identifier::from_ast_kind(node) else {
        return false;
    };
    identifier.str_name() == "exports"
}

/** @internal */
pub fn isModuleIdentifier(node: &AstKind) -> bool {
    let Some(identifier) = Identifier::from_ast_kind(node) else {
        return false;
    };
    identifier.str_name() == "module"
}

/** @internal */
pub fn isModuleExportsAccessExpression(node: &AstKind) -> bool {
    if !isPropertyAccessExpression(node) && !isLiteralLikeElementAccess(node) {
        return false;
    }
    if let Some(access) = AccessExpression::from_ast_kind(node) {
        if !isModuleIdentifier(&access.object().to_ast_kind()) {
            return false;
        }
    }
    getElementOrPropertyAccessName(AccessExpression::from_ast_kind(node).unwrap()) == Some("exports".to_string())
}

/// Given a BinaryExpression, returns SpecialPropertyAssignmentKind for the various kinds of property
/// assignments we treat as special in the binder
/** @internal */
pub fn getAssignmentDeclarationKind(expr: &AstKind) -> AssignmentDeclarationKind {
    let special = getAssignmentDeclarationKindWorker(expr);
    if special == AssignmentDeclarationKind::Property || isInJSFile(expr) {
        special
    } else {
        AssignmentDeclarationKind::None
    }
}

/** @internal */
pub fn isBindableObjectDefinePropertyCall(expr: &CallExpression) -> bool {
    expr.arguments.len() == 3
        && isPropertyAccessExpression(&expr.callee.to_ast_kind())
        && isIdentifier(&PropertyAccessExpression::from_ast_kind(&expr.callee.to_ast_kind()).unwrap().object().to_ast_kind())
        && Identifier::from_ast_kind(&PropertyAccessExpression::from_ast_kind(&expr.callee.to_ast_kind()).unwrap().object().to_ast_kind()).unwrap().str_name() == "Object"
        && PropertyAccessExpression::from_ast_kind(&expr.callee.to_ast_kind()).unwrap().property().str_name() == "defineProperty"
        && isStringOrNumericLiteralLike(&expr.arguments[1].to_ast_kind())
        && isBindableStaticNameExpression(&expr.arguments[0].to_ast_kind(), Some(true))
}
// endregion: 3963

// region: 3970
/**
 * x[0] OR x['a'] OR x[Symbol.y]
 */
pub fn isLiteralLikeElementAccess(node: &AstKind) -> bool {
    let AstKind::ElementAccessExpression(element_access) = node else {
        return false;
    };
    isStringOrNumericLiteralLike(&element_access.argument_expression.to_ast_kind())
}

/**
 * Any series of property and element accesses.
 *
 * @internal
 */
pub fn isBindableStaticAccessExpression(node: &AstKind, excludeThisKeyword: Option<bool>) -> bool {
    if let Some(property_access) = PropertyAccessExpression::from_ast_kind(node) {
        let exclude = excludeThisKeyword.unwrap_or(false);
        !exclude && matches!(property_access.object().to_ast_kind(), AstKind::ThisExpression(_)) || isIdentifier(&property_access.property().to_ast_kind()) && isBindableStaticNameExpression(&property_access.object().to_ast_kind(), Some(true))
    } else {
        isBindableStaticElementAccessExpression(node, excludeThisKeyword)
    }
}

/**
 * Any series of property and element accesses, ending in a literal element access
 *
 * @internal
 */
pub fn isBindableStaticElementAccessExpression(node: &AstKind, excludeThisKeyword: Option<bool>) -> bool {
    let AstKind::ElementAccessExpression(element_access) = node else {
        return false;
    };
    isLiteralLikeElementAccess(node)
        && ((!excludeThisKeyword.unwrap_or(false) && matches!(element_access.object.to_ast_kind(), AstKind::ThisExpression(_))) || isEntityNameExpression(&element_access.object.to_ast_kind()) || isBindableStaticAccessExpression(&element_access.object.to_ast_kind(), Some(true)))
}

/** @internal */
pub fn isBindableStaticNameExpression(node: &AstKind, excludeThisKeyword: Option<bool>) -> bool { isEntityNameExpression(node) || isBindableStaticAccessExpression(node, excludeThisKeyword) }

/** @internal */
pub fn getNameOrArgument<'a>(expr: &'a AccessExpression<'a>) -> AstKind<'a> {
    match expr {
        AccessExpression::PropertyAccessExpression(property_access) => property_access.property().to_ast_kind(),
        AccessExpression::ElementAccessExpression(element_access) => element_access.argument_expression.to_ast_kind(),
    }
}

pub fn getAssignmentDeclarationKindWorker<'a>(expr: &'a AstKind<'a>) -> AssignmentDeclarationKind {
    if let AstKind::CallExpression(call) = expr {
        if !isBindableObjectDefinePropertyCall(call) {
            return AssignmentDeclarationKind::None;
        }
        let entityName = &call.arguments[0];
        if isExportsIdentifier(&entityName.to_ast_kind()) || isModuleExportsAccessExpression(&entityName.to_ast_kind()) {
            return AssignmentDeclarationKind::ObjectDefinePropertyExports;
        }
        if isBindableStaticAccessExpression(&entityName.to_ast_kind(), None) && getElementOrPropertyAccessName(AccessExpression::from_ast_kind(&entityName.to_ast_kind()).unwrap()) == Some("prototype".to_string()) {
            return AssignmentDeclarationKind::ObjectDefinePrototypeProperty;
        }
        return AssignmentDeclarationKind::ObjectDefinePropertyValue;
    }

    if let Some(bin) = BinaryExpression::from_ast_kind(expr) {
        if bin.operator() != BinaryOperator::AssignmentOperator(AssignmentOperator::Assign) || !isAccessExpression(&bin.left().to_ast_kind()) || isVoidZero(&getRightMostAssignedExpression(*expr)) {
            return AssignmentDeclarationKind::None;
        }

        let left = AccessExpression::from_ast_kind(&bin.left().to_ast_kind()).unwrap();
        if isBindableStaticNameExpression(&left.object().to_ast_kind(), Some(true)) && getElementOrPropertyAccessName(left) == Some("prototype".to_string()) && isObjectLiteralExpression(&getInitializerOfBinaryExpression(BinaryExpression::from_ast_kind(expr).unwrap()).to_ast_kind()) {
            // F.prototype = { ... }
            return AssignmentDeclarationKind::Prototype;
        }
        return getAssignmentDeclarationPropertyAccessKind(left);
    }

    panic!("Expected CallExpression or BinaryExpression, got {:?}", expr);
}

pub fn isVoidZero(node: &AstKind) -> bool {
    if let Some(void_expr) = isVoidExpression(node) {
        if let Expression::NumericLiteral(numeric_literal) = &void_expr.argument {
            return numeric_literal.raw.as_ref().unwrap() == "0";
        }
    }
    false
}
/**
 * Does not handle signed numeric names like `a[+0]` - handling those would require handling prefix unary expressions
 * throughout late binding handling as well, which is awkward (but ultimately probably doable if there is demand)
 *
 * @internal
 */
/** @internal */
pub fn getElementOrPropertyAccessArgumentExpressionOrName<'a>(node: AccessExpression<'a>) -> Option<DeclarationName<'a>> {
    match node {
        AccessExpression::PropertyAccessExpression(property_access) => Some(DeclarationName::from_ast_kind(&property_access.property().to_ast_kind()).unwrap()),
        AccessExpression::ElementAccessExpression(element_access) => {
            let expr = element_access.argument_expression.to_ast_kind();
            let arg = skipParentheses(expr, None);
            if isNumericLiteral(&arg) || isStringLiteralLike(&arg) {
                Some(DeclarationName::from_ast_kind(&arg).unwrap())
            } else {
                Some(DeclarationName::ElementAccessExpression(element_access))
            }
        }
    }
}

/** @internal */
pub fn getElementOrPropertyAccessName<'a>(node: AccessExpression<'a>) -> Option<String> {
    let name = getElementOrPropertyAccessArgumentExpressionOrName(node)?.to_ast_kind();
    if let Some(identifier) = Identifier::from_ast_kind(&name) {
        return Some(identifier.str_name().to_string());
    }
    if isStringLiteralLike(&name) {
        let name = StringLiteralLike::from_ast_kind(&name)?.value();
        return Some(escapeLeadingUnderscores(&name));
    }
    if isNumericLiteral(&name) {
        let AstKind::NumericLiteral(numeric) = name else {
            return None;
        };
        return Some(numeric.raw.as_ref().unwrap().to_string());
    }
    None
}

/** @internal */
pub fn getAssignmentDeclarationPropertyAccessKind(lhs: AccessExpression) -> AssignmentDeclarationKind {
    if matches!(lhs.object().to_ast_kind(), AstKind::ThisExpression(_)) {
        return AssignmentDeclarationKind::ThisProperty;
    } else if isModuleExportsAccessExpression(&lhs.to_ast_kind()) {
        // module.exports = expr
        return AssignmentDeclarationKind::ModuleExports;
    } else if isBindableStaticNameExpression(&lhs.object().to_ast_kind(), Some(true)) {
        if isPrototypeAccess(&lhs.object().to_ast_kind()) {
            // F.G....prototype.x = expr
            return AssignmentDeclarationKind::PrototypeProperty;
        }

        let mut next_to_last = lhs;
        while !isIdentifier(&next_to_last.object().to_ast_kind()) {
            next_to_last = AccessExpression::from_ast_kind(&next_to_last.object().to_ast_kind()).unwrap();
        }
        let id: Identifier<'_> = Identifier::from_ast_kind(&next_to_last.object().to_ast_kind()).unwrap();
        if (id.str_name() == "exports" ||
            (id.str_name() == "module" && getElementOrPropertyAccessName(next_to_last) == Some("exports".to_string()))) &&
            // ExportsProperty does not support binding with computed names
            isBindableStaticAccessExpression(&lhs.to_ast_kind(), None)
        {
            // exports.name = expr OR module.exports.name = expr OR exports["name"] = expr ...
            return AssignmentDeclarationKind::ExportsProperty;
        }
        if isBindableStaticNameExpression(&lhs.to_ast_kind(), Some(true)) || (matches!(lhs, AccessExpression::ElementAccessExpression(_)) && isDynamicName(&DeclarationName::from_ast_kind(&lhs.to_ast_kind()).unwrap())) {
            // F.G...x = expr
            return AssignmentDeclarationKind::Property;
        }
    }

    AssignmentDeclarationKind::None
}

/** @internal */
pub fn getInitializerOfBinaryExpression<'a>(expr: BinaryExpression<'a>) -> BinaryExpression<'a> {
    let mut expr = expr;
    while let Some(right) = BinaryExpression::from_ast_kind(&expr.right().to_ast_kind()) {
        expr = right;
    }
    expr
}
// endregion: 4121

// region: 4140
/** @internal */
pub fn setValueDeclaration<'a>(symbol: rc_cell!(Symbol<'a>), node: AstKindDeclaration<'a>) {
    let valueDeclaration = symbol.borrow().valueDeclaration.clone();
    if valueDeclaration.is_none()
        || !((node.to_ast_kind().flags() & NodeFlags::Ambient).0 != 0 && !isInJSFile(&node.to_ast_kind()) && !(valueDeclaration.as_ref().unwrap().to_ast_kind().flags() & NodeFlags::Ambient).0 != 0)
            && (isAssignmentDeclaration(&valueDeclaration.as_ref().unwrap().to_ast_kind()) && !isAssignmentDeclaration(&node.to_ast_kind()))
        || (valueDeclaration.as_ref().unwrap().to_ast_kind().ty() != node.to_ast_kind().ty() && isEffectiveModuleDeclaration(&valueDeclaration.as_ref().unwrap().to_ast_kind()))
    {
        // other kinds of value declarations take precedence over modules and assignment declarations
        symbol.borrow_mut().valueDeclaration = Some(node);
    }
}
// endregion: 4154

// region: 4871
/** @internal */
pub fn skipParentheses<'a>(node: AstKind<'a>, exclude_jsdoc_type_assertions: Option<bool>) -> AstKind<'a> {
    let flags = if exclude_jsdoc_type_assertions.unwrap_or(false) { OuterExpressionKinds::Parentheses | OuterExpressionKinds::ExcludeJSDocTypeAssertion } else { OuterExpressionKinds::Parentheses };
    skipOuterExpressions(node, flags)
}
// endregion: 4882

// region: 5191
/** @internal */
pub fn isStringOrNumericLiteralLike(node: &AstKind) -> bool { isStringLiteralLike(node) || isNumericLiteral(node) }
/** @internal */
pub fn isSignedNumericLiteral(node: &AstKind) -> bool {
    if let AstKind::UnaryExpression(prefix_expr) = node {
        matches!(prefix_expr.operator, UnaryOperator::UnaryPlus | UnaryOperator::UnaryNegation) && isNumericLiteral(&prefix_expr.argument.to_ast_kind())
    } else {
        false
    }
}

/**
 * A declaration has a dynamic name if all of the following are true:
 *   1. The declaration has a computed property name.
 *   2. The computed name is *not* expressed as a StringLiteral.
 *   3. The computed name is *not* expressed as a NumericLiteral.
 *   4. The computed name is *not* expressed as a PlusToken or MinusToken
 *      immediately followed by a NumericLiteral.
 *
 * @internal
 */
pub fn hasDynamicName(declaration: &AstKind) -> bool {
    let name = getNameOfDeclaration(*declaration);
    name.is_some() && isDynamicName(&name.unwrap())
}

/** @internal */
pub fn isDynamicName(name: &DeclarationName) -> bool {
    if !(matches!(name, DeclarationName::PropertyName(PropertyName::ObjectProperty(p)) if p.computed) || matches!(name, DeclarationName::ElementAccessExpression(_))) {
        return false;
    }
    let expr = if let DeclarationName::ElementAccessExpression(expr) = name {
        skipParentheses(expr.argument_expression.to_ast_kind(), None)
    } else {
        let DeclarationName::PropertyName(PropertyName::ObjectProperty(computed)) = name else { panic!("Expected ObjectProperty, got {:?}", name) };
        computed.value.to_ast_kind()
    };
    !isStringOrNumericLiteralLike(&expr) && !isSignedNumericLiteral(&expr)
}
// endregion: 5226

// region: 5256
/** @internal */
pub fn isPropertyNameLiteral(node: &AstKind) -> bool {
    match node {
        // Identifier
        AstKind::IdentifierName(_) |
        AstKind::BindingIdentifier(_) |
        AstKind::IdentifierReference(_) |
        AstKind::JSXIdentifier(_) |
        // end Identifier
        AstKind::StringLiteral(_) |
        AstKind::NoSubstitutionTemplateLiteral(_) |
        AstKind::NumericLiteral(_) => true,
        _ => false,
    }
}

/** @internal */
pub fn getTextOfIdentifierOrLiteral(node: PropertyNameLiteralOrPrivateIdentifier) -> String {
    if isMemberName(&node.to_ast_kind()) {
        idText(MemberName::from_ast_kind(&node.to_ast_kind()).unwrap())
    } else if let AstKind::JSXNamespacedName(jsx_namespaced_name) = node.to_ast_kind() {
        getTextOfJsxNamespacedName(jsx_namespaced_name)
    } else {
        LiteralLikeNode::from_ast_kind(&node.to_ast_kind()).unwrap().str_text().to_string()
    }
}

/** @internal */
pub fn getEscapedTextOfIdentifierOrLiteral(node: &PropertyNameLiteral) -> __String {
    if isMemberName(&node.to_ast_kind()) {
        MemberName::from_ast_kind(&node.to_ast_kind()).unwrap().escaped_text()
    } else if let AstKind::JSXNamespacedName(jsx_namespaced_name) = node.to_ast_kind() {
        getEscapedTextOfJsxNamespacedName(jsx_namespaced_name)
    } else {
        escapeLeadingUnderscores(LiteralLikeNode::from_ast_kind(&node.to_ast_kind()).unwrap().str_text())
    }
}

/** @internal */
pub fn getSymbolNameForPrivateIdentifier(containingClassSymbol: rc_cell!(Symbol), description: __String) -> __String { format!("__#{}@{}", getSymbolId(containingClassSymbol), description) }

/** @internal */
pub fn isKnownSymbol(symbol: &Symbol) -> bool { startsWith(&symbol.escapedName.to_string(), "__@", None) }
// endregion: 5288

// region: 6995
/** @internal */
pub fn hasSyntacticModifier(node: &AstKind, flags: ModifierFlags) -> bool { getSelectedSyntacticModifierFlags(node, flags).0 != 0 }
// endregion: 7000

// region: 7046
/** @internal @knipignore */
pub fn getSelectedSyntacticModifierFlags(node: &AstKind, flags: ModifierFlags) -> ModifierFlags { getSyntacticModifierFlags(node) & flags }
pub fn getModifierFlagsWorker(node: &AstKind, includeJSDoc: bool, alwaysIncludeJSDoc: Option<bool>) -> ModifierFlags {
    // if node.kind() >= SyntaxKind::FirstToken && node.kind() <= SyntaxKind::LastToken {
    //     return ModifierFlags::None;
    // }

    if (node.modifierFlagsCache() & ModifierFlags::HasComputedFlags).0 == 0 {
        node.set_modifierFlagsCache(getSyntacticModifierFlagsNoCache(node) | ModifierFlags::HasComputedFlags);
    }

    // !rb ignoring jsdoc
    // if alwaysIncludeJSDoc.unwrap_or(false) || includeJSDoc && isInJSFile(node) {
    //     if (node.modifierFlagsCache() & ModifierFlags::HasComputedJSDocModifiers).0 == 0 && node.parent().is_some() {
    //         node.set_modifierFlagsCache(node.modifierFlagsCache() | getRawJSDocModifierFlagsNoCache(node) | ModifierFlags::HasComputedJSDocModifiers);
    //     }
    //     return selectEffectiveModifierFlags(node.modifierFlagsCache());
    // }

    return selectSyntacticModifierFlags(node.modifierFlagsCache());
}
// endregion: 7070

// region: 7086
/**
 * Gets the ModifierFlags for syntactic modifiers on the provided node. The modifiers will be cached on the node to improve performance.
 *
 * NOTE: This function does not use `parent` pointers and will not include modifiers from JSDoc.
 *
 * @internal
 */
pub fn getSyntacticModifierFlags(node: &AstKind) -> ModifierFlags { getModifierFlagsWorker(node, false, None) }
// endregion: 7097

// region: 7113
pub fn selectSyntacticModifierFlags(flags: ModifierFlags) -> ModifierFlags { flags & ModifierFlags::SyntacticModifiers }
// endregion: 7117

// region: 7137
/**
 * Gets the ModifierFlags for syntactic modifiers on the provided node. The modifier flags cache on the node is ignored.
 *
 * NOTE: This function does not use `parent` pointers and will not include modifiers from JSDoc.
 *
 * @internal
 * @knipignore
 */
pub fn getSyntacticModifierFlagsNoCache(node: &AstKind) -> ModifierFlags {
    let flags = if canHaveModifiers(node) { modifiersToFlags(&node.modifiers()) } else { ModifierFlags::None };
    if (node.flags() & NodeFlags::NestedNamespace).0 != 0
    // !rb ignoring jsdoc
    //  || (Identifier::from_ast_kind(node).is_some() && (node.flags() & NodeFlags::IdentifierIsInJSDocNamespace).0 != 0)
    {
        flags | ModifierFlags::Export
    } else {
        flags
    }
}

/** @internal */
pub fn modifiersToFlags(modifiers: &[ModifierLike]) -> ModifierFlags {
    let mut flags = ModifierFlags::None;
    for modifier in modifiers {
        flags |= modifierToFlag(modifier);
    }
    return flags;
}

/** @internal */
pub fn modifierToFlag(token: &ModifierLike) -> ModifierFlags {
    match token {
        ModifierLike::Modifier(Modifier::StaticKeyword) => ModifierFlags::Static,
        ModifierLike::Modifier(Modifier::PublicKeyword) => ModifierFlags::Public,
        ModifierLike::Modifier(Modifier::ProtectedKeyword) => ModifierFlags::Protected,
        ModifierLike::Modifier(Modifier::PrivateKeyword) => ModifierFlags::Private,
        ModifierLike::Modifier(Modifier::AbstractKeyword) => ModifierFlags::Abstract,
        ModifierLike::Modifier(Modifier::AccessorKeyword) => ModifierFlags::Accessor,
        ModifierLike::Modifier(Modifier::ExportKeyword) => ModifierFlags::Export,
        ModifierLike::Modifier(Modifier::DeclareKeyword) => ModifierFlags::Ambient,
        ModifierLike::Modifier(Modifier::ConstKeyword) => ModifierFlags::Const,
        ModifierLike::Modifier(Modifier::DefaultKeyword) => ModifierFlags::Default,
        ModifierLike::Modifier(Modifier::AsyncKeyword) => ModifierFlags::Async,
        ModifierLike::Modifier(Modifier::ReadonlyKeyword) => ModifierFlags::Readonly,
        ModifierLike::Modifier(Modifier::OverrideKeyword) => ModifierFlags::Override,
        ModifierLike::Modifier(Modifier::InKeyword) => ModifierFlags::In,
        ModifierLike::Modifier(Modifier::OutKeyword) => ModifierFlags::Out,
        ModifierLike::Decorator(_) => ModifierFlags::Decorator,
    }
}
// endregion: 7203

// region: 7235
/** @internal */
pub fn isAssignmentOperator(op: BinaryOperator) -> bool { matches!(op, BinaryOperator::AssignmentOperator(_)) }
// endregion: 7240

// region: 7271
/** @internal */
pub fn isAssignmentExpression(node: &AstKind, excludeCompoundAssignment: Option<bool>) -> bool {
    if let Some(binary) = BinaryExpression::from_ast_kind(node) {
        let operator = binary.operator();
        let is_valid_operator = if excludeCompoundAssignment.unwrap_or(false) { matches!(operator, BinaryOperator::AssignmentOperator(AssignmentOperator::Assign)) } else { isAssignmentOperator(operator) };
        is_valid_operator && isLeftHandSideExpressionKind(&binary.left().to_ast_kind())
    } else {
        false
    }
}

/** @internal */
pub fn isDestructuringAssignment(node: &AstKind) -> bool {
    if isAssignmentExpression(node, Some(true)) {
        if let Some(binary) = BinaryExpression::from_ast_kind(node) {
            let left = binary.left().to_ast_kind();
            matches!(left, AstKind::ObjectExpression(_) | AstKind::ArrayExpression(_))
        } else {
            false
        }
    } else {
        false
    }
}

/** @internal */
// pub fn isExpressionWithTypeArgumentsInClassExtendsClause(node: &AstKind) -> bool { tryGetClassExtendingExpressionWithTypeArguments(node).is_some() }

/** @internal */
pub fn isEntityNameExpression(node: &AstKind) -> bool { isIdentifier(node) || isPropertyAccessEntityNameExpression(node) }
// endregion: 7305

// region: 7335
/** @internal */
pub fn isPropertyAccessEntityNameExpression(node: &AstKind) -> bool {
    let Some(property_access) = PropertyAccessExpression::from_ast_kind(node) else {
        return false;
    };
    isEntityNameExpression(&property_access.object().to_ast_kind())
}
// endregion: 7340

// region: 7363
/** @internal */
pub fn isPrototypeAccess(node: &AstKind) -> bool { isBindableStaticAccessExpression(node, None) && getElementOrPropertyAccessName(AccessExpression::from_ast_kind(node).unwrap()) == Some("prototype".to_string()) }
// endregion: 7368

// region: 8137
/** @internal */
pub fn isAccessExpression(node: &AstKind) -> bool { AccessExpression::from_ast_kind(node).is_some() }
// endregion: 8142

// region: 8366
/** @internal */
pub fn formatStringFromArgs(text: &str, args: &DiagnosticArguments) -> String {
    let re = regex::Regex::new(r"\{(\d+)\}").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let index: usize = caps[1].parse().unwrap();
        args[index].to_string()
    })
    .into_owned()
}

static mut localizedDiagnosticMessages: Option<HashMap<String, String>> = None;

/** @internal */
pub fn setLocalizedDiagnosticMessages(messages: Option<HashMap<String, String>>) {
    unsafe {
        localizedDiagnosticMessages = messages;
    }
}

/** @internal */
// If the localized messages json is unset, and if given function use it to set the json
pub fn maybeSetLocalizedDiagnosticMessages<F>(getMessages: Option<F>)
where
    F: FnOnce() -> Option<HashMap<String, String>>,
{
    unsafe {
        if localizedDiagnosticMessages.is_none() && getMessages.is_some() {
            localizedDiagnosticMessages = getMessages.unwrap()();
        }
    }
}

/** @internal */
pub fn getLocaleSpecificMessage(message: &DiagnosticMessage) -> String { unsafe { localizedDiagnosticMessages.as_ref().and_then(|messages| messages.get(&message.key)).map(|s| s.to_string()).unwrap_or_else(|| message.message.clone()) } }
// endregion: 8390

// region: 8465
/** @internal */
/** @internal */
pub fn createFileDiagnostic<'a>(file: &'a SourceFile<'a>, start: u32, length: u32, message: DiagnosticMessage, args: DiagnosticArguments) -> DiagnosticWithLocation<'a> {
    assertDiagnosticLocation(&file.source_text, start, length);

    let mut text = getLocaleSpecificMessage(&message);

    if !args.is_empty() {
        text = formatStringFromArgs(&text, &args);
    }

    DiagnosticWithLocation {
        file: Some(file),
        start: Some(start),
        length: Some(length),
        messageText: StringOrDiagnosticMessageChain::String(text),
        category: message.category,
        code: message.code,
        reportsUnnecessary: message.reportsUnnecessary.map(|_| ()),
        reportsDeprecated: message.reportsDeprecated.map(|_| ()),

        fileName: None,
        source: None,
        relatedInformation: None,
        skippedOn: None,
        canonicalHead: None,
    }
}

/** @internal */
pub fn formatMessage(message: &DiagnosticMessage, args: &DiagnosticArguments) -> String {
    let mut text = getLocaleSpecificMessage(message);

    if !args.is_empty() {
        text = formatStringFromArgs(&text, args);
    }

    text
}
// endregion: 8498

// region: 8737
/**
 * This is a somewhat unavoidable full tree walk to locate a JSX tag - `import.meta` requires the same,
 * but we avoid that walk (or parts of it) if at all possible using the `PossiblyContainsImportMeta` node flag.
 * Unfortunately, there's no `NodeFlag` space to do the same for JSX.
 */
// walkTreeForJSXTags
#[derive(Debug, Default)]
struct FindJSX {
    has_jsx: bool,
}

impl<'a> Visit<'a> for FindJSX {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        if kind.is_jsx() {
            self.has_jsx = true;
        }
    }
}

pub fn isFileModuleFromUsingJSXTag(file: &SourceFile) -> bool {
    // Excludes declaration files - they still require an explicit `export {}` or the like
    // for back compat purposes. (not that declaration files should contain JSX tags!)
    if !file.source_type.is_typescript_definition() {
        let mut finder = FindJSX { has_jsx: false };
        finder.visit_source_file(file);
        return finder.has_jsx;
    }
    false
}

/**
* Note that this requires file.impliedNodeFormat be set already; meaning it must be set very early on
* in SourceFile construction.
*/
pub fn isFileForcedToBeModuleByFormat(file: &SourceFile, options: &CompilerOptions) -> bool {
    // Excludes declaration files - they still require an explicit `export {}` or the like
    // for back compat purposes. The only non-declaration files _not_ forced to be a module are `.js` files
    // that aren't esm-mode (meaning not in a `type: module` scope).
    if (getImpliedNodeFormatForEmitWorker(file, options) == ResolutionMode::ESNext || (fileExtensionIsOneOf(&file.filepath().to_string_lossy(), [Extension::Cjs.as_str(), Extension::Cts.as_str(), Extension::Mjs.as_str(), Extension::Mts.as_str()].to_vec())))
        && !file.source_type.is_typescript_definition()
    {
        true
    } else {
        false
    }
}

// getSetExternalModuleIndicator
/** @internal */
pub fn getExternalModuleIndicator(options: &CompilerOptions, file: &SourceFile) -> bool {
    match getEmitModuleDetectionKind(options) {
        ModuleDetectionKind::Force => {
            // All non-declaration files are modules, declaration files still do the usual isFileProbablyExternalModule
            isFileProbablyExternalModule(file) || !file.source_type.is_typescript_definition()
        }
        ModuleDetectionKind::Legacy => {
            // Files are modules if they have imports, exports, or import.meta
            isFileProbablyExternalModule(file)
        }
        ModuleDetectionKind::Auto => {
            // If module is nodenext or node16, all esm format files are modules
            // If jsx is react-jsx or react-jsxdev then jsx tags force module-ness
            // otherwise, the presence of import or export statments (or import.meta) implies module-ness
            let mut is_module = isFileProbablyExternalModule(file);

            if options.jsx.is_some() && (options.jsx.unwrap() == JsxEmit::ReactJSX || options.jsx.unwrap() == JsxEmit::ReactJSXDev) {
                is_module = is_module || isFileModuleFromUsingJSXTag(file);
            }
            is_module = is_module || isFileForcedToBeModuleByFormat(file, options);

            is_module
        }
    }
}
// endregion: 8791

// region: 8917
/** @internal */
pub fn getEmitScriptTarget(compiler_options: &CompilerOptions) -> ScriptTarget {
    let target = if compiler_options.target == Some(ScriptTarget::ES3) { None } else { compiler_options.target };

    target.unwrap_or_else(|| {
        if compiler_options.module == Some(ModuleKind::Node16) {
            ScriptTarget::ES2022
        } else if compiler_options.module == Some(ModuleKind::NodeNext) {
            ScriptTarget::ESNext
        } else {
            ScriptTarget::ES5
        }
    })
}

/** @internal */
pub fn getEmitModuleKind(compiler_options: &CompilerOptions) -> ModuleKind {
    match compiler_options.module {
        Some(module) => module,
        None => {
            if getEmitScriptTarget(compiler_options) >= ScriptTarget::ES2015 {
                ModuleKind::ES2015
            } else {
                ModuleKind::CommonJS
            }
        }
    }
}

/** @internal */
pub fn getEmitModuleResolutionKind(compiler_options: &CompilerOptions) -> ModuleResolutionKind {
    match compiler_options.moduleResolution {
        Some(resolution) => resolution,
        None => match getEmitModuleKind(compiler_options) {
            ModuleKind::CommonJS => ModuleResolutionKind::Node10,
            ModuleKind::Node16 => ModuleResolutionKind::Node16,
            ModuleKind::NodeNext => ModuleResolutionKind::NodeNext,
            ModuleKind::Preserve => ModuleResolutionKind::Bundler,
            _ => ModuleResolutionKind::Classic,
        },
    }
}

/** @internal */
pub fn getEmitModuleDetectionKind(compiler_options: &CompilerOptions) -> ModuleDetectionKind {
    compiler_options.moduleDetection.unwrap_or_else(|| if getEmitModuleKind(compiler_options) == ModuleKind::Node16 || getEmitModuleKind(compiler_options) == ModuleKind::NodeNext { ModuleDetectionKind::Force } else { ModuleDetectionKind::Auto })
}

/** @internal */
pub fn getIsolatedModules(compiler_options: &CompilerOptions) -> bool { compiler_options.isolatedModules.unwrap_or(false) || compiler_options.verbatimModuleSyntax.unwrap_or(false) }

/** @internal */
pub fn getESModuleInterop(compiler_options: &CompilerOptions) -> bool {
    match compiler_options.esModuleInterop {
        Some(value) => value,
        None => matches!(getEmitModuleKind(compiler_options), ModuleKind::Node16 | ModuleKind::NodeNext | ModuleKind::Preserve),
    }
}

/** @internal */
pub fn getAllowSyntheticDefaultImports(compiler_options: &CompilerOptions) -> bool {
    if let Some(value) = compiler_options.allowSyntheticDefaultImports {
        return value;
    }

    getESModuleInterop(compiler_options) || getEmitModuleKind(compiler_options) == ModuleKind::System || getEmitModuleResolutionKind(compiler_options) == ModuleResolutionKind::Bundler
}

/** @internal */
pub fn getResolvePackageJsonExports(compiler_options: &CompilerOptions) -> bool {
    let module_resolution = getEmitModuleResolutionKind(compiler_options);

    if !moduleResolutionSupportsPackageJsonExportsAndImports(module_resolution) {
        return false;
    }

    match compiler_options.resolvePackageJsonExports {
        Some(value) => value,
        None => matches!(module_resolution, ModuleResolutionKind::Node16 | ModuleResolutionKind::NodeNext | ModuleResolutionKind::Bundler),
    }
}

/** @internal */
pub fn getResolvePackageJsonImports(compiler_options: &CompilerOptions) -> bool {
    let module_resolution = getEmitModuleResolutionKind(compiler_options);

    if !moduleResolutionSupportsPackageJsonExportsAndImports(module_resolution) {
        return false;
    }

    match compiler_options.resolvePackageJsonExports {
        Some(value) => value,
        None => matches!(module_resolution, ModuleResolutionKind::Node16 | ModuleResolutionKind::NodeNext | ModuleResolutionKind::Bundler),
    }
}

/** @internal */
pub fn getResolveJsonModule(compiler_options: &CompilerOptions) -> bool { compiler_options.resolveJsonModule.unwrap_or_else(|| getEmitModuleResolutionKind(compiler_options) == ModuleResolutionKind::Bundler) }

/** @internal */
pub fn getEmitDeclarations(compiler_options: &CompilerOptions) -> bool { compiler_options.declaration.unwrap_or(false) || compiler_options.composite.unwrap_or(false) }

/** @internal */
pub fn shouldPreserveConstEnums(compiler_options: &CompilerOptions) -> bool { compiler_options.preserveConstEnums.unwrap_or(false) || getIsolatedModules(compiler_options) }

/** @internal */
pub fn isIncrementalCompilation(compiler_options: &CompilerOptions) -> bool { compiler_options.incremental.unwrap_or(false) || compiler_options.composite.unwrap_or(false) }

/** @internal */
pub fn getAreDeclarationMapsEnabled(compiler_options: &CompilerOptions) -> bool { compiler_options.declarationMap.unwrap_or(false) && getEmitDeclarations(compiler_options) }

/** @internal */
pub fn getAllowJSCompilerOption(compiler_options: &CompilerOptions) -> bool { compiler_options.allowJs.unwrap_or_else(|| compiler_options.checkJs.unwrap_or(false)) }

/** @internal */
pub fn getUseDefineForClassFields(compiler_options: &CompilerOptions) -> bool { compiler_options.useDefineForClassFields.unwrap_or_else(|| getEmitScriptTarget(compiler_options) >= ScriptTarget::ES2022) }

/** @internal */
pub fn emitModuleKindIsNonNodeESM(module_kind: ModuleKind) -> bool { module_kind >= ModuleKind::ES2015 && module_kind <= ModuleKind::ESNext }

/** @internal */
pub fn hasJsonModuleEmitEnabled(options: &CompilerOptions) -> bool {
    match getEmitModuleKind(options) {
        ModuleKind::None | ModuleKind::System | ModuleKind::UMD => false,
        _ => true,
    }
}

/** @internal */
pub fn unreachableCodeIsError(options: &CompilerOptions) -> bool { options.allowUnreachableCode == Some(false) }

/** @internal */
pub fn unusedLabelIsError(options: &CompilerOptions) -> bool { options.allowUnusedLabels == Some(false) }

/** @internal */
pub fn moduleResolutionSupportsPackageJsonExportsAndImports(module_resolution: ModuleResolutionKind) -> bool { (module_resolution >= ModuleResolutionKind::Node16 && module_resolution <= ModuleResolutionKind::NodeNext) || module_resolution == ModuleResolutionKind::Bundler }

/** @internal */
pub enum StrictOptionName {
    NoImplicitAny,
    NoImplicitThis,
    StrictNullChecks,
    StrictFunctionTypes,
    StrictBindCallApply,
    StrictPropertyInitialization,
    StrictBuiltinIteratorReturn,
    AlwaysStrict,
    UseUnknownInCatchVariables,
}

/** @internal */
pub fn getStrictOptionValue(compiler_options: &CompilerOptions, flag: StrictOptionName) -> bool {
    match flag {
        StrictOptionName::NoImplicitAny => compiler_options.noImplicitAny.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::NoImplicitThis => compiler_options.noImplicitThis.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictNullChecks => compiler_options.strictNullChecks.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictFunctionTypes => compiler_options.strictFunctionTypes.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictBindCallApply => compiler_options.strictBindCallApply.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictPropertyInitialization => compiler_options.strictPropertyInitialization.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictBuiltinIteratorReturn => compiler_options.strictBuiltinIteratorReturn.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::AlwaysStrict => compiler_options.alwaysStrict.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::UseUnknownInCatchVariables => compiler_options.useUnknownInCatchVariables.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
    }
}

// /** @internal */
// export function getNameOfScriptTarget(scriptTarget: ScriptTarget): string | undefined {
//   return forEachEntry(targetOptionDeclaration.type, (value, key) => value === scriptTarget ? key : undefined);
// }

/** @internal */
pub fn getEmitStandardClassFields(compiler_options: &CompilerOptions) -> bool { compiler_options.useDefineForClassFields != Some(false) && getEmitScriptTarget(compiler_options) >= ScriptTarget::ES2022 }

// /** @internal */
// export function compilerOptionsAffectSemanticDiagnostics(newOptions: CompilerOptions, oldOptions: CompilerOptions): boolean {
//   return optionsHaveChanges(oldOptions, newOptions, semanticDiagnosticsOptionDeclarations);
// }

// /** @internal */
// export function compilerOptionsAffectEmit(newOptions: CompilerOptions, oldOptions: CompilerOptions): boolean {
//   return optionsHaveChanges(oldOptions, newOptions, affectsEmitOptionDeclarations);
// }

// /** @internal */
// export function compilerOptionsAffectDeclarationPath(newOptions: CompilerOptions, oldOptions: CompilerOptions): boolean {
//   return optionsHaveChanges(oldOptions, newOptions, affectsDeclarationPathOptionDeclarations);
// }

// /** @internal */
// export function getCompilerOptionValue(options: CompilerOptions, option: CommandLineOption): unknown {
//   return option.strictFlag ? getStrictOptionValue(options, option.name as StrictOptionName) :
//       option.allowJsFlag ? getAllowJSCompilerOption(options) :
//       options[option.name];
// }

/** @internal */
pub fn getJSXTransformEnabled(options: &CompilerOptions) -> bool { matches!(options.jsx, Some(JsxEmit::React | JsxEmit::ReactJSX | JsxEmit::ReactJSXDev)) }

// /** @internal */
// export function getJSXImplicitImportBase(compilerOptions: CompilerOptions, file?: SourceFile): string | undefined {
//   const jsxImportSourcePragmas = file?.pragmas.get("jsximportsource");
//   const jsxImportSourcePragma = isArray(jsxImportSourcePragmas) ? jsxImportSourcePragmas[jsxImportSourcePragmas.length - 1] : jsxImportSourcePragmas;
//   const jsxRuntimePragmas = file?.pragmas.get("jsxruntime");
//   const jsxRuntimePragma = isArray(jsxRuntimePragmas) ? jsxRuntimePragmas[jsxRuntimePragmas.length - 1] : jsxRuntimePragmas;
//   if (jsxRuntimePragma?.arguments.factory === "classic") {
//       return undefined;
//   }
//   return compilerOptions.jsx === JsxEmit.ReactJSX ||
//           compilerOptions.jsx === JsxEmit.ReactJSXDev ||
//           compilerOptions.jsxImportSource ||
//           jsxImportSourcePragma ||
//           jsxRuntimePragma?.arguments.factory === "automatic" ?
//       jsxImportSourcePragma?.arguments.factory || compilerOptions.jsxImportSource || "react" :
//       undefined;
// }

/** @internal */
pub fn getJSXRuntimeImport(base: Option<&str>, options: &CompilerOptions) -> Option<String> { base.map(|b| format!("{}/{}", b, if options.jsx == Some(JsxEmit::ReactJSXDev) { "jsx-dev-runtime" } else { "jsx-runtime" })) }

/** @internal */
pub fn hasZeroOrOneAsteriskCharacter(str: &str) -> bool {
    let mut seen_asterisk = false;
    for c in str.chars() {
        if c == '*' {
            if !seen_asterisk {
                seen_asterisk = true;
            } else {
                // have already seen asterisk
                return false;
            }
        }
    }
    true
}
// endregion: 9202

// region: 9974
/** @internal */
pub fn positionIsSynthesized(pos: u32) -> bool {
    // This is a fast way of testing the following conditions:
    //  pos === undefined || pos === null || isNaN(pos) || pos < 0;
    !(pos >= 0)
}
// endregion: 9979

// region: 10058
/** @internal */
pub fn addRelatedInfo<'a, 'b>(diagnostic: &'a mut Diagnostic<'b>, relatedInformation: Vec<DiagnosticRelatedInformation<'b>>) {
    if relatedInformation.is_empty() {
        return;
    }

    let diag = diagnostic;
    if diag.relatedInformation.is_none() {
        diag.relatedInformation = Some(Vec::new());
    }

    debug_assert!(!matches!(diag.relatedInformation, Some(ref arr) if arr.is_empty()), "Diagnostic had empty array singleton for related info, but is still being constructed!");

    if let Some(ref mut related) = diag.relatedInformation {
        related.extend(relatedInformation);
    }
}
// endregion: 10071

// region: 10851
/** @internal */
pub fn getEscapedTextOfJsxNamespacedName(node: &JSXNamespacedName) -> __String { format!("{}:{}", node.namespace.escaped_text(), idText(MemberName::from_ast_kind(&node.property.to_ast_kind()).unwrap())) }

/** @internal */
pub fn getTextOfJsxNamespacedName(node: &JSXNamespacedName) -> String { format!("{}:{}", idText(MemberName::from_ast_kind(&node.namespace.to_ast_kind()).unwrap()), idText(MemberName::from_ast_kind(&node.property.to_ast_kind()).unwrap())) }
// endregion: 10861
