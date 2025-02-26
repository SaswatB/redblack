use oxc_ast::{
    ast::{
        Argument, ArrayExpression, ArrayPattern, ArrayPatternElement, ArrowFunctionExpression, AssignmentExpression, AssignmentOperator, AwaitExpression, BigIntLiteral, BindingIdentifier, BindingProperty, BindingRestElement, BlockStatement, BooleanLiteral, CallExpression, CatchClause,
        ChainExpression, Class, ClassExtends, ConditionalExpression, Declaration, Decorator, DestructureBindingPattern, ElementAccessExpression, ExportAllDeclaration, ExportDefaultDeclaration, ExportNamedDeclaration, ExportSpecifier, Expression, ExpressionWithTypeArguments, ForInStatement,
        ForOfStatement, ForStatement, FormalParameter, Function, FunctionBody, GeneralBinaryExpression, GeneralBinaryOperator, IdentifierName, IdentifierReference, ImportDefaultSpecifier, ImportExpression, ImportNamespaceSpecifier, ImportSpecifier, JSXAttribute, JSXElement, JSXFragment,
        JSXNamespacedName, JSXSpreadAttribute, LogicalExpression, LogicalOperator, MetaProperty, MethodDefinition, NewExpression, NullLiteral, NumericLiteral, ObjectExpression, ObjectPattern, ObjectProperty, ParenthesizedExpression, PrivateFieldExpression, PrivateIdentifier, PrivateInExpression,
        PropertyDefinition, RegExpLiteral, SequenceExpression, SourceFile, SpreadElement, StaticBlock, StaticMemberExpression, StringLiteral, Super, SwitchStatement, TSAsExpression, TSCallSignatureDeclaration, TSClassImplements, TSConditionalType, TSConstructSignatureDeclaration, TSConstructorType,
        TSEnumDeclaration, TSEnumMember, TSEnumMemberName, TSExportAssignment, TSFunctionType, TSImportEqualsDeclaration, TSIndexSignature, TSInstantiationExpression, TSInterfaceDeclaration, TSInterfaceHeritage, TSMappedType, TSMethodSignature, TSModuleDeclaration, TSModuleDeclarationName,
        TSNamespaceExportDeclaration, TSNonNullExpression, TSPropertySignature, TSQualifiedName, TSSatisfiesExpression, TSTypeAliasDeclaration, TSTypeAssertion, TSTypeLiteral, TSTypeParameter, TaggedTemplateExpression, TemplateLiteral, ThisExpression, UnaryExpression, UpdateExpression,
        VariableDeclarator, YieldExpression,
    },
    AstKind, GetChildren,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{define_flags, define_string_enum, define_subset_enum, entity_properties, flag_names_impl, flow_node_enum, opt_rc_cell, rc_cell};

use super::{
    moduleNameResolver::PackageJsonInfoCache,
    rb_unions::{StringOrDiagnosticMessageChain, StringOrNumber},
};

#[derive(Debug, Clone)]
pub struct IndexInfo;

#[derive(Debug, Clone)]
pub struct BaseType;

#[derive(Debug)]
pub struct TypeNode;

#[derive(Debug)]
pub struct TypePredicate;

#[derive(Debug)]
pub struct TypePredicateNode;

#[derive(Debug, Clone)]
pub struct TypeParameter;

#[derive(Debug)]
pub struct AssignmentPattern;

#[derive(Debug)]
pub struct EmitTextWriter;

#[derive(Debug)]
pub struct JsxAttributeLike;

#[derive(Debug)]
pub struct JsxSpreadAttribute;

#[derive(Debug)]
pub struct EnumMember;

#[derive(Debug)]
pub struct QualifiedName;

#[derive(Debug)]
pub struct ImportTypeNode;

#[derive(Debug)]
pub struct JsxAttributes;

#[derive(Debug)]
pub struct CancellationToken;

#[derive(Debug)]
pub struct EmitResolver;

#[derive(Debug)]
pub struct JSDocParameterTag;

#[derive(Debug)]
pub struct ThisContainer;

#[derive(Debug)]
pub struct TypeOnlyAliasDeclaration;

#[derive(Debug)]
pub struct ClassElement;

#[derive(Debug)]
pub struct AnyImportSyntax;

#[derive(Debug)]
pub struct StringLiteralType;

#[derive(Debug)]
pub struct NumberLiteralType;

#[derive(Debug)]
pub struct BigIntLiteralType;

#[derive(Debug)]
pub struct PseudoBigInt;

#[derive(Debug)]
pub struct TransientSymbol;

#[derive(Debug)]
pub struct SymbolAccessibilityResult;

#[derive(Debug)]
pub struct SymbolWalker;

#[derive(Debug)]
pub enum IndexKind {
    String,
    Number,
}

#[derive(Debug)]
pub enum NodeBuilderFlags {}

#[derive(Debug)]
pub enum InternalNodeBuilderFlags {}

#[derive(Debug)]
pub enum TypeFormatFlags {}

#[derive(Debug)]
pub enum SymbolFormatFlags {}

#[derive(Debug)]
pub enum ContextFlags {}

#[derive(Debug)]
pub enum UnionReduction {}

#[derive(Debug)]
pub enum MemberOverrideStatus {}

#[derive(Debug)]
pub struct TypeReference;

#[derive(Debug)]
pub struct SymbolTracker;

#[derive(Debug)]
pub struct JSDocSignature;

// region: 961
type LocalsContainer<'a> = HasLocals<'a>;
entity_properties!(HasLocals, {
    locals: Option<Rc<RefCell<SymbolTable<'static>>>> = None,
    nextContainer: Option<HasLocals<'static>> = None,
});
// endregion: 967

// region: 1403

/**
 * Declarations that can contain other declarations. Corresponds with `ContainerFlags.IsContainer` in binder.ts.
 *
 * @internal
 */
define_subset_enum!(IsContainer from AstKind {
    // ClassExpression
    Class,
    // ClassDeclaration
    // * Part of Class
    // EnumDeclaration
    TSEnumDeclaration,
    // ObjectLiteralExpression
    ObjectExpression,
    // TypeLiteralNode
    TSTypeLiteral,
    // ! rb ignoring jsdoc
    // // JSDocTypeLiteral
    // JsxAttributes
    JSXAttribute,
    // InterfaceDeclaration
    TSInterfaceDeclaration,
    // ModuleDeclaration
    TSModuleDeclaration,
    // TypeAliasDeclaration
    TSTypeAliasDeclaration,
    // MappedTypeNode
    TSMappedType,
    // IndexSignatureDeclaration
    TSIndexSignature,
    // SourceFile
    SourceFile,
    // GetAccessorDeclaration
    // * Part of MethodDefinition
    // SetAccessorDeclaration
    // * Part of MethodDefinition
    // MethodDeclaration
    MethodDefinition,
    // ConstructorDeclaration
    // * Part of MethodDefinition
    // FunctionDeclaration
    Function,
    // MethodSignature
    TSMethodSignature,
    // CallSignatureDeclaration
    TSCallSignatureDeclaration,
    // ! rb ignoring jsdoc
    // // JSDocSignature
    // // JSDocFunctionType
    // FunctionTypeNode
    TSFunctionType,
    // ConstructSignatureDeclaration
    TSConstructSignatureDeclaration,
    // ConstructorTypeNode
    TSConstructorType,
    // ClassStaticBlockDeclaration
    StaticBlock,
    // FunctionExpression
    // * Part of Function
    // ArrowFunction
    ArrowFunctionExpression,
});

/**
 * Nodes that introduce a new block scope. Corresponds with `ContainerFlags.IsBlockScopedContainer` in binder.ts.
 *
 * @internal
 */
define_subset_enum!(IsBlockScopedContainer from AstKind {
    // IsContainer
    Sub(IsContainer),
    // CatchClause
    CatchClause,
    // ForStatement
    ForStatement,
    // ForInStatement
    ForInStatement,
    // ForOfStatement
    ForOfStatement,
    // CaseBlock
    SwitchStatement,
    // Block
    BlockStatement,
});
// endregion: 1450

// region: 1508
/**
 * Nodes that can have local symbols. Corresponds with `ContainerFlags.HasLocals`. Constituents should extend
 * {@link LocalsContainer}.
 *
 * @internal
 */
define_subset_enum!(HasLocals from AstKind {
    ArrowFunctionExpression,
    // Block
    BlockStatement,
    FunctionBody,
    // CallSignatureDeclaration
    TSCallSignatureDeclaration,
    // CaseBlock
    SwitchStatement,
    // CatchClause
    CatchClause,
    // ClassStaticBlockDeclaration
    StaticBlock,
    // ConditionalTypeNode
    TSConditionalType,
    // ConstructorDeclaration
    MethodDefinition,
    // ConstructorTypeNode
    TSConstructorType,
    // ConstructSignatureDeclaration
    TSConstructSignatureDeclaration,
    // ForStatement
    ForStatement,
    // ForInStatement
    ForInStatement,
    // ForOfStatement
    ForOfStatement,
    // FunctionDeclaration
    Function,
    // FunctionExpression
    // * merged with Function
    // FunctionTypeNode
    TSFunctionType,
    // GetAccessorDeclaration
    // * merged with MethodDefinition
    // IndexSignatureDeclaration
    TSIndexSignature,
    // ! rb ignoring jsdoc
    // // JSDocCallbackTag
    // // JSDocEnumTag
    // // JSDocFunctionType
    // // JSDocSignature
    // // JSDocTypedefTag
    // MappedTypeNode
    TSMappedType,
    // MethodDeclaration
    // * merged with MethodDefinition
    // MethodSignature
    TSMethodSignature,
    // ModuleDeclaration
    TSModuleDeclaration,
    // SetAccessorDeclaration
    // * merged with MethodDefinition
    // SourceFile
    SourceFile,
    // TypeAliasDeclaration
    TSTypeAliasDeclaration,
});
impl<'a> HasLocals<'a> {
    pub fn get_node_id(&self) -> u32 { self.to_ast_kind().get_node_id() }
}
// endregion: 1544

// region: 1578
pub type MutableNodeArray<'a> = NodeArray<'a>;

#[derive(Debug, Clone)]
pub struct NodeArray<'a> {
    // TextRange
    pub pos: u32,
    pub end: u32,
    // Array
    pub items: Vec<AstKind<'a>>,
    // NodeArray
    pub hasTrailingComma: bool,
    // pub transformFlags: TransformFlags, // todo(RB): add this if needed
}
// endregion: 1587

// region: 1696
define_subset_enum!(Identifier from AstKind {
    IdentifierName,
    BindingIdentifier,
    IdentifierReference,
});
impl<'a> Identifier<'a> {
    pub fn str_name(&self) -> &str {
        match self {
            Identifier::IdentifierName(identifier_name) => identifier_name.name.as_str(),
            Identifier::BindingIdentifier(binding_identifier) => binding_identifier.name.as_str(),
            Identifier::IdentifierReference(identifier_reference) => identifier_reference.name.as_str(),
        }
    }
}
impl<'a> NamedDeclarationTrait for Identifier<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.to_ast_kind()).unwrap()) }
}
impl<'a> NamedDeclarationTrait for IdentifierName<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.to_ast_kind()).unwrap()) }
}
impl<'a> NamedDeclarationTrait for BindingIdentifier<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.to_ast_kind()).unwrap()) }
}
impl<'a> NamedDeclarationTrait for IdentifierReference<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.to_ast_kind()).unwrap()) }
}
// endregion: 1703

// region: 1724
define_subset_enum!(EntityName from AstKind {
    Sub(Identifier),
    TSQualifiedName,
});

define_subset_enum!(PropertyName from AstKind {
    Sub(Identifier),
    StringLiteral,
    TemplateLiteral, // no_substitution_template: true
    NumericLiteral,
    ObjectProperty, // computed: true
    PrivateIdentifier,
    BigIntLiteral,
});

define_subset_enum!(MemberName from AstKind {
    Sub(Identifier),
    PrivateIdentifier,
});

define_subset_enum!(DeclarationName from AstKind {
    Sub(PropertyName),
    JSXAttribute,
    StringLiteral,
    ElementAccessExpression,
    DestructureBindingPattern,
    Sub(EntityNameExpression),
});

// export interface Declaration extends Node {
//     _declarationBrand: any;
//     /** @internal */ symbol: Symbol; // Symbol declared by node (initialized by binding)
//     /** @internal */ localSymbol?: Symbol; // Local symbol declared by node (initialized by binding only for exported nodes)
// }
define_subset_enum!(NamedDeclaration from AstKind {
    // Sub(DynamicNamedDeclaration),
    Sub(DeclarationStatement),
    TSTypeParameter,
    Sub(SignatureDeclaration),
    VariableDeclarator,
    Sub(ParameterDeclaration),
    Sub(BindingElement),
    Sub(ObjectLiteralElement),
    Sub(PropertyAccessExpression),
    Class,
    Sub(TypeElement),
    TSEnumMember,
    Sub(ImportClause),
    ImportNamespaceSpecifier,
    TSNamespaceExportDeclaration,
    ImportSpecifier,
    ExportSpecifier,
    // ! rb ignoring jsdoc
    // JSDocTypedefTag,
    // JSDocCallbackTag,
});
impl<'a> NamedDeclarationTrait for NamedDeclaration<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            NamedDeclaration::DeclarationStatement(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::TSTypeParameter(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::SignatureDeclaration(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::VariableDeclarator(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::ParameterDeclaration(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::BindingElement(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::ObjectLiteralElement(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::PropertyAccessExpression(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::Class(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::TypeElement(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::TSEnumMember(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::ImportClause(n) => NamedDeclarationTrait::name(n),
            NamedDeclaration::ImportNamespaceSpecifier(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::TSNamespaceExportDeclaration(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::ImportSpecifier(n) => NamedDeclarationTrait::name(*n),
            NamedDeclaration::ExportSpecifier(n) => NamedDeclarationTrait::name(*n),
        }
    }
}

pub trait NamedDeclarationTrait {
    fn name(&self) -> Option<DeclarationName<'_>>;
}

/** @internal */
// define_subset_enum!(DynamicNamedDeclaration from AstKind {
//     LateBoundDeclaration
// });
// endregion: 1765

// region: 1786

define_subset_enum!(DeclarationStatement from AstKind {
    Function,
    // MissingDeclaration,
    Class,
    TSInterfaceDeclaration,
    TSTypeAliasDeclaration,
    TSEnumDeclaration,
    TSModuleDeclaration,
    TSImportEqualsDeclaration,
    TSNamespaceExportDeclaration,
    Sub(ExportDeclaration),
    TSExportAssignment,
    ExportDefaultDeclaration,
});

impl<'a> NamedDeclarationTrait for DeclarationStatement<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            DeclarationStatement::Function(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::Class(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::TSInterfaceDeclaration(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::TSTypeAliasDeclaration(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::TSEnumDeclaration(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::TSModuleDeclaration(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::TSImportEqualsDeclaration(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::TSNamespaceExportDeclaration(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::ExportDeclaration(n) => NamedDeclarationTrait::name(n),
            DeclarationStatement::TSExportAssignment(n) => NamedDeclarationTrait::name(*n),
            DeclarationStatement::ExportDefaultDeclaration(n) => NamedDeclarationTrait::name(*n),
        }
    }
}

// endregion: 1790

// region: 1821
// export interface TypeParameterDeclaration extends NamedDeclaration, JSDocContainer {
impl NamedDeclarationTrait for TSTypeParameter<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.name.to_ast_kind()).expect("TSTypeParameter must have a name?")) }
}
// endregion: 1834

// region: 1844
define_subset_enum!(SignatureDeclaration from AstKind {
    // CallSignatureDeclaration
    TSCallSignatureDeclaration,
    // ConstructSignatureDeclaration
    TSConstructSignatureDeclaration,
    // MethodSignature
    TSMethodSignature,
    // IndexSignatureDeclaration
    TSIndexSignature,
    // FunctionTypeNode
    TSFunctionType,
    // ConstructorTypeNode
    TSConstructorType,
    // JSDocFunctionType
    // ! rb ignoring jsdoc
    // FunctionDeclaration
    Function,
    // MethodDeclaration
    // * merged with MethodDefinition
    MethodDefinition,
    // ConstructorDeclaration
    // * merged with MethodDefinition
    // AccessorDeclaration
    // * merged with MethodDefinition
    // FunctionExpression
    // * merged with Function
    // ArrowFunction
    ArrowFunctionExpression,
});
impl<'a> NamedDeclarationTrait for SignatureDeclaration<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            SignatureDeclaration::TSCallSignatureDeclaration(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::TSConstructSignatureDeclaration(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::TSMethodSignature(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::TSIndexSignature(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::TSFunctionType(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::TSConstructorType(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::Function(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::MethodDefinition(n) => NamedDeclarationTrait::name(*n),
            SignatureDeclaration::ArrowFunctionExpression(n) => NamedDeclarationTrait::name(*n),
        }
    }
}

// export interface CallSignatureDeclaration extends SignatureDeclarationBase, TypeElement, LocalsContainer {
impl NamedDeclarationTrait for TSCallSignatureDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}

// export interface ConstructSignatureDeclaration extends SignatureDeclarationBase, TypeElement, LocalsContainer {
impl NamedDeclarationTrait for TSConstructSignatureDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}

// export type BindingName = Identifier | BindingPattern;

impl NamedDeclarationTrait for VariableDeclarator<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::DestructureBindingPattern(&self.id)) }
}
// endregion: 1877

// region: 1887
define_subset_enum!(ParameterDeclaration from AstKind {
    FormalParameter,
    BindingRestElement,
});
impl<'a> NamedDeclarationTrait for ParameterDeclaration<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            ParameterDeclaration::FormalParameter(formal_parameter) => NamedDeclarationTrait::name(*formal_parameter),
            ParameterDeclaration::BindingRestElement(binding_rest_element) => NamedDeclarationTrait::name(*binding_rest_element),
        }
    }
}
impl<'a> NamedDeclarationTrait for FormalParameter<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::DestructureBindingPattern(&self.pattern)) }
}

define_subset_enum!(BindingElement from AstKind {
    BindingProperty,
    ArrayPatternElement,
    BindingRestElement,
});
impl NamedDeclarationTrait for BindingElement<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            BindingElement::BindingProperty(binding_property) => binding_property.name(),
            BindingElement::ArrayPatternElement(array_pattern_element) => array_pattern_element.name(),
            BindingElement::BindingRestElement(binding_rest_element) => binding_rest_element.name(),
        }
    }
}
impl NamedDeclarationTrait for BindingProperty<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::DestructureBindingPattern(&self.value)) }
}
impl NamedDeclarationTrait for ArrayPatternElement<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        if let Some(element) = &self.element {
            Some(DeclarationName::DestructureBindingPattern(element))
        } else {
            None
        }
    }
}
impl NamedDeclarationTrait for BindingRestElement<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::DestructureBindingPattern(&self.argument)) }
}

// /** @internal */
// export type BindingElementGrandparent = BindingElement["parent"]["parent"];

// export interface PropertySignature extends TypeElement, JSDocContainer
impl NamedDeclarationTrait for TSPropertySignature<'_> {
    // ! rb typescript says that name is a PropertyName, but Oxc says name is a PropertyKey. Idk, so we use a potentially lossy conversion
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.key.to_ast_kind()).expect("TSPropertySignature must have a name?")) }
}

// export interface PropertyDeclaration extends ClassElement, JSDocContainer
impl NamedDeclarationTrait for PropertyDefinition<'_> {
    // ! rb typescript says that name is a PropertyName, but Oxc says name is a PropertyKey. Idk, so we use a potentially lossy conversion
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.key.to_ast_kind()).expect("PropertyDefinition must have a name?")) }
}

// export interface AutoAccessorPropertyDeclaration extends PropertyDeclaration {
//     _autoAccessorBrand: any;
// }

// /** @internal */
// export interface PrivateIdentifierPropertyDeclaration extends PropertyDeclaration {
//     name: PrivateIdentifier;
// }
// /** @internal */
// export interface PrivateIdentifierAutoAccessorPropertyDeclaration extends AutoAccessorPropertyDeclaration {
//     name: PrivateIdentifier;
// }
// /** @internal */
// export interface PrivateIdentifierMethodDeclaration extends MethodDeclaration {
//     name: PrivateIdentifier;
// }
// /** @internal */
// export interface PrivateIdentifierGetAccessorDeclaration extends GetAccessorDeclaration {
//     name: PrivateIdentifier;
// }
// /** @internal */
// export interface PrivateIdentifierSetAccessorDeclaration extends SetAccessorDeclaration {
//     name: PrivateIdentifier;
// }
// /** @internal */
// export type PrivateIdentifierAccessorDeclaration = PrivateIdentifierGetAccessorDeclaration | PrivateIdentifierSetAccessorDeclaration;
// /** @internal */
// export type PrivateClassElementDeclaration =
//     | PrivateIdentifierPropertyDeclaration
//     | PrivateIdentifierAutoAccessorPropertyDeclaration
//     | PrivateIdentifierMethodDeclaration
//     | PrivateIdentifierGetAccessorDeclaration
//     | PrivateIdentifierSetAccessorDeclaration;

// /** @internal */
// export type InitializedPropertyDeclaration = PropertyDeclaration & { readonly initializer: Expression; };

// export interface ObjectLiteralElement extends NamedDeclaration {
define_subset_enum!(ObjectLiteralElement from AstKind {
    ObjectProperty,
    SpreadElement,
    MethodDefinition,
    JSXSpreadAttribute,
});
impl NamedDeclarationTrait for ObjectLiteralElement<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            ObjectLiteralElement::ObjectProperty(object_property) => NamedDeclarationTrait::name(*object_property),
            ObjectLiteralElement::SpreadElement(spread_element) => NamedDeclarationTrait::name(*spread_element),
            ObjectLiteralElement::MethodDefinition(method_definition) => NamedDeclarationTrait::name(*method_definition),
            ObjectLiteralElement::JSXSpreadAttribute(jsx_spread_attribute) => NamedDeclarationTrait::name(*jsx_spread_attribute),
        }
    }
}

define_subset_enum!(ObjectLiteralElementLike from AstKind {
    ObjectProperty,
    SpreadElement,
    MethodDefinition,
});

// export interface PropertyAssignment extends ObjectLiteralElement, JSDocContainer {
// export interface ShorthandPropertyAssignment extends ObjectLiteralElement, JSDocContainer {
impl NamedDeclarationTrait for ObjectProperty<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { DeclarationName::from_ast_kind(&self.key.to_ast_kind()) }
}

// export interface SpreadAssignment extends ObjectLiteralElement, JSDocContainer {
impl NamedDeclarationTrait for SpreadElement<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { DeclarationName::from_ast_kind(&self.argument.to_ast_kind()) }
}
// endregion: 2020

// region: 2046
define_subset_enum!(BindingPattern from AstKind {
    ObjectPattern,
    ArrayPattern,
});

define_subset_enum!(ArrayBindingElement from AstKind {
    ArrayPatternElement,
    BindingRestElement,
});

// /**
//  * Several node kinds share function-like features such as a signature,
//  * a name, and a body. These nodes should extend FunctionLikeDeclarationBase.
//  * Examples:
//  * - FunctionDeclaration
//  * - MethodDeclaration
//  * - AccessorDeclaration
//  */
// export interface FunctionLikeDeclarationBase extends SignatureDeclarationBase {
//     _functionLikeDeclarationBrand: any;

//     readonly asteriskToken?: AsteriskToken | undefined;
//     readonly questionToken?: QuestionToken | undefined;
//     readonly exclamationToken?: ExclamationToken | undefined;
//     readonly body?: Block | Expression | undefined;
//     /** @internal */ endFlowNode?: FlowNode;
//     /** @internal */ returnFlowNode?: FlowNode;
// }

// export type FunctionLikeDeclaration =
//     | FunctionDeclaration
//     | MethodDeclaration
//     | GetAccessorDeclaration
//     | SetAccessorDeclaration
//     | ConstructorDeclaration
//     | FunctionExpression
//     | ArrowFunction;
// /** @deprecated Use SignatureDeclaration */
// export type FunctionLike = SignatureDeclaration;

// export interface FunctionDeclaration extends FunctionLikeDeclarationBase, DeclarationStatement, LocalsContainer {
impl NamedDeclarationTrait for Function<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { self.id.as_ref().map(|id| DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(id)))) }
}

// export interface MethodSignature extends SignatureDeclarationBase, TypeElement, LocalsContainer {
impl NamedDeclarationTrait for TSMethodSignature<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { DeclarationName::from_ast_kind(&self.key.to_ast_kind()) }
}

// // Note that a MethodDeclaration is considered both a ClassElement and an ObjectLiteralElement.
// // Both the grammars for ClassDeclaration and ObjectLiteralExpression allow for MethodDeclarations
// // as child elements, and so a MethodDeclaration satisfies both interfaces.  This avoids the
// // alternative where we would need separate kinds/types for ClassMethodDeclaration and
// // ObjectLiteralMethodDeclaration, which would look identical.
// //
// // Because of this, it may be necessary to determine what sort of MethodDeclaration you have
// // at later stages of the compiler pipeline.  In that case, you can either check the parent kind
// // of the method, or use helpers like isObjectLiteralMethodDeclaration
// export interface MethodDeclaration extends FunctionLikeDeclarationBase, ClassElement, ObjectLiteralElement, JSDocContainer, LocalsContainer, FlowContainer {
//     readonly kind: SyntaxKind.MethodDeclaration;
//     readonly parent: ClassLikeDeclaration | ObjectLiteralExpression;
//     readonly modifiers?: NodeArray<ModifierLike> | undefined;
//     readonly name: PropertyName;
//     readonly body?: FunctionBody | undefined;

//     // The following properties are used only to report grammar errors (see `isGrammarError` in utilities.ts)
//     /** @internal */ readonly exclamationToken?: ExclamationToken | undefined; // A method cannot have an exclamation token
// }
impl NamedDeclarationTrait for MethodDefinition<'_> {
    // ! rb typescript says that name is a PropertyName, but Oxc says name is a PropertyKey. Idk, so we use a potentially lossy conversion
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.key.to_ast_kind()).expect("MethodDefinition must have a name?")) }
}

// export interface ConstructorDeclaration extends FunctionLikeDeclarationBase, ClassElement, JSDocContainer, LocalsContainer {
//     readonly kind: SyntaxKind.Constructor;
//     readonly parent: ClassLikeDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike> | undefined;
//     readonly body?: FunctionBody | undefined;

//     // The following properties are used only to report grammar errors (see `isGrammarError` in utilities.ts)
//     /** @internal */ readonly typeParameters?: NodeArray<TypeParameterDeclaration>; // A constructor cannot have type parameters
//     /** @internal */ readonly type?: TypeNode; // A constructor cannot have a return type annotation
// }

// /** For when we encounter a semicolon in a class declaration. ES6 allows these as class elements. */
// export interface SemicolonClassElement extends ClassElement, JSDocContainer {
//     readonly kind: SyntaxKind.SemicolonClassElement;
//     readonly parent: ClassLikeDeclaration;
// }

// // See the comment on MethodDeclaration for the intuition behind GetAccessorDeclaration being a
// // ClassElement and an ObjectLiteralElement.
// export interface GetAccessorDeclaration extends FunctionLikeDeclarationBase, ClassElement, TypeElement, ObjectLiteralElement, JSDocContainer, LocalsContainer, FlowContainer {
//     readonly kind: SyntaxKind.GetAccessor;
//     readonly parent: ClassLikeDeclaration | ObjectLiteralExpression | TypeLiteralNode | InterfaceDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     readonly name: PropertyName;
//     readonly body?: FunctionBody;

//     // The following properties are used only to report grammar errors (see `isGrammarError` in utilities.ts)
//     /** @internal */ readonly typeParameters?: NodeArray<TypeParameterDeclaration> | undefined; // A get accessor cannot have type parameters
// }

// // See the comment on MethodDeclaration for the intuition behind SetAccessorDeclaration being a
// // ClassElement and an ObjectLiteralElement.
// export interface SetAccessorDeclaration extends FunctionLikeDeclarationBase, ClassElement, TypeElement, ObjectLiteralElement, JSDocContainer, LocalsContainer, FlowContainer {
//     readonly kind: SyntaxKind.SetAccessor;
//     readonly parent: ClassLikeDeclaration | ObjectLiteralExpression | TypeLiteralNode | InterfaceDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     readonly name: PropertyName;
//     readonly body?: FunctionBody;

//     // The following properties are used only to report grammar errors (see `isGrammarError` in utilities.ts)
//     /** @internal */ readonly typeParameters?: NodeArray<TypeParameterDeclaration> | undefined; // A set accessor cannot have type parameters
//     /** @internal */ readonly type?: TypeNode | undefined; // A set accessor cannot have a return type
// }

// export type AccessorDeclaration = GetAccessorDeclaration | SetAccessorDeclaration;

// export interface IndexSignatureDeclaration extends SignatureDeclarationBase, ClassElement, TypeElement, LocalsContainer {
impl NamedDeclarationTrait for TSIndexSignature<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}
// endregion: 2166

// region: 2222
// export interface FunctionTypeNode extends FunctionOrConstructorTypeNodeBase, LocalsContainer {
impl NamedDeclarationTrait for TSFunctionType<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}

// export interface ConstructorTypeNode extends FunctionOrConstructorTypeNodeBase, LocalsContainer {
impl NamedDeclarationTrait for TSConstructorType<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}
// endregion: 2234

// region: 2367
define_subset_enum!(StringLiteralLike from AstKind {
    StringLiteral,
    TemplateLiteral, // NoSubstitutionTemplateLiteral
});
impl<'a> StringLiteralLike<'a> {
    pub fn value(&self) -> String {
        match self {
            StringLiteralLike::StringLiteral(string) => string.value.to_string(),
            StringLiteralLike::TemplateLiteral(template) => {
                let first_quasi = template.quasis.first().unwrap();
                first_quasi.value.cooked.unwrap().to_string()
            }
        }
    }
}
define_subset_enum!(PropertyNameLiteral from AstKind {
    Sub(Identifier),
    Sub(StringLiteralLike),
    NumericLiteral,
    JSXNamespacedName,
    BigIntLiteral,
});
// endregion: 2370

// region: 2383
// Note: 'brands' in our syntax nodes serve to give us a small amount of nominal typing.
// Consider 'Expression'.  Without the brand, 'Expression' is actually no different
// (structurally) than 'Node'.  Because of this you can pass any Node to a function that
// takes an Expression without any error.  By using the 'brands' we ensure that the type
// checker actually thinks you have something of the right type.  Note: the brands are
// never actually given values.  At runtime they have zero cost.

define_subset_enum!(AstKindExpression from AstKind {
    NL(BooleanLiteral),
    NL(NullLiteral),
    NumericLiteral,
    BigIntLiteral,
    RegExpLiteral,
    StringLiteral,
    TemplateLiteral,
    IdentifierReference,
    MetaProperty,
    NL(Super),
    ArrayExpression,
    ArrowFunctionExpression,
    AssignmentExpression,
    AwaitExpression,
    GeneralBinaryExpression,
    CallExpression,
    ChainExpression,
    Class,
    ConditionalExpression,
    Function,
    ImportExpression,
    LogicalExpression,
    NewExpression,
    ObjectExpression,
    ParenthesizedExpression,
    SequenceExpression,
    TaggedTemplateExpression,
    NL(ThisExpression),
    UnaryExpression,
    UpdateExpression,
    YieldExpression,
    PrivateInExpression,
    JSXElement,
    JSXFragment,
    TSAsExpression,
    TSSatisfiesExpression,
    TSTypeAssertion,
    TSNonNullExpression,
    TSInstantiationExpression,
    ElementAccessExpression,
    StaticMemberExpression,
    PrivateFieldExpression,
});

impl<'a> AstKindExpression<'a> {
    pub fn from_expression(expr: &'a Expression<'a>) -> Self { AstKindExpression::from_ast_kind(&expr.to_ast_kind()).unwrap() }
}
// endregion: 2394

// region: 2513
// export type ExponentiationOperator
// export type MultiplicativeOperator
// export type MultiplicativeOperatorOrHigher
// export type AdditiveOperator
// export type AdditiveOperatorOrHigher
// export type ShiftOperator
// export type ShiftOperatorOrHigher
// export type RelationalOperator
// export type RelationalOperatorOrHigher
// export type EqualityOperator
// export type EqualityOperatorOrHigher
// export type BitwiseOperator
// export type BitwiseOperatorOrHigher
// export type LogicalOperator
// export type LogicalOperatorOrHigher
// export type CompoundAssignmentOperator
// export type AssignmentOperator
// export type AssignmentOperatorOrHigher
// export type BinaryOperator
// export type LogicalOrCoalescingAssignmentOperator
// export type BinaryOperatorToken

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    GeneralBinaryOperator(GeneralBinaryOperator),
    AssignmentOperator(AssignmentOperator),
    LogicalOperator(LogicalOperator),
    CommaOperator,
}

define_subset_enum!(BinaryExpression from AstKind {
    GeneralBinaryExpression,
    AssignmentExpression,
    LogicalExpression,
    PrivateInExpression,
    SequenceExpression,
});
impl<'a> BinaryExpression<'a> {
    pub fn left(&self) -> AstKindExpression<'a> {
        match self {
            BinaryExpression::GeneralBinaryExpression(expr) => AstKindExpression::from_expression(&expr.left),
            // !rb this unwrap is hella sus
            BinaryExpression::AssignmentExpression(expr) => AstKindExpression::from_ast_kind(&expr.left.to_ast_kind()).unwrap(),
            BinaryExpression::LogicalExpression(expr) => AstKindExpression::from_expression(&expr.left),
            // !rb this unwrap is hella sus
            BinaryExpression::PrivateInExpression(expr) => AstKindExpression::from_ast_kind(&expr.left.to_ast_kind()).unwrap(),
            BinaryExpression::SequenceExpression(expr) => AstKindExpression::from_expression(&expr.expressions[0]),
        }
    }

    pub fn operator(&self) -> BinaryOperator {
        match self {
            BinaryExpression::GeneralBinaryExpression(expr) => BinaryOperator::GeneralBinaryOperator(expr.operator),
            BinaryExpression::AssignmentExpression(expr) => BinaryOperator::AssignmentOperator(expr.operator),
            BinaryExpression::LogicalExpression(expr) => BinaryOperator::LogicalOperator(expr.operator),
            BinaryExpression::PrivateInExpression(expr) => BinaryOperator::GeneralBinaryOperator(expr.operator),
            BinaryExpression::SequenceExpression(_) => BinaryOperator::CommaOperator,
        }
    }

    pub fn right(&self) -> AstKindExpression<'a> {
        match self {
            BinaryExpression::GeneralBinaryExpression(expr) => AstKindExpression::from_expression(&expr.right),
            BinaryExpression::AssignmentExpression(expr) => AstKindExpression::from_expression(&expr.right),
            BinaryExpression::LogicalExpression(expr) => AstKindExpression::from_expression(&expr.right),
            BinaryExpression::PrivateInExpression(expr) => AstKindExpression::from_expression(&expr.right),
            // !rb taking the last element here can probably cause issues, like missed members
            BinaryExpression::SequenceExpression(expr) => AstKindExpression::from_expression(&expr.expressions.last().unwrap()),
        }
    }
}
// endregion: 2649

// region: 2741
// export interface FunctionExpression extends PrimaryExpression, FunctionLikeDeclarationBase, JSDocContainer, LocalsContainer, FlowContainer {
// defined under FunctionDeclaration

// export interface ArrowFunction extends Expression, FunctionLikeDeclarationBase, JSDocContainer, LocalsContainer, FlowContainer {
impl NamedDeclarationTrait for ArrowFunctionExpression<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}
// endregion: 2756

// region: 2938
/**
 * This interface is a base interface for ObjectLiteralExpression and JSXAttributes to extend from. JSXAttributes is similar to
 * ObjectLiteralExpression in that it contains array of properties; however, JSXAttributes' properties can only be
 * JSXAttribute or JSXSpreadAttribute. ObjectLiteralExpression, on the other hand, can only have properties of type
 * ObjectLiteralElement (e.g. PropertyAssignment, ShorthandPropertyAssignment etc.)
 */
// export interface ObjectLiteralExpressionBase<T extends ObjectLiteralElement> extends PrimaryExpression, Declaration {
define_subset_enum!(ObjectLiteralExpressionBase from AstKind {
    ObjectExpression,
    JSXAttribute,
    JSXSpreadAttribute,
});

// An ObjectLiteralExpression is the declaration node for an anonymous symbol.
// export interface ObjectLiteralExpression extends ObjectLiteralExpressionBase<ObjectLiteralElementLike>, JSDocContainer {

define_subset_enum!(EntityNameExpression from AstKind {
    Sub(Identifier),
    Sub(PropertyAccessEntityNameExpression),
});

define_subset_enum!(EntityNameOrEntityNameExpression from AstKind {
    Sub(EntityName),
    Sub(EntityNameExpression),
});
define_subset_enum!(AccessExpression from AstKind {
    Sub(PropertyAccessExpression),
    ElementAccessExpression,
});
impl<'a> AccessExpression<'a> {
    pub fn object(&self) -> &'a Expression<'a> {
        match self {
            AccessExpression::PropertyAccessExpression(expr) => &expr.object(),
            AccessExpression::ElementAccessExpression(expr) => &expr.object,
        }
    }
}

define_subset_enum!(PropertyAccessExpression from AstKind {
    StaticMemberExpression,
    PrivateFieldExpression,
});
impl<'a> PropertyAccessExpression<'a> {
    pub fn object(&self) -> &'a Expression<'a> {
        match self {
            PropertyAccessExpression::StaticMemberExpression(expr) => &expr.object,
            PropertyAccessExpression::PrivateFieldExpression(expr) => &expr.object,
        }
    }
    pub fn property(&self) -> MemberName<'a> {
        match self {
            PropertyAccessExpression::StaticMemberExpression(expr) => MemberName::Identifier(Identifier::IdentifierName(&expr.property)),
            PropertyAccessExpression::PrivateFieldExpression(expr) => MemberName::PrivateIdentifier(&expr.field),
        }
    }
}
impl NamedDeclarationTrait for PropertyAccessExpression<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            PropertyAccessExpression::StaticMemberExpression(expr) => NamedDeclarationTrait::name(*expr),
            PropertyAccessExpression::PrivateFieldExpression(expr) => NamedDeclarationTrait::name(*expr),
        }
    }
}
impl NamedDeclarationTrait for StaticMemberExpression<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { NamedDeclarationTrait::name(&self.property) }
}

/** @internal */
// export interface PrivateIdentifierPropertyAccessExpression extends PropertyAccessExpression {
impl NamedDeclarationTrait for PrivateFieldExpression<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::PropertyName(PropertyName::PrivateIdentifier(&self.field))) }
}

// endregion: 2971

// region: 2986
// todo(RB): this is supposed to be branded, may need to change
define_subset_enum!(PropertyAccessEntityNameExpression from AstKind {
    Sub(PropertyAccessExpression)
});
// endregion: 2991

// region: 3123
/// Represents an expression that can be called or constructed, including function calls,
/// constructor calls, template literals, decorators, JSX elements, and instanceof checks.
define_subset_enum!(CallLikeExpression from AstKind {
    // Function call expression like `foo()`
    CallExpression,
    // Constructor call with new like `new Foo()`
    NewExpression,
    // Tagged template literal like `foo`bar``
    TaggedTemplateExpression,
    // Class or method decorator like `@decorator`
    Decorator,
    // JSX opening element like `<div>`
    // ! oxc doesn't have a separate JSXSelfClosingElement, so we can't explicitly model JSXOpeningElement | JSXSelfClosingElement
    JSXElement,
    // instanceof check like `foo instanceof Bar`
    // ! There doesn't seem to be a good way to explicitly model InstanceofExpression, so we use GeneralBinaryExpression
    GeneralBinaryExpression,
});
// endregion: 3129

// region: 3270
// export interface JsxSpreadAttribute extends ObjectLiteralElement {
impl NamedDeclarationTrait for JSXSpreadAttribute<'_> {
    // !rb idk how to get a name here
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}
// endregion: 3276

// region: 3520
// export interface ClassLikeDeclarationBase extends NamedDeclarationTrait, JSDocContainer {
//     readonly kind: SyntaxKind.ClassDeclaration | SyntaxKind.ClassExpression;
//     readonly name?: Identifier;
//     readonly typeParameters?: NodeArray<TypeParameterDeclaration>;
//     readonly heritageClauses?: NodeArray<HeritageClause>;
//     readonly members: NodeArray<ClassElement>;
// }

// export interface ClassDeclaration extends ClassLikeDeclarationBase, DeclarationStatement {
//     readonly kind: SyntaxKind.ClassDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     /** May be undefined in `export default class { ... }`. */
//     readonly name?: Identifier;
// }

// export interface ClassExpression extends ClassLikeDeclarationBase, PrimaryExpression {
//     readonly kind: SyntaxKind.ClassExpression;
//     readonly modifiers?: NodeArray<ModifierLike>;
// }

impl NamedDeclarationTrait for Class<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        if let Some(id) = &self.id {
            Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(id))))
        } else {
            None
        }
    }
}

pub type ClassLikeDeclaration<'a> = Class<'a>;

define_subset_enum!(TypeElement from AstKind {
    TSCallSignatureDeclaration,
    TSConstructSignatureDeclaration,
    TSPropertySignature,
    TSMethodSignature,
    MethodDefinition,
    TSIndexSignature,
});
impl<'a> NamedDeclarationTrait for TypeElement<'a> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            TypeElement::TSCallSignatureDeclaration(expr) => NamedDeclarationTrait::name(*expr),
            TypeElement::TSConstructSignatureDeclaration(expr) => NamedDeclarationTrait::name(*expr),
            TypeElement::TSPropertySignature(expr) => NamedDeclarationTrait::name(*expr),
            TypeElement::TSMethodSignature(expr) => NamedDeclarationTrait::name(*expr),
            TypeElement::MethodDefinition(expr) => NamedDeclarationTrait::name(*expr),
            TypeElement::TSIndexSignature(expr) => NamedDeclarationTrait::name(*expr),
        }
    }
}

// export interface InterfaceDeclaration extends DeclarationStatement, JSDocContainer {
//     readonly kind: SyntaxKind.InterfaceDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     readonly name: Identifier;
//     readonly typeParameters?: NodeArray<TypeParameterDeclaration>;
//     readonly heritageClauses?: NodeArray<HeritageClause>;
//     readonly members: NodeArray<TypeElement>;
// }
impl NamedDeclarationTrait for TSInterfaceDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&self.id)))) }
}

define_subset_enum!(HeritageClause from AstKind {
    TSInterfaceHeritage,
    TSClassImplements,
    ClassExtends,
});

// export interface TypeAliasDeclaration extends DeclarationStatement, JSDocContainer, LocalsContainer {
//     readonly kind: SyntaxKind.TypeAliasDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     readonly name: Identifier;
//     readonly typeParameters?: NodeArray<TypeParameterDeclaration>;
//     readonly type: TypeNode;
// }
impl NamedDeclarationTrait for TSTypeAliasDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&self.id)))) }
}

// export interface EnumMember extends NamedDeclarationTrait, JSDocContainer {
//     readonly kind: SyntaxKind.EnumMember;
//     readonly parent: EnumDeclaration;
//     // This does include ComputedPropertyName, but the parser will give an error
//     // if it parses a ComputedPropertyName in an EnumMember
//     readonly name: PropertyName;
//     readonly initializer?: Expression;
// }
impl NamedDeclarationTrait for TSEnumMember<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match &self.id {
            TSEnumMemberName::Identifier(identifier) => Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::IdentifierName(identifier)))),
            TSEnumMemberName::String(string_literal) => Some(DeclarationName::StringLiteral(string_literal)),
        }
    }
}

// export interface EnumDeclaration extends DeclarationStatement, JSDocContainer {
//     readonly kind: SyntaxKind.EnumDeclaration;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     readonly name: Identifier;
//     readonly members: NodeArray<EnumMember>;
// }
impl NamedDeclarationTrait for TSEnumDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&self.id)))) }
}

// export type ModuleName =
//     | Identifier
//     | StringLiteral;

// export type ModuleBody =
//     | NamespaceBody
//     | JSDocNamespaceBody;

// /** @internal */
// export interface AmbientModuleDeclaration extends ModuleDeclaration {
//     readonly body?: ModuleBlock;
// }

// export interface ModuleDeclaration extends DeclarationStatement, JSDocContainer, LocalsContainer {
//     readonly kind: SyntaxKind.ModuleDeclaration;
//     readonly parent: ModuleBody | SourceFile;
//     readonly modifiers?: NodeArray<ModifierLike>;
//     readonly name: ModuleName;
//     readonly body?: ModuleBody | JSDocNamespaceDeclaration;
// }
impl NamedDeclarationTrait for TSModuleDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match &self.id {
            TSModuleDeclarationName::Identifier(id) => Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&id)))),
            TSModuleDeclarationName::StringLiteral(string_literal) => Some(DeclarationName::StringLiteral(string_literal)),
        }
    }
}

// export type NamespaceBody =
//     | ModuleBlock
//     | NamespaceDeclaration;

// export interface NamespaceDeclaration extends ModuleDeclaration {
//     readonly name: Identifier;
//     readonly body: NamespaceBody;
// }

// export type JSDocNamespaceBody =
//     | Identifier
//     | JSDocNamespaceDeclaration;

// export interface JSDocNamespaceDeclaration extends ModuleDeclaration {
//     readonly name: Identifier;
//     readonly body?: JSDocNamespaceBody;
// }

// export interface ModuleBlock extends Node, Statement {
//     readonly kind: SyntaxKind.ModuleBlock;
//     readonly parent: ModuleDeclaration;
//     readonly statements: NodeArray<Statement>;
// }

// export type ModuleReference =
//     | EntityName
//     | ExternalModuleReference;

/**
 * One of:
 * - import x = require("mod");
 * - import x = M.x;
 */
// export interface ImportEqualsDeclaration extends DeclarationStatement, JSDocContainer {
impl NamedDeclarationTrait for TSImportEqualsDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&self.id)))) }
}
// endregion: 3661

// region: 3690
// In case of:
// import d from "mod" => name = d, namedBinding = undefined
// import * as ns from "mod" => name = undefined, namedBinding: NamespaceImport = { name: ns }
// import d, * as ns from "mod" => name = d, namedBinding: NamespaceImport = { name: ns }
// import { a, b as x } from "mod" => name = undefined, namedBinding: NamedImports = { elements: [{ name: a }, { name: x, propertyName: b}]}
// import d, { a, b as x } from "mod" => name = d, namedBinding: NamedImports = { elements: [{ name: a }, { name: x, propertyName: b}]}
define_subset_enum!(ImportClause from AstKind { // based on ImportDeclarationSpecifier
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
});
impl NamedDeclarationTrait for ImportClause<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            ImportClause::ImportSpecifier(import_specifier) => Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&import_specifier.local)))),
            ImportClause::ImportDefaultSpecifier(import_default_specifier) => Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&import_default_specifier.local)))),
            ImportClause::ImportNamespaceSpecifier(import_namespace_specifier) => Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&import_namespace_specifier.local)))),
        }
    }
}
// endregion: 3704

// region: 3713
// export type ImportAttributeName = Identifier | StringLiteral;

// export interface ImportAttribute extends Node {
//     readonly kind: SyntaxKind.ImportAttribute;
//     readonly parent: ImportAttributes;
//     readonly name: ImportAttributeName;
//     readonly value: Expression;
// }

// export interface ImportAttributes extends Node {
//     readonly token: SyntaxKind.WithKeyword | SyntaxKind.AssertKeyword;
//     readonly kind: SyntaxKind.ImportAttributes;
//     readonly parent: ImportDeclaration | ExportDeclaration;
//     readonly elements: NodeArray<ImportAttribute>;
//     readonly multiLine?: boolean;
// }

// export interface NamespaceImport extends NamedDeclaration {
impl NamedDeclarationTrait for ImportNamespaceSpecifier<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&self.local)))) }
}

// export interface NamespaceExport extends NamedDeclaration {

// export interface NamespaceExportDeclaration extends DeclarationStatement, JSDocContainer {
impl NamedDeclarationTrait for TSNamespaceExportDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::IdentifierName(&self.id)))) }
}

// export interface ExportDeclaration extends DeclarationStatement, JSDocContainer {
define_subset_enum!(ExportDeclaration from AstKind {
    ExportAllDeclaration,
    ExportNamedDeclaration,
    // does not include ExportDefaultDeclaration, that's under ExportAssignment
});
impl NamedDeclarationTrait for ExportDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            ExportDeclaration::ExportAllDeclaration(n) => NamedDeclarationTrait::name(*n),
            ExportDeclaration::ExportNamedDeclaration(n) => NamedDeclarationTrait::name(*n),
        }
    }
}
impl NamedDeclarationTrait for ExportAllDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { self.exported.as_ref().map(|exported| DeclarationName::from_ast_kind(&exported.to_ast_kind()).unwrap()) }
}
impl NamedDeclarationTrait for ExportNamedDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}

// export interface NamedImports extends Node {
//     readonly kind: SyntaxKind.NamedImports;
//     readonly parent: ImportClause;
//     readonly elements: NodeArray<ImportSpecifier>;
// }

// export interface NamedExports extends Node {
//     readonly kind: SyntaxKind.NamedExports;
//     readonly parent: ExportDeclaration;
//     readonly elements: NodeArray<ExportSpecifier>;
// }

// export type NamedImportsOrExports = NamedImports | NamedExports;

// export interface ImportSpecifier extends NamedDeclaration {
impl NamedDeclarationTrait for ImportSpecifier<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::EntityNameExpression(EntityNameExpression::Identifier(Identifier::BindingIdentifier(&self.local)))) }
}

// export interface ExportSpecifier extends NamedDeclaration, JSDocContainer {
impl NamedDeclarationTrait for ExportSpecifier<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { Some(DeclarationName::from_ast_kind(&self.local.to_ast_kind()).expect("ExportSpecifier should have a name")) }
}

// export type ModuleExportName = Identifier | StringLiteral;

// export type ImportOrExportSpecifier =
//     | ImportSpecifier
//     | ExportSpecifier;

// export type TypeOnlyCompatibleAliasDeclaration =
//     | ImportClause
//     | ImportEqualsDeclaration
//     | NamespaceImport
//     | ImportOrExportSpecifier
//     | ExportDeclaration
//     | NamespaceExport;

// export type TypeOnlyImportDeclaration =
//     | ImportClause & { readonly isTypeOnly: true; readonly name: Identifier; }
//     | ImportEqualsDeclaration & { readonly isTypeOnly: true; }
//     | NamespaceImport & { readonly parent: ImportClause & { readonly isTypeOnly: true; }; }
//     | ImportSpecifier & ({ readonly isTypeOnly: true; } | { readonly parent: NamedImports & { readonly parent: ImportClause & { readonly isTypeOnly: true; }; }; });

// export type TypeOnlyExportDeclaration =
//     | ExportSpecifier & ({ readonly isTypeOnly: true; } | { readonly parent: NamedExports & { readonly parent: ExportDeclaration & { readonly isTypeOnly: true; }; }; })
//     | ExportDeclaration & { readonly isTypeOnly: true; readonly moduleSpecifier: Expression; } // export * from "mod"
//     | NamespaceExport & { readonly parent: ExportDeclaration & { readonly isTypeOnly: true; readonly moduleSpecifier: Expression; }; } // export * as ns from "mod"
// ;

// export type TypeOnlyAliasDeclaration = TypeOnlyImportDeclaration | TypeOnlyExportDeclaration;

// /**
//  * This is either an `export =` or an `export default` declaration.
//  * Unless `isExportEquals` is set, this node was parsed as an `export default`.
//  */
// export interface ExportAssignment extends DeclarationStatement, JSDocContainer {
define_subset_enum!(ExportAssignment from AstKind {
    TSExportAssignment,
    ExportDefaultDeclaration,
});
impl NamedDeclarationTrait for ExportAssignment<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> {
        match self {
            ExportAssignment::TSExportAssignment(n) => NamedDeclarationTrait::name(*n),
            ExportAssignment::ExportDefaultDeclaration(n) => NamedDeclarationTrait::name(*n),
        }
    }
}
impl NamedDeclarationTrait for TSExportAssignment<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}
impl NamedDeclarationTrait for ExportDefaultDeclaration<'_> {
    fn name(&self) -> Option<DeclarationName<'_>> { None }
}
// endregion: 3833

// region: 4120
// NOTE: Ensure this is up-to-date with src/debug/debug.ts
// @internal
define_flags!(FlowFlags {
    Unreachable    = 1 << 0,  // Unreachable code
    Start          = 1 << 1,  // Start of flow graph
    BranchLabel    = 1 << 2,  // Non-looping junction
    LoopLabel      = 1 << 3,  // Looping junction
    Assignment     = 1 << 4,  // Assignment
    TrueCondition  = 1 << 5,  // Condition known to be true
    FalseCondition = 1 << 6,  // Condition known to be false
    SwitchClause   = 1 << 7,  // Switch statement clause
    ArrayMutation  = 1 << 8,  // Potential array mutation
    Call           = 1 << 9,  // Potential assertion call
    ReduceLabel    = 1 << 10, // Temporarily reduce antecedents of label
    Referenced     = 1 << 11, // Referenced as antecedent once
    Shared         = 1 << 12, // Referenced as antecedent more than once

    Label = Self::BranchLabel.0 | Self::LoopLabel.0,
    Condition = Self::TrueCondition.0 | Self::FalseCondition.0,
});

// @internal
flow_node_enum! {
    Unreachable(FlowUnreachable),
    Start(FlowStart<'a>),
    Label(FlowLabel<'a>),
    Assignment(FlowAssignment<'a>),
    Condition(FlowCondition<'a>),
    SwitchClause(FlowSwitchClause<'a>),
    ArrayMutation(FlowArrayMutation<'a>),
    Call(FlowCall<'a>),
    ReduceLabel(FlowReduceLabel<'a>),
}

/** @internal */
// pub struct FlowNodeBase<'a> {
//     pub flags: FlowFlags,
//     pub id: usize, // Node id used by flow type cache in checker
//     // pub node: Option<AstKind<'a>>, // Node or other data
//     // pub antecedent: Option<FlowNodeAntecedent<'a>>,
// }

// pub enum FlowNodeAntecedent<'a> {
//     Single(Box<FlowNode<'a>>),
//     Multiple(Vec<FlowNode<'a>>),
// }

/** @internal */
pub struct FlowUnreachable {
    pub flags: FlowFlags,
    pub id: usize,
}

// FlowStart represents the start of a control flow. For a function expression or arrow
// function, the node property references the function (which in turn has a flowNode
// property for the containing control flow).
/** @internal */
pub struct FlowStart<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: Option<&'a AstKind<'a>>, // FunctionExpression | ArrowFunction | MethodDeclaration | GetAccessorDeclaration | SetAccessorDeclaration | undefined;
}

// FlowLabel represents a junction with multiple possible preceding control flows.
/** @internal */
pub struct FlowLabel<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub antecedent: Option<Vec<FlowNode<'a>>>,
}

// FlowAssignment represents a node that assigns a value to a narrowable reference,
// i.e. an identifier or a dotted name that starts with an identifier or 'this'.
/** @internal */
pub struct FlowAssignment<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: FlowAssignmentNode<'a>,
    pub antecedent: Box<FlowNode<'a>>,
}

pub enum FlowAssignmentNode<'a> {
    Expression(&'a Expression<'a>),
    VariableDeclarator(&'a VariableDeclarator<'a>),
    BindingElement(&'a BindingElement<'a>),
}

/** @internal */
pub struct FlowCall<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: &'a CallExpression<'a>,
    pub antecedent: Box<FlowNode<'a>>,
}

// FlowCondition represents a condition that is known to be true or false at the
// node's location in the control flow.
/** @internal */
pub struct FlowCondition<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: &'a Expression<'a>,
    pub antecedent: Box<FlowNode<'a>>,
}

/** @internal */
pub struct FlowSwitchClause<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: FlowSwitchClauseData<'a>,
    pub antecedents: Box<FlowNode<'a>>,
}

/** @internal */
pub struct FlowSwitchClauseData<'a> {
    pub switchStatement: &'a SwitchStatement<'a>,
    pub clauseStart: usize, // Start index of case/default clause range
    pub clauseEnd: usize,   // End index of case/default clause range
}

// FlowArrayMutation represents a node potentially mutates an array, i.e. an
// operation of the form 'x.push(value)', 'x.unshift(value)' or 'x[n] = value'.
/** @internal */
pub struct FlowArrayMutation<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: FlowArrayMutationNode<'a>,
    pub antecedent: Box<FlowNode<'a>>,
}

pub enum FlowArrayMutationNode<'a> {
    CallExpression(&'a CallExpression<'a>),
    BinaryExpression(&'a BinaryExpression<'a>),
}

/** @internal */
pub struct FlowReduceLabel<'a> {
    pub flags: FlowFlags,
    pub id: usize,
    pub node: FlowReduceLabelData<'a>,
    pub antecedents: Box<FlowNode<'a>>,
}

/** @internal */
pub struct FlowReduceLabelData<'a> {
    pub target: FlowLabel<'a>,
    pub antecedents: Vec<FlowNode<'a>>,
}

pub enum FlowType<'a> {
    Complete(&'a dyn Type<'a>),
    Incomplete(IncompleteType<'a>),
}

// Incomplete types occur during control flow analysis of loops. An IncompleteType
// is distinguished from a regular type by a flags value of zero. Incomplete type
// objects are internal to the getFlowTypeOfReference function and never escape it.
pub struct IncompleteType<'a> {
    pub flags: TypeFlags,        // No flags set
    pub type_: &'a dyn Type<'a>, // The type marked incomplete
}
// endregion: 4250

// region: 4291
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    ESNext,
    CommonJS,
    Undefined,
}
// endregion: 4291

// region: 5010
/** @internal */
pub trait TypeCheckerHost: ModuleSpecifierResolutionHost + std::fmt::Debug {
    fn getCompilerOptions(&self) -> &CompilerOptions;
    fn getSourceFiles(&self) -> Vec<&SourceFile>;
    fn getSourceFile(&self, file_name: &str) -> Option<&SourceFile>;
    fn getProjectReferenceRedirect(&self, file_name: &str) -> Option<String>;
    fn isSourceOfProjectReferenceRedirect(&self, file_name: &str) -> bool;
    // fn getEmitSyntaxForUsageLocation(&self, file: &SourceFile, usage: &StringLiteralLike) -> ResolutionMode;
    // fn getRedirectReferenceForResolutionFromSourceOfProject(&self, file_path: Path) -> Option<ResolvedProjectReference>;
    // fn getModeForUsageLocation(&self, file: &SourceFile, usage: &StringLiteralLike) -> ResolutionMode;
    // fn getDefaultResolutionModeForFile(&self, source_file: &SourceFile) -> ResolutionMode;
    // fn getImpliedNodeFormatForEmit(&self, source_file: &SourceFile) -> ResolutionMode;
    // fn getEmitModuleFormatOfFile(&self, source_file: &SourceFile) -> ModuleKind;
    // fn getResolvedModule(&self, f: &SourceFile, module_name: &str, mode: ResolutionMode) -> Option<ResolvedModuleWithFailedLookupLocations>;
    // fn getRedirectTargetsMap(&self) -> &RedirectTargetsMap;
    fn typesPackageExists(&self, package_name: &str) -> bool;
    fn packageBundlesTypes(&self, package_name: &str) -> bool;
}

pub trait TypeCheckerTrait<'a>: std::fmt::Debug {
    fn getTypeOfSymbolAtLocation(&self, symbol: &Symbol<'a>, node: &AstKind) -> &dyn Type<'a>;
    fn getTypeOfSymbol(&self, symbol: &Symbol<'a>) -> &dyn Type<'a>;
    fn getDeclaredTypeOfSymbol(&self, symbol: &Symbol<'a>) -> &dyn Type<'a>;
    fn getPropertiesOfType(&self, type_: &dyn Type<'a>) -> Vec<&Symbol<'a>>;
    fn getPropertyOfType(&self, type_: &dyn Type<'a>, property_name: &str) -> Option<&Symbol<'a>>;
    fn getPrivateIdentifierPropertyOfType(&self, left_type: &dyn Type<'a>, name: &str, location: &AstKind) -> Option<&Symbol<'a>>;
    /** @internal */
    fn getTypeOfPropertyOfType(&self, type_: &dyn Type<'a>, propertyName: &str) -> Option<&dyn Type<'a>>;
    fn getIndexInfoOfType(&self, type_: &dyn Type<'a>, kind: IndexKind) -> Option<IndexInfo>;
    fn getIndexInfosOfType(&self, type_: &dyn Type<'a>) -> Vec<IndexInfo>;
    fn getIndexInfosOfIndexSymbol(&self, indexSymbol: Symbol<'a>) -> Vec<IndexInfo>;
    fn getSignaturesOfType(&self, type_: &dyn Type<'a>, kind: SignatureKind) -> Vec<&Signature<'a>>;
    fn getIndexTypeOfType(&self, type_: &dyn Type<'a>, kind: IndexKind) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getIndexType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    fn getBaseTypes(&self, type_: &dyn InterfaceType) -> Vec<BaseType>;
    fn getBaseTypeOfLiteralType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    fn getWidenedType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn getWidenedLiteralType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn getPromisedTypeOfPromise(&self, promise: &dyn Type<'a>, errorNode: Option<AstKind>) -> Option<&dyn Type<'a>>;
    /// Gets the "awaited type" of a type.
    ///
    /// If an expression has a Promise-like type, the "awaited type" of the expression is
    /// derived from the type of the first argument of the fulfillment callback for that
    /// Promise's then method. If the "awaited type" is itself a Promise-like, it is
    /// recursively unwrapped in the same manner until a non-promise type is found.
    ///
    /// If an expression does not have a Promise-like type, its "awaited type" is the type
    /// of the expression.
    ///
    /// If the resulting "awaited type" is a generic object type, then it is wrapped in
    /// an Awaited<T>.
    ///
    /// In the event the "awaited type" circularly references itself, or is a non-Promise
    /// object-type with a callable then() method, an "awaited type" cannot be determined
    /// and the value undefined will be returned.
    ///
    /// This is used to reflect the runtime behavior of the await keyword.
    fn getAwaitedType(&self, type_: &dyn Type<'a>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn isEmptyAnonymousObjectType(&self, type_: &dyn Type<'a>) -> bool;
    fn getReturnTypeOfSignature(&self, signature: Signature<'a>) -> &dyn Type<'a>;
    /// Gets the type of a parameter at a given position in a signature.
    /// Returns any if the index is not valid.
    ///
    /** @internal */
    fn getParameterType(&self, signature: Signature<'a>, parameter_index: usize) -> &dyn Type<'a>;
    /** @internal */
    fn getParameterIdentifierInfoAtPosition(&self, signature: Signature<'a>, parameter_index: usize) -> Option<(Identifier, &str, bool)>;
    fn getNullableType(&self, type_: &dyn Type<'a>, flags: TypeFlags) -> &dyn Type<'a>;
    fn getNonNullableType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn getNonOptionalType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn isNullableType(&self, type_: &dyn Type<'a>) -> bool;
    fn getTypeArguments(&self, type_: TypeReference) -> Vec<&dyn Type<'a>>;

    // TODO: GH#18217 `xToDeclaration` calls are frequently asserted as defined.
    /// Note that the resulting nodes cannot be checked.
    // fn typeToTypeNode(&self, type_: Type, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<TypeNode>;
    /** @internal */
    // fn typeToTypeNodeWithInternalFlags(&self, type_: Type, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>, internalFlags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<TypeNode>;
    /** @internal */
    // fn typePredicateToTypePredicateNode(&self, typePredicate: TypePredicate, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>, internalFlags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<TypePredicateNode>;
    /// Note that the resulting nodes cannot be checked.
    // fn signatureToSignatureDeclaration(&self, signature: Signature, kind: SyntaxKind, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<SignatureDeclaration>;
    /** @internal */
    // fn signatureToSignatureDeclarationWithInternalFlags(&self, signature: Signature, kind: SyntaxKind, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>, internalFlags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<SignatureDeclaration>;
    /// Note that the resulting nodes cannot be checked.
    // fn indexInfoToIndexSignatureDeclaration(&self, indexInfo: IndexInfo, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<IndexSignatureDeclaration>;
    /** @internal */
    // fn indexInfoToIndexSignatureDeclarationWithInternalFlags(&self, indexInfo: IndexInfo, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>, internalFlags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<IndexSignatureDeclaration>;
    /// Note that the resulting nodes cannot be checked.
    // fn symbolToEntityName(&self, symbol: Symbol, meaning: SymbolFlags, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<EntityName>;
    /// Note that the resulting nodes cannot be checked.
    // fn symbolToExpression(&self, symbol: Symbol, meaning: SymbolFlags, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<Expression>;
    /// Note that the resulting nodes cannot be checked.
    ///
    /** @internal */
    // fn symbolToNode(&self, symbol: Symbol, meaning: SymbolFlags, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>, internalFlags: Option<InternalNodeBuilderFlags>) -> Option<Node>;
    /// Note that the resulting nodes cannot be checked.
    // fn symbolToTypeParameterDeclarations(&self, symbol: Symbol, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<Vec<TSTypeParameterDeclaration>>;
    /// Note that the resulting nodes cannot be checked.
    // fn symbolToParameterDeclaration(&self, symbol: Symbol, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<Argument>;
    /// Note that the resulting nodes cannot be checked.
    // fn typeParameterToDeclaration(&self, parameter: TypeParameter, enclosingDeclaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<TSTypeParameterDeclaration>;

    fn getSymbolsInScope(&self, location: AstKind, meaning: SymbolFlags) -> Vec<Symbol>;
    fn getSymbolAtLocation(&self, node: AstKind) -> Option<Symbol>;
    /** @internal */
    fn getIndexInfosAtLocation(&self, node: AstKind) -> Option<Vec<IndexInfo>>;
    fn getSymbolsOfParameterPropertyDeclaration(&self, parameter: Argument, parameter_name: &str) -> Vec<Symbol>;
    /// The function returns the value (local variable) symbol of an identifier in the short-hand property assignment.
    /// This is necessary as an identifier in short-hand property assignment can contains two meaning: property name and property value.
    fn getShorthandAssignmentValueSymbol(&self, location: Option<AstKind>) -> Option<Symbol>;

    fn getExportSpecifierLocalTargetSymbol(&self, location: ExportSpecifier) -> Option<Symbol>;
    /// If a symbol is a local symbol with an associated exported symbol, returns the exported symbol.
    /// Otherwise returns its input.
    /// For example, at `export type T = number;`:
    ///     - `get_symbol_at_location` at the location `T` will return the exported symbol for `T`.
    ///     - But the result of `get_symbols_in_scope` will contain the *local* symbol for `T`, not the exported symbol.
    ///     - Calling `get_export_symbol_of_symbol` on that local symbol will return the exported symbol.
    fn getExportSymbolOfSymbol(&self, symbol: Symbol<'a>) -> Symbol<'a>;
    fn getPropertySymbolOfDestructuringAssignment(&self, location: Identifier) -> Option<Symbol>;
    fn getTypeOfAssignmentPattern(&self, pattern: AssignmentPattern) -> &dyn Type<'a>;
    fn getTypeAtLocation(&self, node: AstKind) -> &dyn Type<'a>;
    fn getTypeFromTypeNode(&self, node: TypeNode) -> &dyn Type<'a>;

    fn signatureToString(&self, signature: Signature<'a>, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>) -> String;
    fn typeToString(&self, type_: &dyn Type<'a>, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>) -> String;
    fn symbolToString(&self, symbol: Symbol<'a>, enclosingDeclaration: Option<AstKind>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>) -> String;
    fn typePredicateToString(&self, predicate: TypePredicate, enclosing_declaration: Option<AstKind>, flags: Option<TypeFormatFlags>) -> String;

    /** @internal */
    fn writeSignature(&self, signature: Signature<'a>, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>, writer: Option<EmitTextWriter>) -> String;
    /** @internal */
    fn writeType(&self, type_: &dyn Type<'a>, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String;
    /** @internal */
    fn writeSymbol(&self, symbol: Symbol<'a>, enclosingDeclaration: Option<AstKind>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>, writer: Option<EmitTextWriter>) -> String;
    /** @internal */
    fn writeTypePredicate(&self, predicate: TypePredicate, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String;

    fn getFullyQualifiedName(&self, symbol: Symbol<'a>) -> String;
    fn getAugmentedPropertiesOfType(&self, type_: &dyn Type<'a>) -> Vec<&Symbol<'a>>;

    fn getRootSymbols(&self, symbol: Symbol<'a>) -> Vec<Symbol<'a>>;
    fn getSymbolOfExpando(&self, node: AstKind, allowDeclaration: bool) -> Option<Symbol<'a>>;
    fn getContextualType(&self, node: Expression<'a>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getContextualTypeWithFlags(&self, node: Expression<'a>, contextFlags: Option<ContextFlags>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getContextualTypeForObjectLiteralElement(&self, element: ObjectLiteralElementLike<'a>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getContextualTypeForArgumentAtIndex(&self, call: CallLikeExpression<'a>, argIndex: usize) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getContextualTypeForJsxAttribute(&self, attribute: JSXAttribute) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn isContextSensitive(&self, node: Expression<'a>) -> bool;
    /** @internal */
    fn getTypeOfPropertyOfContextualType(&self, type_: &dyn Type<'a>, name: &str) -> Option<&dyn Type<'a>>;

    /// returns unknownSignature in the case of an error.
    /// returns undefined if the node is not valid.
    /// @param argument_count Apparent number of arguments, passed in case of a possibly incomplete call. This should come from an ArgumentListInfo. See `signatureHelp.ts`.
    fn getResolvedSignature(&self, node: CallLikeExpression<'a>, candidatesOutArray: Option<Vec<Signature>>, argumentCount: Option<usize>) -> Option<Signature<'a>>;
    /** @internal */
    fn getResolvedSignatureForSignatureHelp(&self, node: CallLikeExpression<'a>, candidatesOutArray: Option<Vec<Signature>>, argumentCount: Option<usize>) -> Option<Signature<'a>>;
    /** @internal */
    fn getCandidateSignaturesForStringLiteralCompletions(&self, call: CallLikeExpression<'a>, editingArgument: AstKind) -> Vec<Signature<'a>>;
    /** @internal */
    fn getExpandedParameters(&self, sig: Signature<'a>) -> Vec<Vec<Symbol<'a>>>;
    /** @internal */
    fn hasEffectiveRestParameter(&self, sig: Signature<'a>) -> bool;
    /** @internal */
    fn containsArgumentsReference(&self, declaration: SignatureDeclaration<'a>) -> bool;

    fn getSignatureFromDeclaration(&self, declaration: SignatureDeclaration<'a>) -> Option<Signature<'a>>;
    fn isImplementationOfOverload(&self, node: SignatureDeclaration<'a>) -> Option<bool>;
    fn isUndefinedSymbol(&self, symbol: Symbol<'a>) -> bool;
    fn isArgumentsSymbol(&self, symbol: Symbol<'a>) -> bool;
    fn isUnknownSymbol(&self, symbol: Symbol<'a>) -> bool;
    fn getMergedSymbol(&self, symbol: Symbol<'a>) -> Symbol<'a>;
    /** @internal */
    fn symbolIsValue(&self, symbol: Symbol<'a>, includeTypeOnlyMembers: Option<bool>) -> bool;

    fn getConstantValue(&self, node: EnumMember) -> Option<String>;
    fn isValidPropertyAccess(&self, node: PropertyAccessExpression, propertyName: &str) -> bool;
    /// Exclude accesses to private properties.
    ///
    /** @internal */
    fn isValidPropertyAccessForCompletions(&self, node: PropertyAccessExpression, type_: &dyn Type<'a>, property: Symbol) -> bool;
    /// Follow all aliases to get the original symbol.
    fn getAliasedSymbol(&self, symbol: Symbol<'a>) -> Symbol<'a>;
    /// Follow a *single* alias to get the immediately aliased symbol.
    fn getImmediateAliasedSymbol(&self, symbol: Symbol<'a>) -> Option<Symbol<'a>>;
    fn getExportsOfModule(&self, moduleSymbol: Symbol<'a>) -> Vec<Symbol<'a>>;
    /// Unlike `get_exports_of_module`, this includes properties of an `export =` value.
    ///
    /** @internal */
    fn getExportsAndPropertiesOfModule(&self, moduleSymbol: Symbol<'a>) -> Vec<Symbol<'a>>;
    /** @internal */
    // fn forEachExportAndPropertyOfModule(&self, moduleSymbol: Symbol, cb: impl Fn(Symbol, &str));
    fn getJsxIntrinsicTagNamesAt(&self, location: AstKind) -> Vec<Symbol<'a>>;
    fn isOptionalParameter(&self, node: Argument) -> bool;
    fn getAmbientModules(&self) -> Vec<Symbol<'a>>;

    fn tryGetMemberInModuleExports(&self, memberName: &str, moduleSymbol: Symbol<'a>) -> Option<Symbol<'a>>;
    /// Unlike `try_get_member_in_module_exports`, this includes properties of an `export =` value.
    /// Does *not* return properties of primitive types.
    ///
    /** @internal */
    fn tryGetMemberInModuleExportsAndProperties(&self, memberName: &str, moduleSymbol: Symbol<'a>) -> Option<Symbol<'a>>;
    fn getApparentType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentProperty(&self, name: MemberName, containingType: &dyn Type<'a>) -> Option<Symbol<'a>>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentJsxAttribute(&self, name: Identifier, containingType: &dyn Type<'a>) -> Option<Symbol<'a>>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentSymbol(&self, location: AstKind, name: &str, meaning: SymbolFlags) -> Option<Symbol<'a>>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentModule(&self, node: Identifier, target: Symbol<'a>) -> Option<Symbol<'a>>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentClassMember(&self, name: &str, baseType: &dyn Type<'a>) -> Option<Symbol<'a>>;
    fn getBaseConstraintOfType(&self, type_: &dyn Type<'a>) -> Option<&dyn Type<'a>>;
    fn getDefaultFromTypeParameter(&self, type_: &dyn Type<'a>) -> Option<&dyn Type<'a>>;

    /// Gets the intrinsic `any` type. There are multiple types that act as `any` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `any`. Instead, use `type.flags & TypeFlags.Any`.
    fn getAnyType(&self) -> &dyn Type<'a>;
    fn getStringType(&self) -> &dyn Type<'a>;
    fn getStringLiteralType(&self, value: &str) -> StringLiteralType;
    fn getNumberType(&self) -> &dyn Type<'a>;
    fn getNumberLiteralType(&self, value: f64) -> NumberLiteralType;
    fn getBigIntType(&self) -> &dyn Type<'a>;
    fn getBigIntLiteralType(&self, value: PseudoBigInt) -> BigIntLiteralType;
    fn getBooleanType(&self) -> &dyn Type<'a>;
    /** @internal */
    fn getFalseType(&self, fresh: Option<bool>) -> &dyn Type<'a>;
    /** @internal */
    fn getTrueType(&self, fresh: Option<bool>) -> &dyn Type<'a>;
    fn getVoidType(&self) -> &dyn Type<'a>;
    /// Gets the intrinsic `undefined` type. There are multiple types that act as `undefined` used internally in the compiler
    /// depending on compiler options, so the type returned by this function should not be used in equality checks to determine
    /// if another type is `undefined`. Instead, use `type.flags & TypeFlags.Undefined`.
    fn getUndefinedType(&self) -> &dyn Type<'a>;
    /// Gets the intrinsic `null` type. There are multiple types that act as `null` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `null`. Instead, use `type.flags & TypeFlags.Null`.
    fn getNullType(&self) -> &dyn Type<'a>;
    fn getESSymbolType(&self) -> &dyn Type<'a>;
    /// Gets the intrinsic `never` type. There are multiple types that act as `never` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `never`. Instead, use `type.flags & TypeFlags.Never`.
    fn getNeverType(&self) -> &dyn Type<'a>;
    /** @internal */
    fn getOptionalType(&self) -> &dyn Type<'a>;
    /** @internal */
    fn getUnionType(&self, types: Vec<&dyn Type<'a>>, subtypeReduction: Option<UnionReduction>) -> &dyn Type<'a>;
    /** @internal */
    fn createArrayType(&self, elementType: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn getElementTypeOfArrayType(&self, arrayType: &dyn Type<'a>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn createPromiseType(&self, type_: &dyn Type<'a>) -> &dyn Type<'a>;
    /** @internal */
    fn getPromiseType(&self) -> &dyn Type<'a>;
    /** @internal */
    fn getPromiseLikeType(&self) -> &dyn Type<'a>;
    /** @internal */
    fn getAnyAsyncIterableType(&self) -> Option<&dyn Type<'a>>;

    /// Returns true if the "source" type is assignable to the "target" type.
    fn isTypeAssignableTo(&self, source: &dyn Type<'a>, target: &dyn Type<'a>) -> bool;
    /** @internal */
    fn createAnonymousType(&self, symbol: Option<Symbol<'a>>, members: SymbolTable, callSignatures: Vec<Signature<'a>>, constructSignatures: Vec<Signature<'a>>, indexInfos: Vec<IndexInfo>) -> &dyn Type<'a>;
    /** @internal */
    fn createSignature(
        &self, declaration: Option<SignatureDeclaration<'a>>, typeParameters: Option<Vec<TypeParameter>>, thisParameter: Option<Symbol>, parameters: Vec<Symbol>, resolvedReturnType: &dyn Type<'a>, typePredicate: Option<TypePredicate>, minArgumentCount: usize, flags: SignatureFlags,
    ) -> Signature<'a>;
    /** @internal */
    fn createSymbol(&self, flags: SymbolFlags, name: &str) -> TransientSymbol;
    /** @internal */
    fn createIndexInfo(&self, keyType: &dyn Type<'a>, type_: &dyn Type<'a>, isReadonly: bool, declaration: Option<SignatureDeclaration<'a>>) -> IndexInfo;
    /** @internal */
    fn isSymbolAccessible(&self, symbol: Symbol<'a>, enclosingDeclaration: Option<AstKind>, meaning: SymbolFlags, shouldComputeAliasToMarkVisible: bool) -> SymbolAccessibilityResult;
    /** @internal */
    fn tryFindAmbientModule(&self, moduleName: &str) -> Option<Symbol<'a>>;

    /** @internal */
    fn getSymbolWalker(&self, accept: Option<fn(Symbol<'a>) -> bool>) -> SymbolWalker;

    // Should not be called directly.  Should only be accessed through the Program instance.
    /** @internal */
    fn getDiagnostics(&self, sourceFile: Option<&SourceFile<'a>>, cancellationToken: Option<CancellationToken>, nodesToCheck: Option<Vec<AstKind>>) -> Vec<Diagnostic>;
    /** @internal */
    fn getGlobalDiagnostics(&self) -> Vec<Diagnostic>;
    /** @internal */
    fn getEmitResolver(&self, sourceFile: Option<&SourceFile<'a>>, cancellationToken: Option<CancellationToken>, forceDts: Option<bool>) -> EmitResolver;
    /** @internal */
    fn requiresAddingImplicitUndefined(&self, parameter: Argument<'a>, enclosingDeclaration: Option<AstKind>) -> bool;

    /** @internal */
    fn getNodeCount(&self) -> usize;
    /** @internal */
    fn getIdentifierCount(&self) -> usize;
    /** @internal */
    fn getSymbolCount(&self) -> usize;
    /** @internal */
    fn getTypeCount(&self) -> usize;
    /** @internal */
    fn getInstantiationCount(&self) -> usize;
    /** @internal */
    fn getRelationCacheSizes(&self) -> (usize, usize, usize, usize);
    /** @internal */
    fn getRecursionIdentity(&self, type_: &dyn Type<'a>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getUnmatchedProperties(&self, source: &dyn Type<'a>, target: &dyn Type<'a>, requireOptionalProperties: bool, matchDiscriminantProperties: bool) -> Box<dyn Iterator<Item = Symbol<'a>>>;

    /// True if this type is the `Array` or `ReadonlyArray` type from lib.d.ts.
    /// This function will _not_ return true if passed a type which
    /// extends `Array` (for example, the TypeScript AST's `NodeArray` type).
    fn isArrayType(&self, type_: &dyn Type<'a>) -> bool;
    /// True if this type is a tuple type. This function will _not_ return true if
    /// passed a type which extends from a tuple.
    fn isTupleType(&self, type_: &dyn Type<'a>) -> bool;
    /// True if this type is assignable to `ReadonlyArray<any>`.
    fn isArrayLikeType(&self, type_: &dyn Type<'a>) -> bool;

    /// True if `contextualType` should not be considered for completions because
    /// e.g. it specifies `kind: "a"` and obj has `kind: "b"`.
    ///
    /** @internal */
    fn isTypeInvalidDueToUnionDiscriminant(&self, contextualType: &dyn Type<'a>, obj: ObjectExpression<'a>) -> bool;
    /** @internal */
    fn getExactOptionalProperties(&self, type_: &dyn Type<'a>) -> Vec<Symbol<'a>>;
    /// For a union, will include a property if it's defined in *any* of the member types.
    /// So for `{ a } | { b }`, this will include both `a` and `b`.
    /// Does not include properties of primitive types.
    ///
    /** @internal */
    fn getAllPossiblePropertiesOfTypes(&self, types: Vec<&dyn Type<'a>>) -> Vec<Symbol<'a>>;
    fn resolveName(&self, name: &str, location: Option<AstKind>, meaning: SymbolFlags, excludeGlobals: bool) -> Option<Symbol<'a>>;
    /** @internal */
    fn getJsxNamespace(&self, location: Option<AstKind>) -> String;
    /** @internal */
    fn getJsxFragmentFactory(&self, location: AstKind) -> Option<String>;

    /// Note that this will return undefined in the following case:
    ///     // a.ts
    ///     export namespace N { export class C { } }
    ///     // b.ts
    ///     <<enclosingDeclaration>>
    /// Where `C` is the symbol we're looking for.
    /// This should be called in a loop climbing parents of the symbol, so we'll get `N`.
    ///
    /** @internal */
    fn getAccessibleSymbolChain(&self, symbol: Symbol<'a>, enclosingDeclaration: Option<AstKind>, meaning: SymbolFlags, useOnlyExternalAliasing: bool) -> Option<Vec<Symbol<'a>>>;
    fn getTypePredicateOfSignature(&self, signature: Signature<'a>) -> Option<TypePredicate>;
    /** @internal */
    fn resolveExternalModuleName(&self, moduleSpecifier: Expression<'a>) -> Option<Symbol<'a>>;
    /// An external module with an 'export =' declaration resolves to the target of the 'export =' declaration,
    /// and an external module with no 'export =' declaration resolves to the module itself.
    ///
    /** @internal */
    fn resolveExternalModuleSymbol(&self, symbol: Symbol<'a>) -> Symbol<'a>;
    /// @param node A location where we might consider accessing `this`. Not necessarily a ThisExpression.
    ///
    /** @internal */
    fn tryGetThisTypeAt(&self, node: AstKind, includeGlobalThis: Option<bool>, container: Option<ThisContainer>) -> Option<&dyn Type<'a>>;
    /** @internal */
    fn getTypeArgumentConstraint(&self, node: TypeNode) -> Option<&dyn Type<'a>>;

    /// Does *not* get *all* suggestion diagnostics, just the ones that were convenient to report in the checker.
    /// Others are added in computeSuggestionDiagnostics.
    ///
    /** @internal */
    fn getSuggestionDiagnostics(&self, file: SourceFile, cancellationToken: Option<CancellationToken>) -> Vec<DiagnosticWithLocation>;

    /// Depending on the operation performed, it may be appropriate to throw away the checker
    /// if the cancellation token is triggered. Typically, if it is used for error checking
    /// and the operation is cancelled, then it should be discarded, otherwise it is safe to keep.
    /** @internal */ // `token = None`
    // fn run_with_cancellationToken<T>(&self, token: Option<CancellationToken>, cb: impl Fn() -> T) -> T;

    /** @internal */
    fn getLocalTypeParametersOfClassOrInterfaceOrTypeAlias(&self, symbol: Symbol<'a>) -> Option<Vec<TypeParameter>>;
    /** @internal */
    fn isDeclarationVisible(&self, node: Declaration<'a>) -> bool;
    /** @internal */
    fn isPropertyAccessible(&self, node: AstKind, isSuper: bool, isWrite: bool, containingType: &dyn Type<'a>, property: Symbol<'a>) -> bool;
    /** @internal */
    fn getTypeOnlyAliasDeclaration(&self, symbol: Symbol<'a>) -> Option<TypeOnlyAliasDeclaration>;
    /** @internal */
    fn getMemberOverrideModifierStatus(&self, node: ClassLikeDeclaration, member: ClassElement, memberSymbol: Symbol<'a>) -> MemberOverrideStatus;
    /** @internal */
    fn isTypeParameterPossiblyReferenced(&self, tp: TypeParameter, node: AstKind) -> bool;
    /** @internal */
    fn typeHasCallOrConstructSignatures(&self, type_: &dyn Type<'a>) -> bool;
    /** @internal */
    fn getSymbolFlags(&self, symbol: Symbol<'a>) -> SymbolFlags;
}
// endregion: 5416

// region: 5864
define_flags!(SymbolFlags {
    None = 0,
    FunctionScopedVariable= 1 << 0, // Variable (var) or parameter
    BlockScopedVariable = 1 << 1, // A block-scoped variable (let or const)
    Property = 1 << 2, // Property or enum member
    EnumMember = 1 << 3, // Enum member
    Function = 1 << 4, // Function
    Class = 1 << 5, // Class
    Interface = 1 << 6, // Interface
    ConstEnum = 1 << 7, // Const enum
    RegularEnum = 1 << 8, // Enum
    ValueModule = 1 << 9, // Instantiated module
    NamespaceModule = 1 << 10, // Uninstantiated module
    TypeLiteral = 1 << 11, // Type Literal or mapped type
    ObjectLiteral = 1 << 12, // Object Literal
    Method = 1 << 13, // Method
    Constructor = 1 << 14, // Constructor
    GetAccessor = 1 << 15, // Get accessor
    SetAccessor = 1 << 16, // Set accessor
    Signature = 1 << 17, // Call, construct, or index signature
    TypeParameter = 1 << 18, // Type parameter
    TypeAlias = 1 << 19, // Type alias
    ExportValue = 1 << 20, // Exported value marker (see comment in declareModuleMember in binder)
    Alias = 1 << 21, // An alias for another symbol (see comment in isAliasSymbolDeclaration in checker)
    Prototype = 1 << 22, // Prototype property (no source representation)
    ExportStar = 1 << 23, // Export * declaration
    Optional = 1 << 24, // Optional property
    Transient = 1 << 25, // Transient symbol (created during type check)
    Assignment = 1 << 26, // Assignment treated as declaration (eg `this.prop = 1`)
    ModuleExports = 1 << 27, // Symbol for CommonJS `module` of `module.exports`
    All = u64::MAX,

    Enum = Self::RegularEnum.0 | Self::ConstEnum.0,
    Variable = Self::FunctionScopedVariable.0 | Self::BlockScopedVariable.0,
    Value = Self::Variable.0 | Self::Property.0 | Self::EnumMember.0 | Self::ObjectLiteral.0 | Self::Function.0 | Self::Class.0 | Self::Enum.0 | Self::ValueModule.0 | Self::Method.0 | Self::GetAccessor.0 | Self::SetAccessor.0,
    Type = Self::Class.0 | Self::Interface.0 | Self::Enum.0 | Self::EnumMember.0 | Self::TypeLiteral.0 | Self::TypeParameter.0 | Self::TypeAlias.0,
    Namespace = Self::ValueModule.0 | Self::NamespaceModule.0 | Self::Enum.0,
    Module = Self::ValueModule.0 | Self::NamespaceModule.0,
    Accessor = Self::GetAccessor.0 | Self::SetAccessor.0,

    // Variables can be redeclared, but can not redeclare a block-scoped declaration with the
    // same name, or any other value that is not a variable, e.g. ValueModule or Class
    FunctionScopedVariableExcludes = Self::Value.0 & !Self::FunctionScopedVariable.0,

    // Block-scoped declarations are not allowed to be re-declared
    // they can not merge with anything in the value space
    BlockScopedVariableExcludes = Self::Value.0,

    ParameterExcludes = Self::Value.0,
    PropertyExcludes = Self::None.0,
    EnumMemberExcludes = Self::Value.0 | Self::Type.0,
    FunctionExcludes = Self::Value.0 & !(Self::Function.0 | Self::ValueModule.0 | Self::Class.0),
    ClassExcludes = (Self::Value.0 | Self::Type.0) & !(Self::ValueModule.0 | Self::Interface.0 | Self::Function.0), // class-interface mergability done in checker.ts
    InterfaceExcludes = Self::Type.0 & !(Self::Interface.0 | Self::Class.0),
    RegularEnumExcludes = (Self::Value.0 | Self::Type.0) & !(Self::RegularEnum.0 | Self::ValueModule.0), // regular enums merge only with regular enums and modules
    ConstEnumExcludes = (Self::Value.0 | Self::Type.0) & !Self::ConstEnum.0, // const enums merge only with const enums
    ValueModuleExcludes = Self::Value.0 & !(Self::Function.0 | Self::Class.0 | Self::RegularEnum.0 | Self::ValueModule.0),
    NamespaceModuleExcludes = Self::None.0,
    MethodExcludes = Self::Value.0 & !Self::Method.0,
    GetAccessorExcludes = Self::Value.0 & !Self::SetAccessor.0,
    SetAccessorExcludes = Self::Value.0 & !Self::GetAccessor.0,
    AccessorExcludes = Self::Value.0 & !Self::Accessor.0,
    TypeParameterExcludes = Self::Type.0 & !Self::TypeParameter.0,
    TypeAliasExcludes = Self::Type.0,
    AliasExcludes = Self::Alias.0,

    ModuleMember = Self::Variable.0 | Self::Function.0 | Self::Class.0 | Self::Interface.0 | Self::Enum.0 | Self::Module.0 | Self::TypeAlias.0 | Self::Alias.0,

    ExportHasLocal = Self::Function.0 | Self::Class.0 | Self::Enum.0 | Self::ValueModule.0,

    BlockScoped = Self::BlockScopedVariable.0 | Self::Class.0 | Self::Enum.0,

    PropertyOrAccessor = Self::Property.0 | Self::Accessor.0,

    ClassMember = Self::Method.0 | Self::Accessor.0 | Self::Property.0,

    // @internal
    ExportSupportsDefaultModifier = Self::Class.0 | Self::Function.0 | Self::Interface.0,

    // @internal
    ExportDoesNotSupportDefaultModifier = !Self::ExportSupportsDefaultModifier.0,

    // @internal
    // The set of things we consider semantically classifiable.  Used to speed up the LS during
    // classification.
    Classifiable = Self::Class.0 | Self::Enum.0 | Self::TypeAlias.0 | Self::Interface.0 | Self::TypeParameter.0 | Self::Module.0 | Self::Alias.0,

    // @internal
    LateBindingContainer = Self::Class.0 | Self::Interface.0 | Self::TypeLiteral.0 | Self::ObjectLiteral.0 | Self::Function.0,

});

/** @internal */
pub type SymbolId = usize;

#[derive(Debug, Clone)]
#[rustfmt::skip]
pub struct Symbol<'a> {
    pub flags: SymbolFlags,                     // Symbol flags
    pub escapedName: __String,                  // Name of symbol
    pub declarations: Option<Vec<&'a Declaration<'a>>>, // Declarations associated with this symbol
    pub valueDeclaration: Option<&'a Declaration<'a>>,  // First value declaration of the symbol
    pub members: Option<SymbolTable<'a>>,           // Class, interface or object literal instance members
    pub exports: Option<SymbolTable<'a>>,           // Module exports
    pub globalExports: Option<SymbolTable<'a>>,     // Conditional global UMD exports
    /** @internal */ pub id: SymbolId,          // Unique id (used to look up SymbolLinks)
    /** @internal */ pub mergeId: usize,        // Merge id (used to look up merged symbol)
    /** @internal */ pub parent: opt_rc_cell!(Symbol<'a>),         // Parent symbol
    /** @internal */ pub exportSymbol: opt_rc_cell!(Symbol<'a>),   // Exported symbol associated with this symbol
    /** @internal */ pub constEnumOnlyModule: Option<bool>,   // True if module contains only const enums or other modules with only const enums
    /** @internal */ pub isReferenced: Option<SymbolFlags>,   // True if the symbol is referenced elsewhere. Keeps track of the meaning of a reference in case a symbol is both a type parameter and parameter.
    /** @internal */ pub lastAssignmentPos: Option<usize>,    // Source position of last node that assigns value to symbol
    /** @internal */ pub isReplaceableByMethod: Option<bool>, // Can this Javascript class property be replaced by a method symbol?
    /** @internal */ pub assignmentDeclarationMembers: Option<HashMap<usize, &'a Declaration<'a>>>, // detected late-bound assignment declarations associated with the symbol
}
// endregion: 5976

// region: 6089
define_string_enum! {
    InternalSymbolName {
        Call => "__call",          // Call signatures
        Constructor => "__constructor", // Constructor implementations
        New => "__new",            // Constructor signatures
        Index => "__index",        // Index signatures
        ExportStar => "__export",  // Module export * declarations
        Global => "__global",      // Global self-reference
        Missing => "__missing",    // Indicates missing symbol
        Type => "__type",          // Anonymous type literal symbol
        Object => "__object",      // Anonymous object literal declaration
        JSXAttributes => "__jsxAttributes", // Anonymous JSX attributes object literal declaration
        Class => "__class",        // Unnamed class expression
        Function => "__function",  // Unnamed function expression
        Computed => "__computed",  // Computed property name declaration with dynamic name
        Resolving => "__resolving__", // Indicator symbol used to mark partially resolved type aliases
        ExportEquals => "export=", // Export assignment symbol
        Default => "default",      // Default export symbol (technically not wholly internal, but included here for usability)
        This => "this",
        InstantiationExpression => "__instantiationExpression", // Instantiation expressions
        ImportAttributes => "__importAttributes",
    }
}

// /**
//  * This represents a string whose leading underscore have been escaped by adding extra leading underscores.
//  * The shape of this brand is rather unique compared to others we've used.
//  * Instead of just an intersection of a string and an object, it is that union-ed
//  * with an intersection of void and an object. This makes it wholly incompatible
//  * with a normal string (which is good, it cannot be misused on assignment or on usage),
//  * while still being comparable with a normal string via === (also good) and castable from a string.
//  */
// export type __String = (string & { __escapedIdentifier: void; }) | (void & { __escapedIdentifier: void; }) | InternalSymbolName;
pub type __String = String;

// /** @deprecated Use ReadonlyMap<__String, T> instead. */
// export type ReadonlyUnderscoreEscapedMap<T> = ReadonlyMap<__String, T>;

// /** @deprecated Use Map<__String, T> instead. */
// export type UnderscoreEscapedMap<T> = Map<__String, T>;

/** SymbolTable based on ES6 Map interface. */
pub type SymbolTable<'a> = HashMap<__String, rc_cell!(Symbol<'a>)>;
// endregion: 6128

// region: 6246
define_flags!(TypeFlags {
    Any = 1 << 0,
    Unknown = 1 << 1,
    String = 1 << 2,
    Number = 1 << 3,
    Boolean = 1 << 4,
    Enum = 1 << 5, // Numeric computed enum member value
    BigInt = 1 << 6,
    StringLiteral = 1 << 7,
    NumberLiteral = 1 << 8,
    BooleanLiteral = 1 << 9,
    EnumLiteral = 1 << 10, // Always combined with StringLiteral, NumberLiteral, or Union
    BigIntLiteral = 1 << 11,
    ESSymbol = 1 << 12, // Type of symbol primitive introduced in ES6
    UniqueESSymbol = 1 << 13, // unique symbol
    Void = 1 << 14,
    Undefined = 1 << 15,
    Null = 1 << 16,
    Never = 1 << 17, // Never type
    TypeParameter = 1 << 18, // Type parameter
    Object = 1 << 19, // Object type
    Union = 1 << 20, // Union (T | U)
    Intersection = 1 << 21, // Intersection (T & U)
    Index = 1 << 22, // keyof T
    IndexedAccess = 1 << 23, // T[K]
    Conditional = 1 << 24, // T extends U ? X : Y
    Substitution = 1 << 25, // Type parameter substitution
    NonPrimitive = 1 << 26, // intrinsic object type
    TemplateLiteral = 1 << 27, // Template literal type
    StringMapping = 1 << 28, // Uppercase/Lowercase type
    // @internal
    Reserved1 = 1 << 29, // Used by union/intersection type construction
    // @internal
    Reserved2 = 1 << 30, // Used by union/intersection type construction

    // @internal
    AnyOrUnknown = TypeFlags::Any.0 | TypeFlags::Unknown.0,
    // @internal
    Nullable = TypeFlags::Undefined.0 | TypeFlags::Null.0,
    Literal = TypeFlags::StringLiteral.0 | TypeFlags::NumberLiteral.0 | TypeFlags::BigIntLiteral.0 | TypeFlags::BooleanLiteral.0,
    Unit = TypeFlags::Enum.0 | TypeFlags::Literal.0 | TypeFlags::UniqueESSymbol.0 | TypeFlags::Nullable.0,
    Freshable = TypeFlags::Enum.0 | TypeFlags::Literal.0,
    StringOrNumberLiteral = TypeFlags::StringLiteral.0 | TypeFlags::NumberLiteral.0,
    // @internal
    StringOrNumberLiteralOrUnique = TypeFlags::StringLiteral.0 | TypeFlags::NumberLiteral.0 | TypeFlags::UniqueESSymbol.0,
    // @internal
    DefinitelyFalsy = TypeFlags::StringLiteral.0 | TypeFlags::NumberLiteral.0 | TypeFlags::BigIntLiteral.0 | TypeFlags::BooleanLiteral.0 | TypeFlags::Void.0 | TypeFlags::Undefined.0 | TypeFlags::Null.0,
    PossiblyFalsy = TypeFlags::DefinitelyFalsy.0 | TypeFlags::String.0 | TypeFlags::Number.0 | TypeFlags::BigInt.0 | TypeFlags::Boolean.0,
    // @internal
    Intrinsic = TypeFlags::Any.0 | TypeFlags::Unknown.0 | TypeFlags::String.0 | TypeFlags::Number.0 | TypeFlags::BigInt.0 | TypeFlags::Boolean.0 | TypeFlags::BooleanLiteral.0 | TypeFlags::ESSymbol.0 | TypeFlags::Void.0 | TypeFlags::Undefined.0 | TypeFlags::Null.0 | TypeFlags::Never.0 | TypeFlags::NonPrimitive.0,
    StringLike = TypeFlags::String.0 | TypeFlags::StringLiteral.0 | TypeFlags::TemplateLiteral.0 | TypeFlags::StringMapping.0,
    NumberLike = TypeFlags::Number.0 | TypeFlags::NumberLiteral.0 | TypeFlags::Enum.0,
    BigIntLiteralLike = TypeFlags::BigInt.0 | TypeFlags::BigIntLiteral.0,
    BooleanLike = TypeFlags::Boolean.0 | TypeFlags::BooleanLiteral.0,
    EnumLike = TypeFlags::Enum.0 | TypeFlags::EnumLiteral.0,
    ESSymbolLike = TypeFlags::ESSymbol.0 | TypeFlags::UniqueESSymbol.0,
    VoidLike = TypeFlags::Void.0 | TypeFlags::Undefined.0,
    // @internal
    Primitive = TypeFlags::StringLike.0 | TypeFlags::NumberLike.0 | TypeFlags::BigIntLiteralLike.0 | TypeFlags::BooleanLike.0 | TypeFlags::EnumLike.0 | TypeFlags::ESSymbolLike.0 | TypeFlags::VoidLike.0 | TypeFlags::Null.0,
    // @internal
    DefinitelyNonNullable = TypeFlags::StringLike.0 | TypeFlags::NumberLike.0 | TypeFlags::BigIntLiteralLike.0 | TypeFlags::BooleanLike.0 | TypeFlags::EnumLike.0 | TypeFlags::ESSymbolLike.0 | TypeFlags::Object.0 | TypeFlags::NonPrimitive.0,
    // @internal
    DisjointDomains = TypeFlags::NonPrimitive.0 | TypeFlags::StringLike.0 | TypeFlags::NumberLike.0 | TypeFlags::BigIntLiteralLike.0 | TypeFlags::BooleanLike.0 | TypeFlags::ESSymbolLike.0 | TypeFlags::VoidLike.0 | TypeFlags::Null.0,
    UnionOrIntersection = TypeFlags::Union.0 | TypeFlags::Intersection.0,
    StructuredType = TypeFlags::Object.0 | TypeFlags::Union.0 | TypeFlags::Intersection.0,
    TypeVariable = TypeFlags::TypeParameter.0 | TypeFlags::IndexedAccess.0,
    InstantiableNonPrimitive = TypeFlags::TypeVariable.0 | TypeFlags::Conditional.0 | TypeFlags::Substitution.0,
    InstantiablePrimitive = TypeFlags::IndexedAccess.0 | TypeFlags::TemplateLiteral.0 | TypeFlags::StringMapping.0,
    Instantiable = TypeFlags::InstantiableNonPrimitive.0 | TypeFlags::InstantiablePrimitive.0,
    StructuredOrInstantiable = TypeFlags::StructuredType.0 | TypeFlags::Instantiable.0,
    // @internal
    ObjectFlagsType = TypeFlags::Any.0 | TypeFlags::Nullable.0 | TypeFlags::Never.0 | TypeFlags::Object.0 | TypeFlags::Union.0 | TypeFlags::Intersection.0,
    // @internal
    Simplifiable = TypeFlags::IndexedAccess.0 | TypeFlags::Conditional.0,
    // @internal
    Singleton = TypeFlags::Any.0 | TypeFlags::Unknown.0 | TypeFlags::String.0 | TypeFlags::Number.0 | TypeFlags::Boolean.0 | TypeFlags::BigInt.0 | TypeFlags::ESSymbol.0 | TypeFlags::Void.0 | TypeFlags::Undefined.0 | TypeFlags::Null.0 | TypeFlags::Never.0 | TypeFlags::NonPrimitive.0,
    // 'Narrowable' types are types where narrowing actually narrows.
    // This *should* be every type other than null, undefined, void, and never
    Narrowable = TypeFlags::Any.0 | TypeFlags::Unknown.0 | TypeFlags::StructuredOrInstantiable.0 | TypeFlags::StringLike.0 | TypeFlags::NumberLike.0 | TypeFlags::BigIntLiteralLike.0 | TypeFlags::BooleanLike.0 | TypeFlags::ESSymbol.0 | TypeFlags::UniqueESSymbol.0 | TypeFlags::NonPrimitive.0,
    // The following flags are aggregated during union and intersection type construction
    // @internal
    IncludesMask = TypeFlags::Any.0 | TypeFlags::Unknown.0 | TypeFlags::Primitive.0 | TypeFlags::Never.0 | TypeFlags::Object.0 | TypeFlags::Union.0 | TypeFlags::Intersection.0 | TypeFlags::NonPrimitive.0 | TypeFlags::TemplateLiteral.0 | TypeFlags::StringMapping.0,
    // The following flags are used for different purposes during union and intersection type construction
    // @internal
    IncludesMissingType = TypeFlags::TypeParameter.0,
    // @internal
    IncludesNonWidenningType = TypeFlags::IndexedAccess.0,
    // @internal
    IncludesWildcard = TypeFlags::IndexedAccess.0,
    // @internal
    IncludesEmptyObject = TypeFlags::Conditional.0,
    // @internal
    IncludesInstantiable = TypeFlags::Substitution.0,
    // @internal
    IncludesConstrainedTypeVariable = TypeFlags::Reserved1.0,
    // @internal
    IncludesError = TypeFlags::Reserved2.0,
    // @internal
    NotPrimitiveUnion = TypeFlags::Any.0 | TypeFlags::Unknown.0 | TypeFlags::Void.0 | TypeFlags::Never.0 | TypeFlags::Object.0 | TypeFlags::Intersection.0 | TypeFlags::IncludesInstantiable.0,
});

#[derive(Debug)]
pub enum DestructuringPattern<'a> {
    BindingPattern(Box<BindingPattern<'a>>),
    ObjectExpression(Box<ObjectExpression<'a>>),
    ArrayExpression(Box<ArrayExpression<'a>>),
}

/** @internal */
pub type TypeId = usize;

#[derive(Debug)]
pub struct TypeObject<'a> {
    pub flags: TypeFlags, // Flags
    /** @internal */
    pub id: TypeId, // Unique ID
    /** @internal */
    // pub checker: Arc<Mutex<dyn TypeCheckerTrait>>, //Arc<&'a dyn TypeCheckerTrait>,
    pub symbol: opt_rc_cell!(Symbol<'a>), // Symbol associated with type (if any)
    pub pattern: Option<DestructuringPattern<'a>>, // Destructuring pattern represented by type (if any)
    pub aliasSymbol: opt_rc_cell!(Symbol<'a>), // Alias associated with type
    pub aliasTypeArguments: Option<Vec<Box<dyn Type<'a>>>>, // Alias type arguments (if any)
    /** @internal */
    pub permissiveInstantiation: Option<Box<dyn Type<'a>>>, // Instantiation with type parameters mapped to wildcard type
    /** @internal */
    pub restrictiveInstantiation: Option<Box<dyn Type<'a>>>, // Instantiation with type parameters mapped to unconstrained form
    /** @internal */
    pub immediateBaseConstraint: Option<Box<dyn Type<'a>>>, // Immediate base constraint cache
    /** @internal */
    pub widened: Option<Box<dyn Type<'a>>>, // Cached widened form of the type

    pub object_flags: Option<ObjectFlags>,               // ObjectFlagsType
    pub intrinsic_props: Option<IntrinsicTypeProps>,     // IntrinsicType
    pub freshable_props: Option<FreshableTypeProps<'a>>, // FreshableType
    pub object_props: Option<ObjectTypeProps<'a>>,       // ObjectType
    pub interface_props: Option<InterfaceTypeProps<'a>>, // InterfaceType
}
pub trait Type<'a>: std::fmt::Debug {
    fn getFlags(&self) -> TypeFlags;
    fn getSymbol(&self) -> opt_rc_cell!(Symbol<'a>);
    // fn getProperties(&self) -> Vec<&Symbol>;
    // fn getProperty(&self, property_name: &str) -> Option<&Symbol>;
    // fn getApparentProperties(&self) -> Vec<&Symbol>;
    // fn getCallSignatures(&self) -> Vec<&Signature>;
    // fn getConstructSignatures(&self) -> Vec<&Signature>;
    // fn getStringIndexType(&self) -> Option<&dyn Type>;
    // fn getNumberIndexType(&self) -> Option<&dyn Type>;
    // fn getBaseTypes(&self) -> Option<Vec<BaseType>>;
    // fn getNonNullableType(&self) -> &dyn Type;
    // fn getNonOptionalType(&self) -> &dyn Type;
    // fn isNullableType(&self) -> bool;
    // fn getConstraint(&self) -> Option<&dyn Type>;
    // fn getDefault(&self) -> Option<&dyn Type>;
    fn isUnion(&self) -> bool;
    fn isIntersection(&self) -> bool;
    fn isUnionOrIntersection(&self) -> bool;
    fn isLiteral(&self) -> bool;
    fn isStringLiteral(&self) -> bool;
    fn isNumberLiteral(&self) -> bool;
    fn isTypeParameter(&self) -> bool;
    fn isClassOrInterface(&self) -> bool;
    fn isClass(&self) -> bool;
    fn isIndexType(&self) -> bool;

    fn as_type(&self) -> &dyn Type<'a>;
}

/** @internal */
// Intrinsic types (TypeFlags.Intrinsic)
#[derive(Debug, Clone)]
pub struct IntrinsicTypeProps {
    pub intrinsicName: String, // Name of intrinsic type
    pub debugIntrinsicName: Option<String>,
}

pub trait IntrinsicType<'a>: ObjectFlagsTrait<'a> {
    fn get_intrinsic_props(&self) -> &IntrinsicTypeProps;
}

/** @internal */
pub trait NullableType<'a>: IntrinsicType<'a> {}

#[derive(Debug, Clone)]
pub struct FreshableTypeProps<'a> {
    pub freshType: &'a dyn FreshableType<'a>,   // Fresh version of type
    pub regularType: &'a dyn FreshableType<'a>, // Regular version of type
}

pub trait FreshableType<'a>: Type<'a> {
    fn get_freshable_type_props(&self) -> &FreshableTypeProps<'a>;
}

/** @internal */
pub trait FreshableIntrinsicType<'a>: FreshableType<'a> + IntrinsicType<'a> {}
// endregion: 6392

// region: 6423
// Types included in TypeFlags.ObjectFlagsType have an objectFlags property. Some ObjectFlags
// are specific to certain types and reuse the same bit position. Those ObjectFlags require a check
// for a certain TypeFlags value to determine their meaning.
define_flags!(ObjectFlags {
    None = 0,
    Class = 1 << 0,  // Class
    Interface = 1 << 1,  // Interface
    Reference = 1 << 2,  // Generic type reference
    Tuple = 1 << 3,  // Synthesized generic tuple type
    Anonymous = 1 << 4,  // Anonymous
    Mapped = 1 << 5,  // Mapped
    Instantiated = 1 << 6,  // Instantiated anonymous or mapped type
    ObjectLiteral = 1 << 7,  // Originates in an object literal
    EvolvingArray = 1 << 8,  // Evolving array type
    ObjectLiteralPatternWithComputedProperties = 1 << 9,  // Object literal pattern with computed properties
    ReverseMapped = 1 << 10, // Object contains a property from a reverse-mapped type
    JsxAttributes = 1 << 11, // Jsx attributes type
    JSLiteral = 1 << 12, // Object type declared in JS - disables errors on read/write of nonexisting members
    FreshLiteral = 1 << 13, // Fresh object literal
    ArrayLiteral = 1 << 14, // Originates in an array literal
    // @internal
    PrimitiveUnion = 1 << 15, // Union of only primitive types
    // @internal
    ContainsWideningType = 1 << 16, // Type is or contains undefined or null widening type
    // @internal
    ContainsObjectOrArrayLiteral = 1 << 17, // Type is or contains object literal type
    // @internal
    NonInferrableType = 1 << 18, // Type is or contains anyFunctionType or silentNeverType
    // @internal
    CouldContainTypeVariablesComputed = 1 << 19, // CouldContainTypeVariables flag has been computed
    // @internal
    CouldContainTypeVariables = 1 << 20, // Type could contain a type variable

    ClassOrInterface = Self::Class.0 | Self::Interface.0,
    // @internal
    RequiresWidening = Self::ContainsWideningType.0 | Self::ContainsObjectOrArrayLiteral.0,
    // @internal
    PropagatingFlags = Self::ContainsWideningType.0 | Self::ContainsObjectOrArrayLiteral.0 | Self::NonInferrableType.0,
    // @internal
    InstantiatedMapped = Self::Mapped.0 | Self::Instantiated.0,
    // Object flags that uniquely identify the kind of ObjectType
    // @internal
    ObjectTypeKindMask = Self::ClassOrInterface.0 | Self::Reference.0 | Self::Tuple.0 | Self::Anonymous.0 | Self::Mapped.0 | Self::ReverseMapped.0 | Self::EvolvingArray.0,

    // Flags that require TypeFlags.Object
    ContainsSpread = 1 << 21,  // Object literal contains spread operation
    ObjectRestType = 1 << 22,  // Originates in object rest declaration
    InstantiationExpressionType = 1 << 23,  // Originates in instantiation expression
    SingleSignatureType = 1 << 27,  // A single signature type extracted from a potentially broader type
    // @internal
    IsClassInstanceClone = 1 << 24, // Type is a clone of a class instance type
    // Flags that require TypeFlags.Object and ObjectFlags.Reference
    // @internal
    IdenticalBaseTypeCalculated = 1 << 25, // has had `getSingleBaseForNonAugmentingSubtype` invoked on it already
    // @internal
    IdenticalBaseTypeExists = 1 << 26, // has a defined cachedEquivalentBaseType member

    // Flags that require TypeFlags.UnionOrIntersection or TypeFlags.Substitution
    // @internal
    IsGenericTypeComputed = 1 << 21, // IsGenericObjectType flag has been computed
    // @internal
    IsGenericObjectType = 1 << 22, // Union or intersection contains generic object type
    // @internal
    IsGenericIndexType = 1 << 23, // Union or intersection contains generic index type
    // @internal
    IsGenericType = Self::IsGenericObjectType.0 | Self::IsGenericIndexType.0,

    // Flags that require TypeFlags.Union
    // @internal
    ContainsIntersections = 1 << 24, // Union contains intersections
    // @internal
    IsUnknownLikeUnionComputed = 1 << 25, // IsUnknownLikeUnion flag has been computed
    // @internal
    IsUnknownLikeUnion = 1 << 26, // Union of null, undefined, and empty object type

    // Flags that require TypeFlags.Intersection
    // @internal
    IsNeverIntersectionComputed = 1 << 24, // IsNeverLike flag has been computed
    // @internal
    IsNeverIntersection = 1 << 25, // Intersection reduces to never
    // @internal
    IsConstrainedTypeVariable = 1 << 26, // T & C, where T's constraint and C are primitives, object, or {}
});

pub trait ObjectFlagsTrait<'a>: Type<'a> {
    fn get_object_flags(&self) -> ObjectFlags;
}

enum ObjectFlagsType<'a> {
    NullableType(Box<dyn NullableType<'a>>),
    ObjectType(Box<dyn ObjectType<'a>>),
    // UnionType(Box<dyn UnionType>),
    // IntersectionType(Box<dyn IntersectionType>),
}

#[derive(Debug)]
pub struct ObjectTypeProps<'a> {
    /** @internal */
    pub members: Option<SymbolTable<'a>>, // Properties by name
    /** @internal */
    pub properties: Option<Vec<rc_cell!(Symbol<'a>)>>, // Properties
    /** @internal */
    pub callSignatures: Option<Vec<rc_cell!(Signature<'a>)>>, // Call signatures of type
    /** @internal */
    pub constructSignatures: Option<Vec<rc_cell!(Signature<'a>)>>, // Construct signatures of type
    /** @internal */
    pub indexInfos: Option<Vec<IndexInfo>>, // Index signatures
    /** @internal */
    pub objectTypeWithoutAbstractConstructSignatures: Option<Box<dyn ObjectType<'a>>>,
}

pub trait ObjectType<'a>: ObjectFlagsTrait<'a> {
    fn get_object_props(&self) -> &ObjectTypeProps<'a>;
}

#[derive(Debug)]
pub struct InterfaceTypeProps<'a> {
    pub typeParameters: Option<Vec<TypeParameter>>,      // Type parameters (undefined if non-generic)
    pub outerTypeParameters: Option<Vec<TypeParameter>>, // Outer type parameters (undefined if none)
    pub localTypeParameters: Option<Vec<TypeParameter>>, // Local type parameters (undefined if none)
    pub thisType: Option<TypeParameter>,                 // The "this" type (undefined if none)
    /** @internal */
    pub resolvedBaseConstructorType: Option<Box<dyn Type<'a>>>, // Resolved base constructor type of class
    /** @internal */
    pub resolvedBaseTypes: Vec<BaseType>, // Resolved base types
    /** @internal */
    pub baseTypesResolved: Option<bool>,
}

pub trait InterfaceType<'a>: ObjectType<'a> {
    fn get_interface_props(&self) -> &InterfaceTypeProps<'a>;
}
// endregion: 6537

// region: 6908
#[derive(Debug)]
pub enum SignatureKind {
    Call,
    Construct,
}

define_flags!(SignatureFlags {
    // Propagating flags
    HasRestParameter = 1 << 0,  // Indicates last parameter is rest parameter
    HasLiteralTypes = 1 << 1,  // Indicates signature is specialized
    Abstract = 1 << 2,  // Indicates signature comes from an abstract class, abstract construct signature, or abstract constructor type

    // Non-propagating flags
    IsInnerCallChain = 1 << 3,  // Indicates signature comes from a CallChain nested in an outer OptionalChain
    IsOuterCallChain = 1 << 4,  // Indicates signature comes from a CallChain that is the outermost chain of an optional expression
    IsUntypedSignatureInJsFile = 1 << 5,  // Indicates signature is from a js file and has no types
    IsNonInferrable = 1 << 6,  // Indicates signature comes from a non-inferrable type
    IsSignatureCandidateForOverloadFailure = 1 << 7,

    PropagatingFlags = Self::HasRestParameter.0 | Self::HasLiteralTypes.0 | Self::Abstract.0 | Self::IsUntypedSignatureInJsFile.0 | Self::IsSignatureCandidateForOverloadFailure.0,

    CallChainFlags = Self::IsInnerCallChain.0 | Self::IsOuterCallChain.0,
});

#[derive(Debug)]
pub struct Signature<'a> {
    /** @internal */
    pub flags: SignatureFlags,
    /** @internal */
    pub checker: Option<Box<dyn TypeCheckerTrait<'a>>>,
    pub declaration: Option<SignatureDeclaration<'a>>, // Originating declaration
    pub typeParameters: Option<Vec<TypeParameter>>,    // Type parameters (undefined if non-generic)
    pub parameters: Vec<rc_cell!(Symbol<'a>)>,         // Parameters
    pub thisParameter: opt_rc_cell!(Symbol<'a>),       // symbol of this-type parameter
    /** @internal */
    pub resolvedReturnType: Option<Box<dyn Type<'a>>>, // Lazily set by `getReturnTypeOfSignature`
    /** @internal */
    pub resolvedTypePredicate: Option<TypePredicate>, // Lazily set by `getTypePredicateOfSignature`
    /** @internal */
    pub minArgumentCount: i32,     // Number of non-optional parameters
    /** @internal */
    pub resolvedMinArgumentCount: Option<i32>, // Number of non-optional parameters (excluding trailing `void`)
    /** @internal */
    pub target: opt_rc_cell!(Signature<'a>), // Instantiation target
    /** @internal */
    pub mapper: Option<TypeMapper<'a>>, // Instantiation mapper
    /** @internal */
    pub compositeSignatures: Option<Vec<Signature<'a>>>, // Underlying signatures of a union/intersection signature
    /** @internal */
    pub compositeKind: Option<TypeFlags>, // TypeFlags.Union if the underlying signatures are from union members, otherwise TypeFlags.Intersection
    /** @internal */
    pub erasedSignatureCache: opt_rc_cell!(Signature<'a>), // Erased version of signature (deferred)
    /** @internal */
    pub canonicalSignatureCache: opt_rc_cell!(Signature<'a>), // Canonical version of signature (deferred)
    /** @internal */
    pub baseSignatureCache: opt_rc_cell!(Signature<'a>), // Base version of signature (deferred)
    /** @internal */
    pub optionalCallSignatureCache: Option<OptionalCallSignatureCache<'a>>, // Optional chained call version of signature (deferred)
    /** @internal */
    pub isolatedSignatureType: Option<Box<dyn ObjectType<'a>>>, // A manufactured type that just contains the signature for purposes of signature comparison
    /** @internal */
    pub instantiations: Option<HashMap<String, rc_cell!(Signature<'a>)>>, // Generic signature instantiation cache
    /** @internal */
    pub implementationSignatureCache: opt_rc_cell!(Signature<'a>), // Copy of the signature with fresh type parameters to use in checking the body of a potentially self-referential generic function (deferred)
}

#[derive(Debug)]
pub struct OptionalCallSignatureCache<'a> {
    pub inner: opt_rc_cell!(Signature<'a>),
    pub outer: opt_rc_cell!(Signature<'a>),
}
// endregion: 6980

// region: 6995
#[derive(Debug)]
pub enum TypeMapKind {
    Simple,
    Array,
    Deferred,
    Function,
    Composite,
    Merged,
}

pub enum TypeMapper<'a> {
    Simple { source: Box<dyn Type<'a>>, target: Box<dyn Type<'a>> },
    Array { sources: Vec<Box<dyn Type<'a>>>, targets: Option<Vec<Box<dyn Type<'a>>>> },
    Deferred { sources: Vec<Box<dyn Type<'a>>>, targets: Vec<Box<dyn Fn() -> Box<dyn Type<'a>>>> },
    Function { func: Box<dyn Fn(Box<dyn Type<'a>>) -> Box<dyn Type<'a>>>, debug_info: Option<Box<dyn Fn() -> String>> },
    Composite { mapper1: Box<TypeMapper<'a>>, mapper2: Box<TypeMapper<'a>> },
    Merged { mapper1: Box<TypeMapper<'a>>, mapper2: Box<TypeMapper<'a>> },
}

impl<'a> std::fmt::Debug for TypeMapper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple { source, target } => f.debug_struct("Simple").field("source", source).field("target", target).finish(),
            Self::Array { sources, targets } => f.debug_struct("Array").field("sources", sources).field("targets", targets).finish(),
            Self::Deferred { sources, .. } => f.debug_struct("Deferred").field("sources", sources).finish(),
            Self::Function { .. } => f.debug_struct("Function").finish(),
            Self::Composite { mapper1, mapper2 } => f.debug_struct("Composite").field("mapper1", mapper1).field("mapper2", mapper2).finish(),
            Self::Merged { mapper1, mapper2 } => f.debug_struct("Merged").field("mapper1", mapper1).field("mapper2", mapper2).finish(),
        }
    }
}
// endregion: 7010

// region: 7102
/** @internal */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentDeclarationKind {
    None,
    /// exports.name = expr
    /// module.exports.name = expr
    ExportsProperty,
    /// module.exports = expr
    ModuleExports,
    /// className.prototype.name = expr
    PrototypeProperty,
    /// this.name = expr
    ThisProperty,
    // F.name = expr
    Property,
    // F.prototype = { ... }
    Prototype,
    // Object.defineProperty(x, 'name', { value: any, writable?: boolean (false by default) });
    // Object.defineProperty(x, 'name', { get: Function, set: Function });
    // Object.defineProperty(x, 'name', { get: Function });
    // Object.defineProperty(x, 'name', { set: Function });
    ObjectDefinePropertyValue,
    // Object.defineProperty(exports || module.exports, 'name', ...);
    ObjectDefinePropertyExports,
    // Object.defineProperty(Foo.prototype, 'name', ...);
    ObjectDefinePrototypeProperty,
}

#[derive(Debug)]
pub struct FileExtensionInfo {
    pub extension: String,
    pub isMixedContent: bool,
    pub scriptKind: Option<ScriptKind>,
}

#[derive(Debug)]
pub struct DiagnosticMessage {
    pub code: i32,
    pub category: DiagnosticCategory,
    pub key: String,
    pub message: String,
    pub reportsUnnecessary: Option<bool>,
    pub elidedInCompatibilityPyramid: Option<bool>,
    pub reportsDeprecated: Option<bool>,
}

/** @internal */
#[derive(Debug)]
pub struct RepopulateModuleNotFoundDiagnosticChain {
    pub moduleReference: String,
    pub mode: ResolutionMode,
    pub packageName: Option<String>,
}

/** @internal */
pub type RepopulateModeMismatchDiagnosticChain = bool; // true

/** @internal */
#[derive(Debug)]
pub enum RepopulateDiagnosticChainInfo {
    ModuleNotFound(RepopulateModuleNotFoundDiagnosticChain),
    ModeMismatch(RepopulateModeMismatchDiagnosticChain),
}

/**
 * A linked list of formatted diagnostic messages to be used as part of a multiline message.
 * It is built from the bottom up, leaving the head to be the "main" diagnostic.
 * While it seems that DiagnosticMessageChain is structurally similar to DiagnosticMessage,
 * the difference is that messages are all preformatted in DMC.
 */
#[derive(Debug, Clone)]
pub struct DiagnosticMessageChain {
    pub messageText: String,
    pub category: DiagnosticCategory,
    pub code: i32,
    pub next: Option<Vec<DiagnosticMessageChain>>,
    /** @internal */
    // pub repopulateInfo: Option<Box<dyn Fn() -> RepopulateDiagnosticChainInfo>>, // todo(rb): implement this if needed
    /** @internal */
    pub canonicalHead: Option<CanonicalDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<'a> {
    // DiagnosticRelatedInformation
    pub category: DiagnosticCategory,
    pub code: i32,
    pub file: Option<&'a SourceFile<'a>>,
    pub start: Option<u32>,
    pub length: Option<u32>,
    pub messageText: StringOrDiagnosticMessageChain,

    // DiagnosticWithDetachedLocation
    pub fileName: Option<String>,

    // Diagnostic
    /** May store more in future. For now, this will simply be `true` to indicate when a diagnostic is an unused-identifier diagnostic. */
    pub reportsUnnecessary: Option<()>,

    pub reportsDeprecated: Option<()>,
    pub source: Option<String>,
    pub relatedInformation: Option<Vec<DiagnosticRelatedInformation<'a>>>,
    /** @internal */
    pub skippedOn: Option<String>, // keyof CompilerOptions
    /**
     * @internal
     * Used for deduplication and comparison.
     * Whenever it is possible for two diagnostics that report the same problem to be produced with
     * different messages (e.g. "Cannot find name 'foo'" vs "Cannot find name 'foo'. Did you mean 'bar'?"),
     * this property can be set to a canonical message,
     * so that those two diagnostics are appropriately considered to be the same.
     */
    pub canonicalHead: Option<CanonicalDiagnostic>,
}

/** @internal */
#[derive(Debug, Clone)]
pub struct CanonicalDiagnostic {
    pub code: i32,
    pub messageText: String,
}

/** @internal */
pub type DiagnosticArguments = Vec<StringOrNumber>;

/** @internal */
pub type DiagnosticAndArguments<'a> = (DiagnosticMessage, DiagnosticArguments);

pub type DiagnosticRelatedInformation<'a> = Diagnostic<'a>;

pub type DiagnosticWithLocation<'a> = Diagnostic<'a>;

/** @internal */
pub type DiagnosticWithDetachedLocation<'a> = Diagnostic<'a>;

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticCategory {
    Error,
    Warning,
    Suggestion,
    Message,
}
/** @internal */
pub fn diagnosticCategoryName(d: &DiagnosticCategory, lower_case: bool) -> String {
    let name = match d {
        DiagnosticCategory::Error => "Error",
        DiagnosticCategory::Warning => "Warning",
        DiagnosticCategory::Suggestion => "Suggestion",
        DiagnosticCategory::Message => "Message",
    };
    return if lower_case { name.to_lowercase() } else { name.to_string() };
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum ModuleResolutionKind {
    Classic = 1,
    /**
     * @deprecated
     * `NodeJs` was renamed to `Node10` to better reflect the version of Node that it targets.
     * Use the new name or consider switching to a modern module resolution target.
     */
    // NodeJs = 2,
    Node10 = 2,
    // Starting with node12, node's module resolver has significant departures from traditional cjs resolution
    // to better support ECMAScript modules and their use within node - however more features are still being added.
    // TypeScript's Node ESM support was introduced after Node 12 went end-of-life, and Node 14 is the earliest stable
    // version that supports both pattern trailers - *but*, Node 16 is the first version that also supports ECMAScript 2022.
    // In turn, we offer both a `NodeNext` moving resolution target, and a `Node16` version-anchored resolution target
    Node16 = 3,
    NodeNext = 99, // Not simply `Node16` so that compiled code linked against TS can use the `Next` value reliably (same as with `ModuleKind`)
    Bundler = 100,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleDetectionKind {
    /**
     * Files with imports, exports and/or import.meta are considered modules
     */
    Legacy = 1,
    /**
     * Legacy, but also files with jsx under react-jsx or react-jsxdev and esm mode files under moduleResolution: node16+
     */
    Auto = 2,
    /**
     * Consider all non-declaration files modules, regardless of present syntax
     */
    Force = 3,
}
// endregion: 7275

// region: 7317
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompilerOptions {
    /** @internal */
    pub all: Option<bool>,
    pub allowImportingTsExtensions: Option<bool>,
    pub allowJs: Option<bool>,
    /** @internal */
    pub allowNonTsExtensions: Option<bool>,
    pub allowArbitraryExtensions: Option<bool>,
    pub allowSyntheticDefaultImports: Option<bool>,
    pub allowUmdGlobalAccess: Option<bool>,
    pub allowUnreachableCode: Option<bool>,
    pub allowUnusedLabels: Option<bool>,
    pub alwaysStrict: Option<bool>, // Always combine with strict property
    pub baseUrl: Option<String>,
    /**
     * An error if set - this should only go through the -b pipeline and not actually be observed
     *
     * @internal
     */
    pub build: Option<bool>,
    /** @deprecated */
    pub charset: Option<String>,
    pub checkJs: Option<bool>,
    /** @internal */
    pub configFilePath: Option<String>,
    /**
     * configFile is set as non enumerable property so as to avoid checking of json source files
     *
     * @internal
     */
    // pub configFile: Option<TsConfigSourceFile>,
    pub customConditions: Option<Vec<String>>,
    pub declaration: Option<bool>,
    pub declarationMap: Option<bool>,
    pub emitDeclarationOnly: Option<bool>,
    pub declarationDir: Option<String>,
    /** @internal */
    pub diagnostics: Option<bool>,
    /** @internal */
    pub extendedDiagnostics: Option<bool>,
    pub disableSizeLimit: Option<bool>,
    pub disableSourceOfProjectReferenceRedirect: Option<bool>,
    pub disableSolutionSearching: Option<bool>,
    pub disableReferencedProjectLoad: Option<bool>,
    pub downlevelIteration: Option<bool>,
    pub emitBom: Option<bool>,
    pub emitDecoratorMetadata: Option<bool>,
    pub exactOptionalPropertyTypes: Option<bool>,
    pub experimentalDecorators: Option<bool>,
    pub forceConsistentCasingInFileNames: Option<bool>,
    /** @internal */
    pub generateCpuProfile: Option<String>,
    /** @internal */
    pub generateTrace: Option<String>,
    /** @internal */
    pub help: Option<bool>,
    pub ignoreDeprecations: Option<String>,
    pub importHelpers: Option<bool>,
    /** @deprecated */
    pub importsNotUsedAsValues: Option<ImportsNotUsedAsValues>,
    /** @internal */
    pub init: Option<bool>,
    pub inlineSourceMap: Option<bool>,
    pub inlineSources: Option<bool>,
    pub isolatedModules: Option<bool>,
    pub isolatedDeclarations: Option<bool>,
    pub jsx: Option<JsxEmit>,
    /** @deprecated */
    pub keyofStringsOnly: Option<bool>,
    pub lib: Option<Vec<String>>,
    /** @internal */
    pub listEmittedFiles: Option<bool>,
    /** @internal */
    pub listFiles: Option<bool>,
    /** @internal */
    pub explainFiles: Option<bool>,
    /** @internal */
    pub listFilesOnly: Option<bool>,
    pub locale: Option<String>,
    pub mapRoot: Option<String>,
    pub maxNodeModuleJsDepth: Option<i32>,
    pub module: Option<ModuleKind>,
    pub moduleResolution: Option<ModuleResolutionKind>,
    pub moduleSuffixes: Option<Vec<String>>,
    pub moduleDetection: Option<ModuleDetectionKind>,
    pub newLine: Option<NewLineKind>,
    pub noEmit: Option<bool>,
    /** @internal */
    pub noCheck: Option<bool>,
    /** @internal */
    pub noEmitForJsFiles: Option<bool>,
    pub noEmitHelpers: Option<bool>,
    pub noEmitOnError: Option<bool>,
    pub noErrorTruncation: Option<bool>,
    pub noFallthroughCasesInSwitch: Option<bool>,
    pub noImplicitAny: Option<bool>, // Always combine with strict property
    pub noImplicitReturns: Option<bool>,
    pub noImplicitThis: Option<bool>, // Always combine with strict property
    /** @deprecated */
    pub noStrictGenericChecks: Option<bool>,
    pub noUnusedLocals: Option<bool>,
    pub noUnusedParameters: Option<bool>,
    /** @deprecated */
    pub noImplicitUseStrict: Option<bool>,
    pub noPropertyAccessFromIndexSignature: Option<bool>,
    pub assumeChangesOnlyAffectDirectDependencies: Option<bool>,
    pub noLib: Option<bool>,
    pub noResolve: Option<bool>,
    /** @internal */
    pub noDtsResolution: Option<bool>,
    pub noUncheckedIndexedAccess: Option<bool>,
    /** @deprecated */
    pub out: Option<String>,
    pub outDir: Option<String>,
    pub outFile: Option<String>,
    pub paths: Option<HashMap<String, Vec<String>>>,
    /**
     * The directory of the config file that specified 'paths'. Used to resolve relative paths when 'baseUrl' is absent.
     *
     * @internal
     */
    pub pathsBasePath: Option<String>,
    /** @internal */
    // pub plugins: Option<Vec<PluginImport>>,
    pub preserveConstEnums: Option<bool>,
    pub noImplicitOverride: Option<bool>,
    pub preserveSymlinks: Option<bool>,
    /** @deprecated */
    pub preserveValueImports: Option<bool>,
    /** @internal */
    pub preserveWatchOutput: Option<bool>,
    pub project: Option<String>,
    /** @internal */
    pub pretty: Option<bool>,
    pub reactNamespace: Option<String>,
    pub jsxFactory: Option<String>,
    pub jsxFragmentFactory: Option<String>,
    pub jsxImportSource: Option<String>,
    pub composite: Option<bool>,
    pub incremental: Option<bool>,
    pub tsBuildInfoFile: Option<String>,
    pub removeComments: Option<bool>,
    pub resolvePackageJsonExports: Option<bool>,
    pub resolvePackageJsonImports: Option<bool>,
    pub rootDir: Option<String>,
    pub rootDirs: Option<Vec<String>>,
    pub skipLibCheck: Option<bool>,
    pub skipDefaultLibCheck: Option<bool>,
    pub sourceMap: Option<bool>,
    pub sourceRoot: Option<String>,
    pub strict: Option<bool>,
    pub strictFunctionTypes: Option<bool>,          // Always combine with strict property
    pub strictBindCallApply: Option<bool>,          // Always combine with strict property
    pub strictNullChecks: Option<bool>,             // Always combine with strict property
    pub strictPropertyInitialization: Option<bool>, // Always combine with strict property
    pub strictBuiltinIteratorReturn: Option<bool>,  // Always combine with strict property
    pub stripInternal: Option<bool>,
    /** @deprecated */
    pub suppressExcessPropertyErrors: Option<bool>,
    /** @deprecated */
    pub suppressImplicitAnyIndexErrors: Option<bool>,
    /** @internal */
    pub suppressOutputPathCheck: Option<bool>,
    pub target: Option<ScriptTarget>,
    pub traceResolution: Option<bool>,
    pub useUnknownInCatchVariables: Option<bool>,
    pub noUncheckedSideEffectImports: Option<bool>,
    pub resolveJsonModule: Option<bool>,
    pub types: Option<Vec<String>>,
    /** Paths used to compute primary types search locations */
    pub typeRoots: Option<Vec<String>>,
    pub verbatimModuleSyntax: Option<bool>,
    /** @internal */
    pub version: Option<bool>,
    /** @internal */
    pub watch: Option<bool>,
    pub esModuleInterop: Option<bool>,
    /** @internal */
    pub showConfig: Option<bool>,
    pub useDefineForClassFields: Option<bool>,
    /** @internal */
    pub tscBuild: Option<bool>,
}
// endregion: 7478

// region: 7499
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum ModuleKind {
    None = 0,
    CommonJS = 1,
    AMD = 2,
    UMD = 3,
    System = 4,

    // NOTE: ES module kinds should be contiguous to more easily check whether a module kind is *any* ES module kind.
    //       Non-ES module kinds should not come between ES2015 (the earliest ES module kind) and ESNext (the last ES
    //       module kind).
    ES2015 = 5,
    ES2020 = 6,
    ES2022 = 7,
    ESNext = 99,

    // Node16+ is an amalgam of commonjs (albeit updated) and es2022+, and represents a distinct module system from es2020/esnext
    Node16 = 100,
    NodeNext = 199,

    // Emit as written
    Preserve = 200,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsxEmit {
    None = 0,
    Preserve = 1,
    React = 2,
    ReactNative = 3,
    ReactJSX = 4,
    ReactJSXDev = 5,
}

/** @deprecated */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportsNotUsedAsValues {
    Remove,
    Preserve,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewLineKind {
    CarriageReturnLineFeed = 0,
    LineFeed = 1,
}

#[derive(Debug, Clone)]
pub struct LineAndCharacter {
    /** 0-based. */
    pub line: usize,
    /*
     * 0-based. This value denotes the character position in line and is different from the 'column' because of tab characters.
     */
    pub character: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    Unknown = 0,
    JS = 1,
    JSX = 2,
    TS = 3,
    TSX = 4,
    External = 5,
    JSON = 6,
    /**
     * Used on extensions that doesn't define the ScriptKind but the content defines it.
     * Deferred extensions are going to be included in all project contexts.
     */
    Deferred = 7,
}

// NOTE: We must reevaluate the target for upcoming features when each successive TC39 edition is ratified in
//       June of each year. This includes changes to `LanguageFeatureMinimumTarget`, `ScriptTarget`,
//       transformers/esnext.ts, commandLineParser.ts, and the contents of each lib/esnext.*.d.ts file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptTarget {
    /** @deprecated */
    ES3 = 0,
    ES5 = 1,
    ES2015 = 2,
    ES2016 = 3,
    ES2017 = 4,
    ES2018 = 5,
    ES2019 = 6,
    ES2020 = 7,
    ES2021 = 8,
    ES2022 = 9,
    ES2023 = 10,
    ESNext = 99,
    JSON = 100,
    // Latest = 99, // Same as ESNext
}

// import LanguageVariant from oxc
// endregion: 7591

// region: 7734
// dprint-ignore
/** @internal */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterCodes {
    EOF = -1,
    nullCharacter = 0,
    maxAsciiCharacter = 0x7F,

    lineFeed = 0x0A,       // \n
    carriageReturn = 0x0D, // \r
    lineSeparator = 0x2028,
    paragraphSeparator = 0x2029,
    nextLine = 0x0085,

    // Unicode 3.0 space characters
    space = 0x0020,            // " "
    nonBreakingSpace = 0x00A0, //
    enQuad = 0x2000,
    emQuad = 0x2001,
    enSpace = 0x2002,
    emSpace = 0x2003,
    threePerEmSpace = 0x2004,
    fourPerEmSpace = 0x2005,
    sixPerEmSpace = 0x2006,
    figureSpace = 0x2007,
    punctuationSpace = 0x2008,
    thinSpace = 0x2009,
    hairSpace = 0x200A,
    zeroWidthSpace = 0x200B,
    narrowNoBreakSpace = 0x202F,
    ideographicSpace = 0x3000,
    mathematicalSpace = 0x205F,
    ogham = 0x1680,

    // Unicode replacement character produced when a byte sequence is invalid
    replacementCharacter = 0xFFFD,

    underscore = 0x5F,
    dollar = 0x24,

    _0 = 0x30,
    _1 = 0x31,
    _2 = 0x32,
    _3 = 0x33,
    _4 = 0x34,
    _5 = 0x35,
    _6 = 0x36,
    _7 = 0x37,
    _8 = 0x38,
    _9 = 0x39,

    a = 0x61,
    b = 0x62,
    c = 0x63,
    d = 0x64,
    e = 0x65,
    f = 0x66,
    g = 0x67,
    h = 0x68,
    i = 0x69,
    j = 0x6A,
    k = 0x6B,
    l = 0x6C,
    m = 0x6D,
    n = 0x6E,
    o = 0x6F,
    p = 0x70,
    q = 0x71,
    r = 0x72,
    s = 0x73,
    t = 0x74,
    u = 0x75,
    v = 0x76,
    w = 0x77,
    x = 0x78,
    y = 0x79,
    z = 0x7A,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5a,

    ampersand = 0x26,    // &
    asterisk = 0x2A,     // *
    at = 0x40,           // @
    backslash = 0x5C,    // \
    backtick = 0x60,     // `
    bar = 0x7C,          // |
    caret = 0x5E,        // ^
    closeBrace = 0x7D,   // }
    closeBracket = 0x5D, // ]
    closeParen = 0x29,   // )
    colon = 0x3A,        // :
    comma = 0x2C,        // ,
    dot = 0x2E,          // .
    doubleQuote = 0x22,  // "
    equals = 0x3D,       // =
    exclamation = 0x21,  // !
    greaterThan = 0x3E,  // >
    hash = 0x23,         // #
    lessThan = 0x3C,     // <
    minus = 0x2D,        // -
    openBrace = 0x7B,    // {
    openBracket = 0x5B,  // [
    openParen = 0x28,    // (
    percent = 0x25,      // %
    plus = 0x2B,         // +
    question = 0x3F,     // ?
    semicolon = 0x3B,    // ;
    singleQuote = 0x27,  // '
    slash = 0x2F,        // /
    tilde = 0x7E,        // ~

    backspace = 0x08, // \b
    formFeed = 0x0C,  // \f
    byteOrderMark = 0xFEFF,
    tab = 0x09,         // \t
    verticalTab = 0x0B, // \v
}

impl PartialEq<u32> for CharacterCodes {
    fn eq(&self, other: &u32) -> bool { *self as u32 == *other }
}

impl PartialEq<CharacterCodes> for u32 {
    fn eq(&self, other: &CharacterCodes) -> bool { *self == *other as u32 }
}

impl PartialEq<char> for CharacterCodes {
    fn eq(&self, other: &char) -> bool { *self as u32 == *other as u32 }
}

impl PartialEq<CharacterCodes> for char {
    fn eq(&self, other: &CharacterCodes) -> bool { *self as u32 == *other as u32 }
}

pub trait ModuleResolutionHost {
    // TODO: GH#18217 Optional methods frequently used as non-optional

    fn fileExists(&self, fileName: &str) -> bool;
    // readFile function is used to read arbitrary text files on disk, i.e. when resolution procedure needs the content of 'package.json'
    // to determine location of bundled typings for node module
    fn readFile(&self, fileName: &str) -> Option<String>;
    fn trace(&self, s: &str) -> Option<()>;
    fn directoryExists(&self, directoryName: &str) -> Option<bool>;
    /**
     * Resolve a symbolic link.
     * @see https://nodejs.org/api/fs.html#fs_fs_realpathsync_path_options
     */
    fn realpath(&self, path: &str) -> Option<String>;
    fn getCurrentDirectory(&self) -> Option<String>;
    fn getDirectories(&self, path: &str) -> Option<Vec<String>>;
    fn useCaseSensitiveFileNames(&self) -> Option<bool>;
}
// endregion: 7893

// region: 7961
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    Ts,
    Tsx,
    Dts,
    Js,
    Jsx,
    Json,
    TsBuildInfo,
    Mjs,
    Mts,
    Dmts,
    Cjs,
    Cts,
    Dcts,
}
impl Extension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Extension::Ts => ".ts",
            Extension::Tsx => ".tsx",
            Extension::Dts => ".d.ts",
            Extension::Js => ".js",
            Extension::Jsx => ".jsx",
            Extension::Json => ".json",
            Extension::TsBuildInfo => ".tsbuildinfo",
            Extension::Mjs => ".mjs",
            Extension::Mts => ".mts",
            Extension::Dmts => ".d.mts",
            Extension::Cjs => ".cjs",
            Extension::Cts => ".cts",
            Extension::Dcts => ".d.cts",
        }
    }
}
// endregion: 7976

// region: 8499
define_flags!(OuterExpressionKinds {
    Parentheses = 1 << 0,
    TypeAssertions = 1 << 1,
    NonNullAssertions = 1 << 2,
    PartiallyEmittedExpressions = 1 << 3,
    ExpressionsWithTypeArguments = 1 << 4,

    Assertions = Self::TypeAssertions.0 | Self::NonNullAssertions.0,
    All = Self::Parentheses.0 | Self::Assertions.0 | Self::PartiallyEmittedExpressions.0 | Self::ExpressionsWithTypeArguments.0,

    ExcludeJSDocTypeAssertion = 1 << 31,
});

define_subset_enum!(OuterExpression from AstKind {
    ParenthesizedExpression,
    TSTypeAssertion,
    TSSatisfiesExpression,
    TSAsExpression,
    TSNonNullExpression,
    ExpressionWithTypeArguments,
    // !rb skipping PartiallyEmittedExpression
    // PartiallyEmittedExpression,
});
// endregion: 8522

// region: 9915
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSpan {
    pub start: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextChangeRange {
    pub span: TextSpan,
    pub newLength: u32,
}
// endregion: 9925

// region: 9840
/** @internal */
pub trait ModuleSpecifierResolutionHost: ModuleResolutionHost {
    // fn getSymlinkCache(&self) -> Option<SymlinkCache>;
    // fn getModuleSpecifierCache(&self) -> Option<ModuleSpecifierCache>;
    fn getPackageJsonInfoCache(&self) -> Option<&dyn PackageJsonInfoCache>;
    fn getGlobalTypingsCacheLocation(&self) -> Option<String>;
    fn getNearestAncestorDirectoryWithPackageJson(&self, file_name: &str, root_dir: Option<&str>) -> Option<String>;

    // fn getRedirectTargetsMap(&self) -> &RedirectTargetsMap;
    fn getProjectReferenceRedirect(&self, file_name: &str) -> Option<String>;
    fn isSourceOfProjectReferenceRedirect(&self, file_name: &str) -> bool;
    // fn getFileIncludeReasons(&self) -> MultiMap<Path, FileIncludeReason>;
    fn getCommonSourceDirectory(&self) -> String;
    // fn getDefaultResolutionModeForFile(&self, source_file: &SourceFile) -> ResolutionMode;
    // fn getModeForResolutionAtIndex(&self, file: &SourceFile, index: usize) -> ResolutionMode;

    // fn getModuleResolutionCache(&self) -> Option<ModuleResolutionCache>;
}
// endregion: 9864
