use oxc::ast::{
    ast::{Argument, ArrayExpression, BinaryExpression, BindingPattern, CallExpression, Decorator, Expression, JSXElement, NewExpression, ObjectExpression, TaggedTemplateExpression},
    AstKind,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct IndexInfo;

#[derive(Debug)]
pub struct BaseType;

#[derive(Debug)]
pub struct TypeNode;

#[derive(Debug)]
pub struct TypePredicate;

#[derive(Debug)]
pub struct TypePredicateNode;

#[derive(Debug)]
pub struct SignatureDeclaration;

#[derive(Debug)]
pub struct TypeParameter;

#[derive(Debug)]
pub struct ExportSpecifier;

#[derive(Debug)]
pub struct Identifier;

#[derive(Debug)]
pub struct AssignmentPattern;

#[derive(Debug)]
pub struct EmitTextWriter;

/// Represents an expression that can be called or constructed, including function calls,
/// constructor calls, template literals, decorators, JSX elements, and instanceof checks.
#[derive(Debug)]
pub enum CallLikeExpression<'a> {
    /// Function call expression like `foo()`
    CallExpression(Box<CallExpression<'a>>),
    /// Constructor call with new like `new Foo()`
    NewExpression(Box<NewExpression<'a>>),
    /// Tagged template literal like `foo`bar``
    TaggedTemplateExpression(Box<TaggedTemplateExpression<'a>>),
    /// Class or method decorator like `@decorator`
    Decorator(Box<Decorator<'a>>),
    /// JSX opening element like `<div>`
    ///! oxc doesn't have a separate JSXSelfClosingElement, so we can't explicitly model JSXOpeningElement | JSXSelfClosingElement
    JsxElement(Box<JSXElement<'a>>),
    /// instanceof check like `foo instanceof Bar`
    ///! There doesn't seem to be a good way to explicitly model InstanceofExpression, so we use BinaryExpression
    BinaryExpression(Box<BinaryExpression<'a>>),
}

#[derive(Debug)]
pub struct ObjectLiteralElementLike;

#[derive(Debug)]
pub struct JsxAttributeLike;

#[derive(Debug)]
pub struct JsxAttribute;

#[derive(Debug)]
pub struct JsxSpreadAttribute;

#[derive(Debug)]
pub struct EnumMember;

#[derive(Debug)]
pub struct PropertyAccessExpression;

#[derive(Debug)]
pub struct ElementAccessExpression;

#[derive(Debug)]
pub struct QualifiedName;

#[derive(Debug)]
pub struct ImportTypeNode;

#[derive(Debug)]
pub struct MemberName;

#[derive(Debug)]
pub struct JsxAttributes;

#[derive(Debug)]
pub struct SourceFile;

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
pub struct ClassLikeDeclaration;

#[derive(Debug)]
pub struct ClassElement;

#[derive(Debug)]
pub struct AnyImportSyntax;

#[derive(Debug, Clone)]
pub struct Declaration;

#[derive(Debug)]
pub struct StringLiteralType;

#[derive(Debug)]
pub struct NumberLiteralType;

#[derive(Debug)]
pub struct BigIntLiteralType;

#[derive(Debug)]
pub struct PseudoBigInt;

#[derive(Debug, Clone)]
pub struct SymbolTable;

#[derive(Debug)]
pub struct TransientSymbol;

#[derive(Debug)]
pub struct SymbolAccessibilityResult;

#[derive(Debug)]
pub struct SymbolWalker;

#[derive(Debug)]
pub struct Diagnostic;

#[derive(Debug)]
pub struct DiagnosticWithLocation;

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

// 5010
/** @internal */
pub trait TypeCheckerHost: ModuleSpecifierResolutionHost {
    fn getCompilerOptions(&self) -> CompilerOptions;
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

pub trait TypeCheckerTrait: std::fmt::Debug {
    fn getTypeOfSymbolAtLocation(&self, symbol: &Symbol, node: &AstKind) -> &dyn Type;
    fn getTypeOfSymbol(&self, symbol: &Symbol) -> &dyn Type;
    fn getDeclaredTypeOfSymbol(&self, symbol: &Symbol) -> &dyn Type;
    fn getPropertiesOfType(&self, type_: &dyn Type) -> Vec<&Symbol>;
    fn getPropertyOfType(&self, type_: &dyn Type, property_name: &str) -> Option<&Symbol>;
    fn getPrivateIdentifierPropertyOfType(&self, left_type: &dyn Type, name: &str, location: &AstKind) -> Option<&Symbol>;
    /** @internal */
    fn getTypeOfPropertyOfType(&self, type_: &dyn Type, propertyName: &str) -> Option<&dyn Type>;
    fn getIndexInfoOfType(&self, type_: &dyn Type, kind: IndexKind) -> Option<IndexInfo>;
    fn getIndexInfosOfType(&self, type_: &dyn Type) -> Vec<IndexInfo>;
    fn getIndexInfosOfIndexSymbol(&self, indexSymbol: Symbol) -> Vec<IndexInfo>;
    fn getSignaturesOfType(&self, type_: &dyn Type, kind: SignatureKind) -> Vec<&Signature>;
    fn getIndexTypeOfType(&self, type_: &dyn Type, kind: IndexKind) -> Option<&dyn Type>;
    /** @internal */
    fn getIndexType(&self, type_: &dyn Type) -> &dyn Type;
    fn getBaseTypes(&self, type_: &dyn InterfaceType) -> Vec<BaseType>;
    fn getBaseTypeOfLiteralType(&self, type_: &dyn Type) -> &dyn Type;
    fn getWidenedType(&self, type_: &dyn Type) -> &dyn Type;
    /** @internal */
    fn getWidenedLiteralType(&self, type_: &dyn Type) -> &dyn Type;
    /** @internal */
    fn getPromisedTypeOfPromise(&self, promise: &dyn Type, errorNode: Option<AstKind>) -> Option<&dyn Type>;
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
    fn getAwaitedType(&self, type_: &dyn Type) -> Option<&dyn Type>;
    /** @internal */
    fn isEmptyAnonymousObjectType(&self, type_: &dyn Type) -> bool;
    fn getReturnTypeOfSignature(&self, signature: Signature) -> &dyn Type;
    /// Gets the type of a parameter at a given position in a signature.
    /// Returns any if the index is not valid.
    ///
    /** @internal */
    fn getParameterType(&self, signature: Signature, parameter_index: usize) -> &dyn Type;
    /** @internal */
    fn getParameterIdentifierInfoAtPosition(&self, signature: Signature, parameter_index: usize) -> Option<(Identifier, &str, bool)>;
    fn getNullableType(&self, type_: &dyn Type, flags: TypeFlags) -> &dyn Type;
    fn getNonNullableType(&self, type_: &dyn Type) -> &dyn Type;
    /** @internal */
    fn getNonOptionalType(&self, type_: &dyn Type) -> &dyn Type;
    /** @internal */
    fn isNullableType(&self, type_: &dyn Type) -> bool;
    fn getTypeArguments(&self, type_: TypeReference) -> Vec<&dyn Type>;

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
    fn getExportSymbolOfSymbol(&self, symbol: Symbol) -> Symbol;
    fn getPropertySymbolOfDestructuringAssignment(&self, location: Identifier) -> Option<Symbol>;
    fn getTypeOfAssignmentPattern(&self, pattern: AssignmentPattern) -> &dyn Type;
    fn getTypeAtLocation(&self, node: AstKind) -> &dyn Type;
    fn getTypeFromTypeNode(&self, node: TypeNode) -> &dyn Type;

    fn signatureToString(&self, signature: Signature, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>) -> String;
    fn typeToString(&self, type_: &dyn Type, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>) -> String;
    fn symbolToString(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>) -> String;
    fn typePredicateToString(&self, predicate: TypePredicate, enclosing_declaration: Option<AstKind>, flags: Option<TypeFormatFlags>) -> String;

    /** @internal */
    fn writeSignature(&self, signature: Signature, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>, writer: Option<EmitTextWriter>) -> String;
    /** @internal */
    fn writeType(&self, type_: &dyn Type, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String;
    /** @internal */
    fn writeSymbol(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>, writer: Option<EmitTextWriter>) -> String;
    /** @internal */
    fn writeTypePredicate(&self, predicate: TypePredicate, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String;

    fn getFullyQualifiedName(&self, symbol: Symbol) -> String;
    fn getAugmentedPropertiesOfType(&self, type_: &dyn Type) -> Vec<&Symbol>;

    fn getRootSymbols(&self, symbol: Symbol) -> Vec<Symbol>;
    fn getSymbolOfExpando(&self, node: AstKind, allowDeclaration: bool) -> Option<Symbol>;
    fn getContextualType(&self, node: Expression) -> Option<&dyn Type>;
    /** @internal */
    fn getContextualTypeWithFlags(&self, node: Expression, contextFlags: Option<ContextFlags>) -> Option<&dyn Type>;
    /** @internal */
    fn getContextualTypeForObjectLiteralElement(&self, element: ObjectLiteralElementLike) -> Option<&dyn Type>;
    /** @internal */
    fn getContextualTypeForArgumentAtIndex(&self, call: CallLikeExpression, argIndex: usize) -> Option<&dyn Type>;
    /** @internal */
    fn getContextualTypeForJsxAttribute(&self, attribute: JsxAttribute) -> Option<&dyn Type>;
    /** @internal */
    fn isContextSensitive(&self, node: Expression) -> bool;
    /** @internal */
    fn getTypeOfPropertyOfContextualType(&self, type_: &dyn Type, name: &str) -> Option<&dyn Type>;

    /// returns unknownSignature in the case of an error.
    /// returns undefined if the node is not valid.
    /// @param argument_count Apparent number of arguments, passed in case of a possibly incomplete call. This should come from an ArgumentListInfo. See `signatureHelp.ts`.
    fn getResolvedSignature(&self, node: CallLikeExpression, candidatesOutArray: Option<Vec<Signature>>, argumentCount: Option<usize>) -> Option<Signature>;
    /** @internal */
    fn getResolvedSignatureForSignatureHelp(&self, node: CallLikeExpression, candidatesOutArray: Option<Vec<Signature>>, argumentCount: Option<usize>) -> Option<Signature>;
    /** @internal */
    fn getCandidateSignaturesForStringLiteralCompletions(&self, call: CallLikeExpression, editingArgument: AstKind) -> Vec<Signature>;
    /** @internal */
    fn getExpandedParameters(&self, sig: Signature) -> Vec<Vec<Symbol>>;
    /** @internal */
    fn hasEffectiveRestParameter(&self, sig: Signature) -> bool;
    /** @internal */
    fn containsArgumentsReference(&self, declaration: SignatureDeclaration) -> bool;

    fn getSignatureFromDeclaration(&self, declaration: SignatureDeclaration) -> Option<Signature>;
    fn isImplementationOfOverload(&self, node: SignatureDeclaration) -> Option<bool>;
    fn isUndefinedSymbol(&self, symbol: Symbol) -> bool;
    fn isArgumentsSymbol(&self, symbol: Symbol) -> bool;
    fn isUnknownSymbol(&self, symbol: Symbol) -> bool;
    fn getMergedSymbol(&self, symbol: Symbol) -> Symbol;
    /** @internal */
    fn symbolIsValue(&self, symbol: Symbol, includeTypeOnlyMembers: Option<bool>) -> bool;

    fn getConstantValue(&self, node: EnumMember) -> Option<String>;
    fn isValidPropertyAccess(&self, node: PropertyAccessExpression, propertyName: &str) -> bool;
    /// Exclude accesses to private properties.
    ///
    /** @internal */
    fn isValidPropertyAccessForCompletions(&self, node: PropertyAccessExpression, type_: &dyn Type, property: Symbol) -> bool;
    /// Follow all aliases to get the original symbol.
    fn getAliasedSymbol(&self, symbol: Symbol) -> Symbol;
    /// Follow a *single* alias to get the immediately aliased symbol.
    fn getImmediateAliasedSymbol(&self, symbol: Symbol) -> Option<Symbol>;
    fn getExportsOfModule(&self, moduleSymbol: Symbol) -> Vec<Symbol>;
    /// Unlike `get_exports_of_module`, this includes properties of an `export =` value.
    ///
    /** @internal */
    fn getExportsAndPropertiesOfModule(&self, moduleSymbol: Symbol) -> Vec<Symbol>;
    /** @internal */
    // fn forEachExportAndPropertyOfModule(&self, moduleSymbol: Symbol, cb: impl Fn(Symbol, &str));
    fn getJsxIntrinsicTagNamesAt(&self, location: AstKind) -> Vec<Symbol>;
    fn isOptionalParameter(&self, node: Argument) -> bool;
    fn getAmbientModules(&self) -> Vec<Symbol>;

    fn tryGetMemberInModuleExports(&self, memberName: &str, moduleSymbol: Symbol) -> Option<Symbol>;
    /// Unlike `try_get_member_in_module_exports`, this includes properties of an `export =` value.
    /// Does *not* return properties of primitive types.
    ///
    /** @internal */
    fn tryGetMemberInModuleExportsAndProperties(&self, memberName: &str, moduleSymbol: Symbol) -> Option<Symbol>;
    fn getApparentType(&self, type_: &dyn Type) -> &dyn Type;
    /** @internal */
    fn getSuggestedSymbolForNonexistentProperty(&self, name: MemberName, containingType: &dyn Type) -> Option<Symbol>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentJsxAttribute(&self, name: Identifier, containingType: &dyn Type) -> Option<Symbol>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentSymbol(&self, location: AstKind, name: &str, meaning: SymbolFlags) -> Option<Symbol>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentModule(&self, node: Identifier, target: Symbol) -> Option<Symbol>;
    /** @internal */
    fn getSuggestedSymbolForNonexistentClassMember(&self, name: &str, baseType: &dyn Type) -> Option<Symbol>;
    fn getBaseConstraintOfType(&self, type_: &dyn Type) -> Option<&dyn Type>;
    fn getDefaultFromTypeParameter(&self, type_: &dyn Type) -> Option<&dyn Type>;

    /// Gets the intrinsic `any` type. There are multiple types that act as `any` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `any`. Instead, use `type.flags & TypeFlags.Any`.
    fn getAnyType(&self) -> &dyn Type;
    fn getStringType(&self) -> &dyn Type;
    fn getStringLiteralType(&self, value: &str) -> StringLiteralType;
    fn getNumberType(&self) -> &dyn Type;
    fn getNumberLiteralType(&self, value: f64) -> NumberLiteralType;
    fn getBigIntType(&self) -> &dyn Type;
    fn getBigIntLiteralType(&self, value: PseudoBigInt) -> BigIntLiteralType;
    fn getBooleanType(&self) -> &dyn Type;
    /** @internal */
    fn getFalseType(&self, fresh: Option<bool>) -> &dyn Type;
    /** @internal */
    fn getTrueType(&self, fresh: Option<bool>) -> &dyn Type;
    fn getVoidType(&self) -> &dyn Type;
    /// Gets the intrinsic `undefined` type. There are multiple types that act as `undefined` used internally in the compiler
    /// depending on compiler options, so the type returned by this function should not be used in equality checks to determine
    /// if another type is `undefined`. Instead, use `type.flags & TypeFlags.Undefined`.
    fn getUndefinedType(&self) -> &dyn Type;
    /// Gets the intrinsic `null` type. There are multiple types that act as `null` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `null`. Instead, use `type.flags & TypeFlags.Null`.
    fn getNullType(&self) -> &dyn Type;
    fn getESSymbolType(&self) -> &dyn Type;
    /// Gets the intrinsic `never` type. There are multiple types that act as `never` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `never`. Instead, use `type.flags & TypeFlags.Never`.
    fn getNeverType(&self) -> &dyn Type;
    /** @internal */
    fn getOptionalType(&self) -> &dyn Type;
    /** @internal */
    fn getUnionType(&self, types: Vec<&dyn Type>, subtypeReduction: Option<UnionReduction>) -> &dyn Type;
    /** @internal */
    fn createArrayType(&self, elementType: &dyn Type) -> &dyn Type;
    /** @internal */
    fn getElementTypeOfArrayType(&self, arrayType: &dyn Type) -> Option<&dyn Type>;
    /** @internal */
    fn createPromiseType(&self, type_: &dyn Type) -> &dyn Type;
    /** @internal */
    fn getPromiseType(&self) -> &dyn Type;
    /** @internal */
    fn getPromiseLikeType(&self) -> &dyn Type;
    /** @internal */
    fn getAnyAsyncIterableType(&self) -> Option<&dyn Type>;

    /// Returns true if the "source" type is assignable to the "target" type.
    fn isTypeAssignableTo(&self, source: &dyn Type, target: &dyn Type) -> bool;
    /** @internal */
    fn createAnonymousType(&self, symbol: Option<Symbol>, members: SymbolTable, callSignatures: Vec<Signature>, constructSignatures: Vec<Signature>, indexInfos: Vec<IndexInfo>) -> &dyn Type;
    /** @internal */
    fn createSignature(
        &self, declaration: Option<SignatureDeclaration>, typeParameters: Option<Vec<TypeParameter>>, thisParameter: Option<Symbol>, parameters: Vec<Symbol>, resolvedReturnType: &dyn Type, typePredicate: Option<TypePredicate>, minArgumentCount: usize, flags: SignatureFlags,
    ) -> Signature;
    /** @internal */
    fn createSymbol(&self, flags: SymbolFlags, name: &str) -> TransientSymbol;
    /** @internal */
    fn createIndexInfo(&self, keyType: &dyn Type, type_: &dyn Type, isReadonly: bool, declaration: Option<SignatureDeclaration>) -> IndexInfo;
    /** @internal */
    fn isSymbolAccessible(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: SymbolFlags, shouldComputeAliasToMarkVisible: bool) -> SymbolAccessibilityResult;
    /** @internal */
    fn tryFindAmbientModule(&self, moduleName: &str) -> Option<Symbol>;

    /** @internal */
    fn getSymbolWalker(&self, accept: Option<fn(Symbol) -> bool>) -> SymbolWalker;

    // Should not be called directly.  Should only be accessed through the Program instance.
    /** @internal */
    fn getDiagnostics(&self, sourceFile: Option<SourceFile>, cancellationToken: Option<CancellationToken>, nodesToCheck: Option<Vec<AstKind>>) -> Vec<Diagnostic>;
    /** @internal */
    fn getGlobalDiagnostics(&self) -> Vec<Diagnostic>;
    /** @internal */
    fn getEmitResolver(&self, sourceFile: Option<SourceFile>, cancellationToken: Option<CancellationToken>, forceDts: Option<bool>) -> EmitResolver;
    /** @internal */
    fn requiresAddingImplicitUndefined(&self, parameter: Argument, enclosingDeclaration: Option<AstKind>) -> bool;

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
    fn getRecursionIdentity(&self, type_: &dyn Type) -> Option<&dyn Type>;
    /** @internal */
    fn getUnmatchedProperties(&self, source: &dyn Type, target: &dyn Type, requireOptionalProperties: bool, matchDiscriminantProperties: bool) -> Box<dyn Iterator<Item = Symbol>>;

    /// True if this type is the `Array` or `ReadonlyArray` type from lib.d.ts.
    /// This function will _not_ return true if passed a type which
    /// extends `Array` (for example, the TypeScript AST's `NodeArray` type).
    fn isArrayType(&self, type_: &dyn Type) -> bool;
    /// True if this type is a tuple type. This function will _not_ return true if
    /// passed a type which extends from a tuple.
    fn isTupleType(&self, type_: &dyn Type) -> bool;
    /// True if this type is assignable to `ReadonlyArray<any>`.
    fn isArrayLikeType(&self, type_: &dyn Type) -> bool;

    /// True if `contextualType` should not be considered for completions because
    /// e.g. it specifies `kind: "a"` and obj has `kind: "b"`.
    ///
    /** @internal */
    fn isTypeInvalidDueToUnionDiscriminant(&self, contextualType: &dyn Type, obj: ObjectExpression) -> bool;
    /** @internal */
    fn getExactOptionalProperties(&self, type_: &dyn Type) -> Vec<Symbol>;
    /// For a union, will include a property if it's defined in *any* of the member types.
    /// So for `{ a } | { b }`, this will include both `a` and `b`.
    /// Does not include properties of primitive types.
    ///
    /** @internal */
    fn getAllPossiblePropertiesOfTypes(&self, types: Vec<&dyn Type>) -> Vec<Symbol>;
    fn resolveName(&self, name: &str, location: Option<AstKind>, meaning: SymbolFlags, excludeGlobals: bool) -> Option<Symbol>;
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
    fn getAccessibleSymbolChain(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: SymbolFlags, useOnlyExternalAliasing: bool) -> Option<Vec<Symbol>>;
    fn getTypePredicateOfSignature(&self, signature: Signature) -> Option<TypePredicate>;
    /** @internal */
    fn resolveExternalModuleName(&self, moduleSpecifier: Expression) -> Option<Symbol>;
    /// An external module with an 'export =' declaration resolves to the target of the 'export =' declaration,
    /// and an external module with no 'export =' declaration resolves to the module itself.
    ///
    /** @internal */
    fn resolveExternalModuleSymbol(&self, symbol: Symbol) -> Symbol;
    /// @param node A location where we might consider accessing `this`. Not necessarily a ThisExpression.
    ///
    /** @internal */
    fn tryGetThisTypeAt(&self, node: AstKind, includeGlobalThis: Option<bool>, container: Option<ThisContainer>) -> Option<&dyn Type>;
    /** @internal */
    fn getTypeArgumentConstraint(&self, node: TypeNode) -> Option<&dyn Type>;

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
    fn getLocalTypeParametersOfClassOrInterfaceOrTypeAlias(&self, symbol: Symbol) -> Option<Vec<TypeParameter>>;
    /** @internal */
    fn isDeclarationVisible(&self, node: Declaration) -> bool;
    /** @internal */
    fn isPropertyAccessible(&self, node: AstKind, isSuper: bool, isWrite: bool, containingType: &dyn Type, property: Symbol) -> bool;
    /** @internal */
    fn getTypeOnlyAliasDeclaration(&self, symbol: Symbol) -> Option<TypeOnlyAliasDeclaration>;
    /** @internal */
    fn getMemberOverrideModifierStatus(&self, node: ClassLikeDeclaration, member: ClassElement, memberSymbol: Symbol) -> MemberOverrideStatus;
    /** @internal */
    fn isTypeParameterPossiblyReferenced(&self, tp: TypeParameter, node: AstKind) -> bool;
    /** @internal */
    fn typeHasCallOrConstructSignatures(&self, type_: &dyn Type) -> bool;
    /** @internal */
    fn getSymbolFlags(&self, symbol: Symbol) -> SymbolFlags;
}
// 5416

#[derive(Debug)]
pub enum SignatureKind {
    Call,
    Construct,
}

#[derive(Debug, Clone, Copy)]
pub struct SignatureFlags(pub isize);

impl SignatureFlags {
    pub const None: SignatureFlags = SignatureFlags(0);

    // Propagating flags
    pub const HasRestParameter: SignatureFlags = SignatureFlags(1 << 0); // Indicates last parameter is rest parameter
    pub const HasLiteralTypes: SignatureFlags = SignatureFlags(1 << 1); // Indicates signature is specialized
    pub const Abstract: SignatureFlags = SignatureFlags(1 << 2); // Indicates signature comes from an abstract class, abstract construct signature, or abstract constructor type

    // Non-propagating flags
    pub const IsInnerCallChain: SignatureFlags = SignatureFlags(1 << 3); // Indicates signature comes from a CallChain nested in an outer OptionalChain
    pub const IsOuterCallChain: SignatureFlags = SignatureFlags(1 << 4); // Indicates signature comes from a CallChain that is the outermost chain of an optional expression
    pub const IsUntypedSignatureInJsFile: SignatureFlags = SignatureFlags(1 << 5); // Indicates signature is from a js file and has no types
    pub const IsNonInferrable: SignatureFlags = SignatureFlags(1 << 6); // Indicates signature comes from a non-inferrable type
    pub const IsSignatureCandidateForOverloadFailure: SignatureFlags = SignatureFlags(1 << 7);

    pub const PropagatingFlags: SignatureFlags = SignatureFlags(Self::HasRestParameter.0 | Self::HasLiteralTypes.0 | Self::Abstract.0 | Self::IsUntypedSignatureInJsFile.0 | Self::IsSignatureCandidateForOverloadFailure.0);

    pub const CallChainFlags: SignatureFlags = SignatureFlags(Self::IsInnerCallChain.0 | Self::IsOuterCallChain.0);
}

#[derive(Debug)]
pub struct Signature {
    /** @internal */
    pub flags: SignatureFlags,
    /** @internal */
    pub checker: Option<Box<dyn TypeCheckerTrait>>,
    pub declaration: Option<SignatureDeclaration>,  // Originating declaration
    pub typeParameters: Option<Vec<TypeParameter>>, // Type parameters (undefined if non-generic)
    pub parameters: Vec<Symbol>,                    // Parameters
    pub thisParameter: Option<Symbol>,              // symbol of this-type parameter
    /** @internal */
    pub resolvedReturnType: Option<Box<dyn Type>>, // Lazily set by `getReturnTypeOfSignature`
    /** @internal */
    pub resolvedTypePredicate: Option<TypePredicate>, // Lazily set by `getTypePredicateOfSignature`
    /** @internal */
    pub minArgumentCount: i32,  // Number of non-optional parameters
    /** @internal */
    pub resolvedMinArgumentCount: Option<i32>, // Number of non-optional parameters (excluding trailing `void`)
    /** @internal */
    pub target: Option<Box<Signature>>, // Instantiation target
    /** @internal */
    pub mapper: Option<TypeMapper>, // Instantiation mapper
    /** @internal */
    pub compositeSignatures: Option<Vec<Signature>>, // Underlying signatures of a union/intersection signature
    /** @internal */
    pub compositeKind: Option<TypeFlags>, // TypeFlags.Union if the underlying signatures are from union members, otherwise TypeFlags.Intersection
    /** @internal */
    pub erasedSignatureCache: Option<Box<Signature>>, // Erased version of signature (deferred)
    /** @internal */
    pub canonicalSignatureCache: Option<Box<Signature>>, // Canonical version of signature (deferred)
    /** @internal */
    pub baseSignatureCache: Option<Box<Signature>>, // Base version of signature (deferred)
    /** @internal */
    pub optionalCallSignatureCache: Option<OptionalCallSignatureCache>, // Optional chained call version of signature (deferred)
    /** @internal */
    pub isolatedSignatureType: Option<Box<dyn ObjectType>>, // A manufactured type that just contains the signature for purposes of signature comparison
    /** @internal */
    pub instantiations: Option<HashMap<String, Signature>>, // Generic signature instantiation cache
    /** @internal */
    pub implementationSignatureCache: Option<Box<Signature>>, // Copy of the signature with fresh type parameters to use in checking the body of a potentially self-referential generic function (deferred)
}

#[derive(Debug)]
pub struct OptionalCallSignatureCache {
    pub inner: Option<Box<Signature>>,
    pub outer: Option<Box<Signature>>,
}

// region: 5864
#[derive(Debug, Clone, Copy)]
pub struct SymbolFlags(pub isize);

impl SymbolFlags {
    pub const None: SymbolFlags = SymbolFlags(0);
    pub const FunctionScopedVariable: SymbolFlags = SymbolFlags(1 << 0); // Variable (var) or parameter
    pub const BlockScopedVariable: SymbolFlags = SymbolFlags(1 << 1); // A block-scoped variable (let or const)
    pub const Property: SymbolFlags = SymbolFlags(1 << 2); // Property or enum member
    pub const EnumMember: SymbolFlags = SymbolFlags(1 << 3); // Enum member
    pub const Function: SymbolFlags = SymbolFlags(1 << 4); // Function
    pub const Class: SymbolFlags = SymbolFlags(1 << 5); // Class
    pub const Interface: SymbolFlags = SymbolFlags(1 << 6); // Interface
    pub const ConstEnum: SymbolFlags = SymbolFlags(1 << 7); // Const enum
    pub const RegularEnum: SymbolFlags = SymbolFlags(1 << 8); // Enum
    pub const ValueModule: SymbolFlags = SymbolFlags(1 << 9); // Instantiated module
    pub const NamespaceModule: SymbolFlags = SymbolFlags(1 << 10); // Uninstantiated module
    pub const TypeLiteral: SymbolFlags = SymbolFlags(1 << 11); // Type Literal or mapped type
    pub const ObjectLiteral: SymbolFlags = SymbolFlags(1 << 12); // Object Literal
    pub const Method: SymbolFlags = SymbolFlags(1 << 13); // Method
    pub const Constructor: SymbolFlags = SymbolFlags(1 << 14); // Constructor
    pub const GetAccessor: SymbolFlags = SymbolFlags(1 << 15); // Get accessor
    pub const SetAccessor: SymbolFlags = SymbolFlags(1 << 16); // Set accessor
    pub const Signature: SymbolFlags = SymbolFlags(1 << 17); // Call, construct, or index signature
    pub const TypeParameter: SymbolFlags = SymbolFlags(1 << 18); // Type parameter
    pub const TypeAlias: SymbolFlags = SymbolFlags(1 << 19); // Type alias
    pub const ExportValue: SymbolFlags = SymbolFlags(1 << 20); // Exported value marker (see comment in declareModuleMember in binder)
    pub const Alias: SymbolFlags = SymbolFlags(1 << 21); // An alias for another symbol (see comment in isAliasSymbolDeclaration in checker)
    pub const Prototype: SymbolFlags = SymbolFlags(1 << 22); // Prototype property (no source representation)
    pub const ExportStar: SymbolFlags = SymbolFlags(1 << 23); // Export * declaration
    pub const Optional: SymbolFlags = SymbolFlags(1 << 24); // Optional property
    pub const Transient: SymbolFlags = SymbolFlags(1 << 25); // Transient symbol (created during type check)
    pub const Assignment: SymbolFlags = SymbolFlags(1 << 26); // Assignment treated as declaration (eg `this.prop = 1`)
    pub const ModuleExports: SymbolFlags = SymbolFlags(1 << 27); // Symbol for CommonJS `module` of `module.exports`
    pub const All: SymbolFlags = SymbolFlags(-1);

    pub const Enum: SymbolFlags = SymbolFlags(Self::RegularEnum.0 | Self::ConstEnum.0);
    pub const Variable: SymbolFlags = SymbolFlags(Self::FunctionScopedVariable.0 | Self::BlockScopedVariable.0);
    pub const Value: SymbolFlags = SymbolFlags(Self::Variable.0 | Self::Property.0 | Self::EnumMember.0 | Self::ObjectLiteral.0 | Self::Function.0 | Self::Class.0 | Self::Enum.0 | Self::ValueModule.0 | Self::Method.0 | Self::GetAccessor.0 | Self::SetAccessor.0);
    pub const Type: SymbolFlags = SymbolFlags(Self::Class.0 | Self::Interface.0 | Self::Enum.0 | Self::EnumMember.0 | Self::TypeLiteral.0 | Self::TypeParameter.0 | Self::TypeAlias.0);
    pub const Namespace: SymbolFlags = SymbolFlags(Self::ValueModule.0 | Self::NamespaceModule.0 | Self::Enum.0);
    pub const Module: SymbolFlags = SymbolFlags(Self::ValueModule.0 | Self::NamespaceModule.0);
    pub const Accessor: SymbolFlags = SymbolFlags(Self::GetAccessor.0 | Self::SetAccessor.0);

    // Variables can be redeclared, but can not redeclare a block-scoped declaration with the
    // same name, or any other value that is not a variable, e.g. ValueModule or Class
    pub const FunctionScopedVariableExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !Self::FunctionScopedVariable.0);

    // Block-scoped declarations are not allowed to be re-declared
    // they can not merge with anything in the value space
    pub const BlockScopedVariableExcludes: SymbolFlags = Self::Value;

    pub const ParameterExcludes: SymbolFlags = Self::Value;
    pub const PropertyExcludes: SymbolFlags = Self::None;
    pub const EnumMemberExcludes: SymbolFlags = SymbolFlags(Self::Value.0 | Self::Type.0);
    pub const FunctionExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !(Self::Function.0 | Self::ValueModule.0 | Self::Class.0));
    pub const ClassExcludes: SymbolFlags = SymbolFlags((Self::Value.0 | Self::Type.0) & !(Self::ValueModule.0 | Self::Interface.0 | Self::Function.0)); // class-interface mergability done in checker.ts
    pub const InterfaceExcludes: SymbolFlags = SymbolFlags(Self::Type.0 & !(Self::Interface.0 | Self::Class.0));
    pub const RegularEnumExcludes: SymbolFlags = SymbolFlags((Self::Value.0 | Self::Type.0) & !(Self::RegularEnum.0 | Self::ValueModule.0)); // regular enums merge only with regular enums and modules
    pub const ConstEnumExcludes: SymbolFlags = SymbolFlags((Self::Value.0 | Self::Type.0) & !Self::ConstEnum.0); // const enums merge only with const enums
    pub const ValueModuleExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !(Self::Function.0 | Self::Class.0 | Self::RegularEnum.0 | Self::ValueModule.0));
    pub const NamespaceModuleExcludes: SymbolFlags = Self::None;
    pub const MethodExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !Self::Method.0);
    pub const GetAccessorExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !Self::SetAccessor.0);
    pub const SetAccessorExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !Self::GetAccessor.0);
    pub const AccessorExcludes: SymbolFlags = SymbolFlags(Self::Value.0 & !Self::Accessor.0);
    pub const TypeParameterExcludes: SymbolFlags = SymbolFlags(Self::Type.0 & !Self::TypeParameter.0);
    pub const TypeAliasExcludes: SymbolFlags = Self::Type;
    pub const AliasExcludes: SymbolFlags = Self::Alias;

    pub const ModuleMember: SymbolFlags = SymbolFlags(Self::Variable.0 | Self::Function.0 | Self::Class.0 | Self::Interface.0 | Self::Enum.0 | Self::Module.0 | Self::TypeAlias.0 | Self::Alias.0);

    pub const ExportHasLocal: SymbolFlags = SymbolFlags(Self::Function.0 | Self::Class.0 | Self::Enum.0 | Self::ValueModule.0);

    pub const BlockScoped: SymbolFlags = SymbolFlags(Self::BlockScopedVariable.0 | Self::Class.0 | Self::Enum.0);

    pub const PropertyOrAccessor: SymbolFlags = SymbolFlags(Self::Property.0 | Self::Accessor.0);

    pub const ClassMember: SymbolFlags = SymbolFlags(Self::Method.0 | Self::Accessor.0 | Self::Property.0);

    /** @internal */
    pub const ExportSupportsDefaultModifier: SymbolFlags = SymbolFlags(Self::Class.0 | Self::Function.0 | Self::Interface.0);

    /** @internal */
    pub const ExportDoesNotSupportDefaultModifier: SymbolFlags = SymbolFlags(!Self::ExportSupportsDefaultModifier.0);

    /** @internal */
    // The set of things we consider semantically classifiable.  Used to speed up the LS during
    // classification.
    pub const Classifiable: SymbolFlags = SymbolFlags(Self::Class.0 | Self::Enum.0 | Self::TypeAlias.0 | Self::Interface.0 | Self::TypeParameter.0 | Self::Module.0 | Self::Alias.0);

    /** @internal */
    pub const LateBindingContainer: SymbolFlags = SymbolFlags(Self::Class.0 | Self::Interface.0 | Self::TypeLiteral.0 | Self::ObjectLiteral.0 | Self::Function.0);

    pub fn contains(&self, flags: SymbolFlags) -> bool { (self.0 & flags.0) == flags.0 }
}

impl std::ops::BitOr for SymbolFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self { SymbolFlags(self.0 | rhs.0) }
}

impl std::ops::BitAnd for SymbolFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self { SymbolFlags(self.0 & rhs.0) }
}

/** @internal */
pub type SymbolId = usize;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub flags: SymbolFlags,                     // Symbol flags
    pub escapedName: String,                    // Name of symbol
    pub declarations: Option<Vec<Declaration>>, // Declarations associated with this symbol
    pub valueDeclaration: Option<Declaration>,  // First value declaration of the symbol
    pub members: Option<SymbolTable>,           // Class, interface or object literal instance members
    pub exports: Option<SymbolTable>,           // Module exports
    pub globalExports: Option<SymbolTable>,     // Conditional global UMD exports
    /** @internal */
    pub id: SymbolId,       // Unique id (used to look up SymbolLinks)
    /** @internal */
    pub mergeId: usize,     // Merge id (used to look up merged symbol)
    /** @internal */
    pub parent: Option<Box<Symbol>>, // Parent symbol
    /** @internal */
    pub exportSymbol: Option<Box<Symbol>>, // Exported symbol associated with this symbol
    /** @internal */
    pub constEnumOnlyModule: Option<bool>, // True if module contains only const enums or other modules with only const enums
    /** @internal */
    pub isReferenced: Option<SymbolFlags>, // True if the symbol is referenced elsewhere. Keeps track of the meaning of a reference in case a symbol is both a type parameter and parameter.
    /** @internal */
    pub lastAssignmentPos: Option<usize>, // Source position of last node that assigns value to symbol
    /** @internal */
    pub isReplaceableByMethod: Option<bool>, // Can this Javascript class property be replaced by a method symbol?
    /** @internal */
    pub assignmentDeclarationMembers: Option<HashMap<usize, Declaration>>, // detected late-bound assignment declarations associated with the symbol
}
// endregion: 5976

// region: 6246
#[derive(Debug, Clone, Copy)]
pub struct TypeFlags(pub isize);

impl TypeFlags {
    pub const Any: TypeFlags = TypeFlags(1 << 0);
    pub const Unknown: TypeFlags = TypeFlags(1 << 1);
    pub const String: TypeFlags = TypeFlags(1 << 2);
    pub const Number: TypeFlags = TypeFlags(1 << 3);
    pub const Boolean: TypeFlags = TypeFlags(1 << 4);
    pub const Enum: TypeFlags = TypeFlags(1 << 5); // Numeric computed enum member value
    pub const BigInt: TypeFlags = TypeFlags(1 << 6);
    pub const StringLiteral: TypeFlags = TypeFlags(1 << 7);
    pub const NumberLiteral: TypeFlags = TypeFlags(1 << 8);
    pub const BooleanLiteral: TypeFlags = TypeFlags(1 << 9);
    pub const EnumLiteral: TypeFlags = TypeFlags(1 << 10); // Always combined with StringLiteral, NumberLiteral, or Union
    pub const BigIntLiteral: TypeFlags = TypeFlags(1 << 11);
    pub const ESSymbol: TypeFlags = TypeFlags(1 << 12); // Type of symbol primitive introduced in ES6
    pub const UniqueESSymbol: TypeFlags = TypeFlags(1 << 13); // unique symbol
    pub const Void: TypeFlags = TypeFlags(1 << 14);
    pub const Undefined: TypeFlags = TypeFlags(1 << 15);
    pub const Null: TypeFlags = TypeFlags(1 << 16);
    pub const Never: TypeFlags = TypeFlags(1 << 17); // Never type
    pub const TypeParameter: TypeFlags = TypeFlags(1 << 18); // Type parameter
    pub const Object: TypeFlags = TypeFlags(1 << 19); // Object type
    pub const Union: TypeFlags = TypeFlags(1 << 20); // Union (T | U)
    pub const Intersection: TypeFlags = TypeFlags(1 << 21); // Intersection (T & U)
    pub const Index: TypeFlags = TypeFlags(1 << 22); // keyof T
    pub const IndexedAccess: TypeFlags = TypeFlags(1 << 23); // T[K]
    pub const Conditional: TypeFlags = TypeFlags(1 << 24); // T extends U ? X : Y
    pub const Substitution: TypeFlags = TypeFlags(1 << 25); // Type parameter substitution
    pub const NonPrimitive: TypeFlags = TypeFlags(1 << 26); // intrinsic object type
    pub const TemplateLiteral: TypeFlags = TypeFlags(1 << 27); // Template literal type
    pub const StringMapping: TypeFlags = TypeFlags(1 << 28); // Uppercase/Lowercase type
    /** @internal */
    pub const Reserved1: TypeFlags = TypeFlags(1 << 29); // Used by union/intersection type construction
    /** @internal */
    pub const Reserved2: TypeFlags = TypeFlags(1 << 30); // Used by union/intersection type construction

    /** @internal */
    pub const AnyOrUnknown: TypeFlags = TypeFlags(Self::Any.0 | Self::Unknown.0);
    /** @internal */
    pub const Nullable: TypeFlags = TypeFlags(Self::Undefined.0 | Self::Null.0);
    pub const Literal: TypeFlags = TypeFlags(Self::StringLiteral.0 | Self::NumberLiteral.0 | Self::BigIntLiteral.0 | Self::BooleanLiteral.0);
    pub const Unit: TypeFlags = TypeFlags(Self::Enum.0 | Self::Literal.0 | Self::UniqueESSymbol.0 | Self::Nullable.0);
    pub const Freshable: TypeFlags = TypeFlags(Self::Enum.0 | Self::Literal.0);
    pub const StringOrNumberLiteral: TypeFlags = TypeFlags(Self::StringLiteral.0 | Self::NumberLiteral.0);
    /** @internal */
    pub const StringOrNumberLiteralOrUnique: TypeFlags = TypeFlags(Self::StringLiteral.0 | Self::NumberLiteral.0 | Self::UniqueESSymbol.0);
    /** @internal */
    pub const DefinitelyFalsy: TypeFlags = TypeFlags(Self::StringLiteral.0 | Self::NumberLiteral.0 | Self::BigIntLiteral.0 | Self::BooleanLiteral.0 | Self::Void.0 | Self::Undefined.0 | Self::Null.0);
    pub const PossiblyFalsy: TypeFlags = TypeFlags(Self::DefinitelyFalsy.0 | Self::String.0 | Self::Number.0 | Self::BigInt.0 | Self::Boolean.0);
    /** @internal */
    pub const Intrinsic: TypeFlags = TypeFlags(Self::Any.0 | Self::Unknown.0 | Self::String.0 | Self::Number.0 | Self::BigInt.0 | Self::Boolean.0 | Self::BooleanLiteral.0 | Self::ESSymbol.0 | Self::Void.0 | Self::Undefined.0 | Self::Null.0 | Self::Never.0 | Self::NonPrimitive.0);
    pub const StringLike: TypeFlags = TypeFlags(Self::String.0 | Self::StringLiteral.0 | Self::TemplateLiteral.0 | Self::StringMapping.0);
    pub const NumberLike: TypeFlags = TypeFlags(Self::Number.0 | Self::NumberLiteral.0 | Self::Enum.0);
    pub const BigIntLiteralLike: TypeFlags = TypeFlags(Self::BigInt.0 | Self::BigIntLiteral.0);
    pub const BooleanLike: TypeFlags = TypeFlags(Self::Boolean.0 | Self::BooleanLiteral.0);
    pub const EnumLike: TypeFlags = TypeFlags(Self::Enum.0 | Self::EnumLiteral.0);
    pub const ESSymbolLike: TypeFlags = TypeFlags(Self::ESSymbol.0 | Self::UniqueESSymbol.0);
    pub const VoidLike: TypeFlags = TypeFlags(Self::Void.0 | Self::Undefined.0);
    /** @internal */
    pub const Primitive: TypeFlags = TypeFlags(Self::StringLike.0 | Self::NumberLike.0 | Self::BigIntLiteralLike.0 | Self::BooleanLike.0 | Self::EnumLike.0 | Self::ESSymbolLike.0 | Self::VoidLike.0 | Self::Null.0);
    /** @internal */
    pub const DefinitelyNonNullable: TypeFlags = TypeFlags(Self::StringLike.0 | Self::NumberLike.0 | Self::BigIntLiteralLike.0 | Self::BooleanLike.0 | Self::EnumLike.0 | Self::ESSymbolLike.0 | Self::Object.0 | Self::NonPrimitive.0);
    /** @internal */
    pub const DisjointDomains: TypeFlags = TypeFlags(Self::NonPrimitive.0 | Self::StringLike.0 | Self::NumberLike.0 | Self::BigIntLiteralLike.0 | Self::BooleanLike.0 | Self::ESSymbolLike.0 | Self::VoidLike.0 | Self::Null.0);
    pub const UnionOrIntersection: TypeFlags = TypeFlags(Self::Union.0 | Self::Intersection.0);
    pub const StructuredType: TypeFlags = TypeFlags(Self::Object.0 | Self::Union.0 | Self::Intersection.0);
    pub const TypeVariable: TypeFlags = TypeFlags(Self::TypeParameter.0 | Self::IndexedAccess.0);
    pub const InstantiableNonPrimitive: TypeFlags = TypeFlags(Self::TypeVariable.0 | Self::Conditional.0 | Self::Substitution.0);
    pub const InstantiablePrimitive: TypeFlags = TypeFlags(Self::Index.0 | Self::TemplateLiteral.0 | Self::StringMapping.0);
    pub const Instantiable: TypeFlags = TypeFlags(Self::InstantiableNonPrimitive.0 | Self::InstantiablePrimitive.0);
    pub const StructuredOrInstantiable: TypeFlags = TypeFlags(Self::StructuredType.0 | Self::Instantiable.0);
    /** @internal */
    pub const ObjectFlagsType: TypeFlags = TypeFlags(Self::Any.0 | Self::Nullable.0 | Self::Never.0 | Self::Object.0 | Self::Union.0 | Self::Intersection.0);
    /** @internal */
    pub const Simplifiable: TypeFlags = TypeFlags(Self::IndexedAccess.0 | Self::Conditional.0);
    /** @internal */
    pub const Singleton: TypeFlags = TypeFlags(Self::Any.0 | Self::Unknown.0 | Self::String.0 | Self::Number.0 | Self::Boolean.0 | Self::BigInt.0 | Self::ESSymbol.0 | Self::Void.0 | Self::Undefined.0 | Self::Null.0 | Self::Never.0 | Self::NonPrimitive.0);
    // 'Narrowable' types are types where narrowing actually narrows.
    // This *should* be every type other than null, undefined, void, and never
    pub const Narrowable: TypeFlags = TypeFlags(Self::Any.0 | Self::Unknown.0 | Self::StructuredOrInstantiable.0 | Self::StringLike.0 | Self::NumberLike.0 | Self::BigIntLiteralLike.0 | Self::BooleanLike.0 | Self::ESSymbol.0 | Self::UniqueESSymbol.0 | Self::NonPrimitive.0);
    // The following flags are aggregated during union and intersection type construction
    /** @internal */
    pub const IncludesMask: TypeFlags = TypeFlags(Self::Any.0 | Self::Unknown.0 | Self::Primitive.0 | Self::Never.0 | Self::Object.0 | Self::Union.0 | Self::Intersection.0 | Self::NonPrimitive.0 | Self::TemplateLiteral.0 | Self::StringMapping.0);
    // The following flags are used for different purposes during union and intersection type construction
    /** @internal */
    pub const IncludesMissingType: TypeFlags = TypeFlags(Self::TypeParameter.0);
    /** @internal */
    pub const IncludesNonWidenningType: TypeFlags = TypeFlags(Self::Index.0);
    /** @internal */
    pub const IncludesWildcard: TypeFlags = TypeFlags(Self::IndexedAccess.0);
    /** @internal */
    pub const IncludesEmptyObject: TypeFlags = TypeFlags(Self::Conditional.0);
    /** @internal */
    pub const IncludesInstantiable: TypeFlags = TypeFlags(Self::Substitution.0);
    /** @internal */
    pub const IncludesConstrainedTypeVariable: TypeFlags = TypeFlags(Self::Reserved1.0);
    /** @internal */
    pub const IncludesError: TypeFlags = TypeFlags(Self::Reserved2.0);
    /** @internal */
    pub const NotPrimitiveUnion: TypeFlags = TypeFlags(Self::Any.0 | Self::Unknown.0 | Self::Void.0 | Self::Never.0 | Self::Object.0 | Self::Intersection.0 | Self::IncludesInstantiable.0);

    pub fn contains(&self, flags: TypeFlags) -> bool { (self.0 & flags.0) == flags.0 }

    pub fn intersects(&self, flags: TypeFlags) -> bool { (self.0 & flags.0) != 0 }
}

impl std::ops::BitOr for TypeFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self { TypeFlags(self.0 | rhs.0) }
}

impl std::ops::BitAnd for TypeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self { TypeFlags(self.0 & rhs.0) }
}

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
    pub symbol: Symbol, // Symbol associated with type (if any)
    pub pattern: Option<DestructuringPattern<'a>>, // Destructuring pattern represented by type (if any)
    pub aliasSymbol: Option<Symbol>, // Alias associated with type
    pub aliasTypeArguments: Option<Vec<Box<dyn Type>>>, // Alias type arguments (if any)
    /** @internal */
    pub permissiveInstantiation: Option<Box<dyn Type>>, // Instantiation with type parameters mapped to wildcard type
    /** @internal */
    pub restrictiveInstantiation: Option<Box<dyn Type>>, // Instantiation with type parameters mapped to unconstrained form
    /** @internal */
    pub immediateBaseConstraint: Option<Box<dyn Type>>, // Immediate base constraint cache
    /** @internal */
    pub widened: Option<Box<dyn Type>>, // Cached widened form of the type

    pub object_flags: Option<ObjectFlags>,               // ObjectFlagsType
    pub intrinsic_props: Option<IntrinsicTypeProps>,     // IntrinsicType
    pub freshable_props: Option<FreshableTypeProps<'a>>, // FreshableType
    pub object_props: Option<ObjectTypeProps>,           // ObjectType
    pub interface_props: Option<InterfaceTypeProps>,     // InterfaceType
}
pub trait Type: std::fmt::Debug {
    fn getFlags(&self) -> TypeFlags;
    fn getSymbol(&self) -> Option<&Symbol>;
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
}

/** @internal */
// Intrinsic types (TypeFlags.Intrinsic)
#[derive(Debug, Clone)]
pub struct IntrinsicTypeProps {
    pub intrinsicName: String, // Name of intrinsic type
    pub debugIntrinsicName: Option<String>,
}

pub trait IntrinsicType: ObjectFlagsTrait {
    fn get_intrinsic_props(&self) -> &IntrinsicTypeProps;
}

/** @internal */
pub trait NullableType: IntrinsicType {}

#[derive(Debug, Clone)]
pub struct FreshableTypeProps<'a> {
    pub freshType: &'a dyn FreshableType,   // Fresh version of type
    pub regularType: &'a dyn FreshableType, // Regular version of type
}

pub trait FreshableType: Type {
    fn get_freshable_type_props(&self) -> &FreshableTypeProps;
}

/** @internal */
pub trait FreshableIntrinsicType: FreshableType + IntrinsicType {}
// endregion: 6392

// region: 6423
#[derive(Debug, Clone, Copy)]
pub struct ObjectFlags(pub isize);

// Types included in TypeFlags.ObjectFlagsType have an objectFlags property. Some ObjectFlags
// are specific to certain types and reuse the same bit position. Those ObjectFlags require a check
// for a certain TypeFlags value to determine their meaning.
impl ObjectFlags {
    pub const None: ObjectFlags = ObjectFlags(0);
    pub const Class: ObjectFlags = ObjectFlags(1 << 0);
    pub const Interface: ObjectFlags = ObjectFlags(1 << 1);
    pub const Reference: ObjectFlags = ObjectFlags(1 << 2);
    pub const Tuple: ObjectFlags = ObjectFlags(1 << 3);
    pub const Anonymous: ObjectFlags = ObjectFlags(1 << 4);
    pub const Mapped: ObjectFlags = ObjectFlags(1 << 5);
    pub const Instantiated: ObjectFlags = ObjectFlags(1 << 6);
    pub const ObjectLiteral: ObjectFlags = ObjectFlags(1 << 7);
    pub const EvolvingArray: ObjectFlags = ObjectFlags(1 << 8);
    pub const ObjectLiteralPatternWithComputedProperties: ObjectFlags = ObjectFlags(1 << 9);
    pub const ReverseMapped: ObjectFlags = ObjectFlags(1 << 10);
    pub const JSXAttributes: ObjectFlags = ObjectFlags(1 << 11);
    pub const JSLiteral: ObjectFlags = ObjectFlags(1 << 12);
    pub const FreshLiteral: ObjectFlags = ObjectFlags(1 << 13);
    pub const ArrayLiteral: ObjectFlags = ObjectFlags(1 << 14);
    pub const PrimitiveUnion: ObjectFlags = ObjectFlags(1 << 15);
    pub const ContainsWideningType: ObjectFlags = ObjectFlags(1 << 16);
    pub const ContainsObjectOrArrayLiteral: ObjectFlags = ObjectFlags(1 << 17);
    pub const NonInferrableType: ObjectFlags = ObjectFlags(1 << 18);
    pub const CouldContainTypeVariablesComputed: ObjectFlags = ObjectFlags(1 << 19);
    pub const CouldContainTypeVariables: ObjectFlags = ObjectFlags(1 << 20);
    pub const ContainsSpread: ObjectFlags = ObjectFlags(1 << 21);
    pub const ObjectRestType: ObjectFlags = ObjectFlags(1 << 22);
    pub const InstantiationExpressionType: ObjectFlags = ObjectFlags(1 << 23);
    pub const IsClassInstanceClone: ObjectFlags = ObjectFlags(1 << 24);
    pub const IdenticalBaseTypeCalculated: ObjectFlags = ObjectFlags(1 << 25);
    pub const IdenticalBaseTypeExists: ObjectFlags = ObjectFlags(1 << 26);
    pub const SingleSignatureType: ObjectFlags = ObjectFlags(1 << 27);

    pub const ClassOrInterface: ObjectFlags = ObjectFlags(Self::Class.0 | Self::Interface.0);
    pub const RequiresWidening: ObjectFlags = ObjectFlags(Self::ContainsWideningType.0 | Self::ContainsObjectOrArrayLiteral.0);
    pub const PropagatingFlags: ObjectFlags = ObjectFlags(Self::ContainsWideningType.0 | Self::ContainsObjectOrArrayLiteral.0 | Self::NonInferrableType.0);
    pub const InstantiatedMapped: ObjectFlags = ObjectFlags(Self::Mapped.0 | Self::Instantiated.0);
    pub const ObjectTypeKindMask: ObjectFlags = ObjectFlags(Self::ClassOrInterface.0 | Self::Reference.0 | Self::Tuple.0 | Self::Anonymous.0 | Self::Mapped.0 | Self::ReverseMapped.0 | Self::EvolvingArray.0);

    pub const IsGenericTypeComputed: ObjectFlags = ObjectFlags(1 << 21);
    pub const IsGenericObjectType: ObjectFlags = ObjectFlags(1 << 22);
    pub const IsGenericIndexType: ObjectFlags = ObjectFlags(1 << 23);
    pub const IsGenericType: ObjectFlags = ObjectFlags(Self::IsGenericObjectType.0 | Self::IsGenericIndexType.0);

    pub const ContainsIntersections: ObjectFlags = ObjectFlags(1 << 24);
    pub const IsUnknownLikeUnionComputed: ObjectFlags = ObjectFlags(1 << 25);
    pub const IsUnknownLikeUnion: ObjectFlags = ObjectFlags(1 << 26);

    pub const IsNeverIntersectionComputed: ObjectFlags = ObjectFlags(1 << 24);
    pub const IsNeverIntersection: ObjectFlags = ObjectFlags(1 << 25);
    pub const IsConstrainedTypeVariable: ObjectFlags = ObjectFlags(1 << 26);
}

impl std::ops::BitOr for ObjectFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { ObjectFlags(self.0 | rhs.0) }
}

impl std::ops::BitAnd for ObjectFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { ObjectFlags(self.0 & rhs.0) }
}

impl ObjectFlags {
    pub fn contains(&self, other: ObjectFlags) -> bool { (self.0 & other.0) == other.0 }
}

pub trait ObjectFlagsTrait: Type {
    fn get_object_flags(&self) -> ObjectFlags;
}

enum ObjectFlagsType {
    NullableType(Box<dyn NullableType>),
    ObjectType(Box<dyn ObjectType>),
    // UnionType(Box<dyn UnionType>),
    // IntersectionType(Box<dyn IntersectionType>),
}

#[derive(Debug)]
pub struct ObjectTypeProps {
    /** @internal */
    pub members: Option<SymbolTable>, // Properties by name
    /** @internal */
    pub properties: Option<Vec<Symbol>>, // Properties
    /** @internal */
    pub callSignatures: Option<Vec<Signature>>, // Call signatures of type
    /** @internal */
    pub constructSignatures: Option<Vec<Signature>>, // Construct signatures of type
    /** @internal */
    pub indexInfos: Option<Vec<IndexInfo>>, // Index signatures
    /** @internal */
    pub objectTypeWithoutAbstractConstructSignatures: Option<Box<dyn ObjectType>>,
}

pub trait ObjectType: ObjectFlagsTrait {
    fn get_object_props(&self) -> &ObjectTypeProps;
}

#[derive(Debug)]
pub struct InterfaceTypeProps {
    pub typeParameters: Option<Vec<TypeParameter>>,      // Type parameters (undefined if non-generic)
    pub outerTypeParameters: Option<Vec<TypeParameter>>, // Outer type parameters (undefined if none)
    pub localTypeParameters: Option<Vec<TypeParameter>>, // Local type parameters (undefined if none)
    pub thisType: Option<TypeParameter>,                 // The "this" type (undefined if none)
    /** @internal */
    pub resolvedBaseConstructorType: Option<Box<dyn Type>>, // Resolved base constructor type of class
    /** @internal */
    pub resolvedBaseTypes: Vec<BaseType>, // Resolved base types
    /** @internal */
    pub baseTypesResolved: Option<bool>,
}

pub trait InterfaceType: ObjectType {
    fn get_interface_props(&self) -> &InterfaceTypeProps;
}
// endregion: 6537

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

pub enum TypeMapper {
    Simple { source: Box<dyn Type>, target: Box<dyn Type> },
    Array { sources: Vec<Box<dyn Type>>, targets: Option<Vec<Box<dyn Type>>> },
    Deferred { sources: Vec<Box<dyn Type>>, targets: Vec<Box<dyn Fn() -> Box<dyn Type>>> },
    Function { func: Box<dyn Fn(Box<dyn Type>) -> Box<dyn Type>>, debug_info: Option<Box<dyn Fn() -> String>> },
    Composite { mapper1: Box<TypeMapper>, mapper2: Box<TypeMapper> },
    Merged { mapper1: Box<TypeMapper>, mapper2: Box<TypeMapper> },
}

impl std::fmt::Debug for TypeMapper {
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

// region: 7136
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
// endregion: 7145

// region: 7231
#[derive(Debug)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug)]
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
    // pub importsNotUsedAsValues: Option<ImportsNotUsedAsValues>,
    /** @internal */
    pub init: Option<bool>,
    pub inlineSourceMap: Option<bool>,
    pub inlineSources: Option<bool>,
    pub isolatedModules: Option<bool>,
    pub isolatedDeclarations: Option<bool>,
    // pub jsx: Option<JsxEmit>,
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
    // pub module: Option<ModuleKind>,
    // pub moduleResolution: Option<ModuleResolutionKind>,
    // pub moduleSuffixes: Option<Vec<String>>,
    // pub moduleDetection: Option<ModuleDetectionKind>,
    // pub newLine: Option<NewLineKind>,
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
    // pub paths: Option<MapLike<Vec<String>>>,
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
    // pub target: Option<ScriptTarget>,
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

// region: 9840
/** @internal */
pub trait ModuleSpecifierResolutionHost {
    fn useCaseSensitiveFileNames(&self) -> Option<bool>;
    fn fileExists(&self, path: &str) -> bool;
    fn getCurrentDirectory(&self) -> String;
    fn directoryExists(&self, path: &str) -> Option<bool>;
    fn readFile(&self, path: &str) -> Option<String>;
    fn realpath(&self, path: &str) -> Option<String>;
    // fn getSymlinkCache(&self) -> Option<SymlinkCache>;
    // fn getModuleSpecifierCache(&self) -> Option<ModuleSpecifierCache>;
    // fn getPackageJsonInfoCache(&self) -> Option<PackageJsonInfoCache>;
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
    fn trace(&self, s: &str) -> Option<()>;
}
// endregion: 9864
