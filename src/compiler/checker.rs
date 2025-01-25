use crate::compiler::types::*;
use oxc::ast::{
    ast::{Argument, Expression, ObjectExpression, Program},
    AstKind,
};

#[derive(Debug)]
pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self { Self {} }

    // fn is_source_file<'a>(&self, node: AstKind<'a>) -> Option<&'a Program<'a>> {
    //     if let AstKind::Program(program) = node {
    //         return Some(program);
    //     }
    //     None
    // }

    // // fn is_external_module(&self, node: Program) -> bool { node. }

    // fn error_type(&self) -> Type {
    //     TypeObject {
    //         flags: TypeFlags::Any,
    //         id: TypeId(0),
    //         checker: Box::new(Self::new()),
    //         symbol: Symbol::new(SymbolFlags::NONE, "error"),
    //         pattern: None,
    //         alias_symbol: None,
    //         alias_type_arguments: None,
    //         permissive_instantiation: None,
    //         restrictive_instantiation: None,
    //         immediate_base_constraint: None,
    //         widened: None,
    //     }
    // }

    // fn get_type_of_node(&self, node: AstKind) -> Type {
    //     if let Some(program) = self.is_source_file(node) {
    //         if !self.is_external_module(program) {
    //             return self.error_type();
    //         }
    //     }

    //     // if node.flags & NodeFlags::IN_WITH_STATEMENT != 0 {
    //     //     // We cannot answer semantic questions within a with block, do not proceed any further
    //     //     return self.error_type();
    //     // }

    //     // let class_decl = self.try_get_class_implementing_or_extending_expression_with_type_arguments(node);
    //     // let class_type = class_decl.and_then(|decl| {
    //     //     let symbol = self.get_symbol_of_declaration(decl.class);
    //     //     self.get_declared_type_of_class_or_interface(symbol)
    //     // });

    //     // if self.is_part_of_type_node(node) {
    //     //     let type_from_type_node = self.get_type_from_type_node(node);
    //     //     return if let Some(class_type) = class_type { self.get_type_with_this_argument(type_from_type_node, class_type.this_type) } else { type_from_type_node };
    //     // }

    //     // if self.is_expression_node(node) {
    //     //     return self.get_regular_type_of_expression(node);
    //     // }

    //     // if let Some(class_type) = class_type {
    //     //     if !class_decl.unwrap().is_implements {
    //     //         // A SyntaxKind.ExpressionWithTypeArguments is considered a type node, except when it occurs in the
    //     //         // extends clause of a class. We handle that case here.
    //     //         let base_type = self.get_base_types(class_type).into_iter().next();
    //     //         return if let Some(base_type) = base_type { self.get_type_with_this_argument(base_type, class_type.this_type) } else { self.error_type() };
    //     //     }
    //     // }

    //     // if self.is_type_declaration(node) {
    //     //     // In this case, we call get_symbol_of_node instead of get_symbol_at_location because it is a declaration
    //     //     let symbol = self.get_symbol_of_declaration(node);
    //     //     return self.get_declared_type_of_symbol(symbol);
    //     // }

    //     // if self.is_type_declaration_name(node) {
    //     //     let symbol = self.get_symbol_at_location(node);
    //     //     return if let Some(symbol) = symbol { self.get_declared_type_of_symbol(symbol) } else { self.error_type() };
    //     // }

    //     // if self.is_binding_element(node) {
    //     //     return self.get_type_for_variable_like_declaration(node, /*includeOptionality*/ true, CheckMode::Normal).unwrap_or_else(|| self.error_type());
    //     // }

    //     // if self.is_declaration(node) {
    //     //     // In this case, we call get_symbol_of_node instead of get_symbol_at_location because it is a declaration
    //     //     let symbol = self.get_symbol_of_declaration(node);
    //     //     return if let Some(symbol) = symbol { self.get_type_of_symbol(symbol) } else { self.error_type() };
    //     // }

    //     // if self.is_declaration_name_or_import_property_name(node) {
    //     //     let symbol = self.get_symbol_at_location(node);
    //     //     if let Some(symbol) = symbol {
    //     //         return self.get_type_of_symbol(symbol);
    //     //     }
    //     //     return self.error_type();
    //     // }

    //     // if self.is_binding_pattern(node) {
    //     //     return self.get_type_for_variable_like_declaration(node.parent, /*includeOptionality*/ true, CheckMode::Normal).unwrap_or_else(|| self.error_type());
    //     // }

    //     // if self.is_in_right_side_of_import_or_export_assignment(node) {
    //     //     let symbol = self.get_symbol_at_location(node);
    //     //     if let Some(symbol) = symbol {
    //     //         let declared_type = self.get_declared_type_of_symbol(symbol);
    //     //         return if !self.is_error_type(declared_type) { declared_type } else { self.get_type_of_symbol(symbol) };
    //     //     }
    //     // }

    //     // if let Some(parent) = self.get_parent(node) {
    //     //     if self.is_meta_property(parent) && parent.keyword_token == node.kind {
    //     //         return self.check_meta_property_keyword(parent);
    //     //     }
    //     // }

    //     // if self.is_import_attributes(node) {
    //     //     return self.get_global_import_attributes_type(/*reportErrors*/ false);
    //     // }

    //     return self.error_type();
    // }
}

impl crate::compiler::types::TypeChecker for TypeChecker {
    fn getTypeOfSymbolAtLocation(&self, symbol: &Symbol, node: &AstKind) -> &dyn Type { todo!() }
    fn getTypeOfSymbol(&self, symbol: &Symbol) -> &dyn Type { todo!() }
    fn getDeclaredTypeOfSymbol(&self, symbol: &Symbol) -> &dyn Type { todo!() }
    fn getPropertiesOfType(&self, type_: &dyn Type) -> Vec<&Symbol> { todo!() }
    fn getPropertyOfType(&self, type_: &dyn Type, property_name: &str) -> Option<&Symbol> { todo!() }
    fn getPrivateIdentifierPropertyOfType(&self, left_type: &dyn Type, name: &str, location: &AstKind) -> Option<&Symbol> { todo!() }
    fn getTypeOfPropertyOfType(&self, type_: &dyn Type, propertyName: &str) -> Option<&dyn Type> { todo!() }
    fn getIndexInfoOfType(&self, type_: &dyn Type, kind: IndexKind) -> Option<IndexInfo> { todo!() }
    fn getIndexInfosOfType(&self, type_: &dyn Type) -> Vec<IndexInfo> { todo!() }
    fn getIndexInfosOfIndexSymbol(&self, indexSymbol: Symbol) -> Vec<IndexInfo> { todo!() }
    fn getSignaturesOfType(&self, type_: &dyn Type, kind: SignatureKind) -> Vec<&Signature> { todo!() }
    fn getIndexTypeOfType(&self, type_: &dyn Type, kind: IndexKind) -> Option<&dyn Type> { todo!() }
    fn getIndexType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getBaseTypes(&self, type_: &dyn InterfaceType) -> Vec<BaseType> { todo!() }
    fn getBaseTypeOfLiteralType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getWidenedType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getWidenedLiteralType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getPromisedTypeOfPromise(&self, promise: &dyn Type, errorNode: Option<AstKind>) -> Option<&dyn Type> { todo!() }
    fn getAwaitedType(&self, type_: &dyn Type) -> Option<&dyn Type> { todo!() }
    fn isEmptyAnonymousObjectType(&self, type_: &dyn Type) -> bool { todo!() }
    fn getReturnTypeOfSignature(&self, signature: Signature) -> &dyn Type { todo!() }
    fn getParameterType(&self, signature: Signature, parameter_index: usize) -> &dyn Type { todo!() }
    fn getParameterIdentifierInfoAtPosition(&self, signature: Signature, parameter_index: usize) -> Option<(Identifier, &str, bool)> { todo!() }
    fn getNullableType(&self, type_: &dyn Type, flags: TypeFlags) -> &dyn Type { todo!() }
    fn getNonNullableType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getNonOptionalType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn isNullableType(&self, type_: &dyn Type) -> bool { todo!() }
    fn getTypeArguments(&self, type_: TypeReference) -> Vec<&dyn Type> { todo!() }
    fn getSymbolsInScope(&self, location: AstKind, meaning: SymbolFlags) -> Vec<Symbol> { todo!() }
    fn getSymbolAtLocation(&self, node: AstKind) -> Option<Symbol> { todo!() }
    fn getIndexInfosAtLocation(&self, node: AstKind) -> Option<Vec<IndexInfo>> { todo!() }
    fn getSymbolsOfParameterPropertyDeclaration(&self, parameter: Argument, parameter_name: &str) -> Vec<Symbol> { todo!() }
    fn getShorthandAssignmentValueSymbol(&self, location: Option<AstKind>) -> Option<Symbol> { todo!() }
    fn getExportSpecifierLocalTargetSymbol(&self, location: ExportSpecifier) -> Option<Symbol> { todo!() }
    fn getExportSymbolOfSymbol(&self, symbol: Symbol) -> Symbol { todo!() }
    fn getPropertySymbolOfDestructuringAssignment(&self, location: Identifier) -> Option<Symbol> { todo!() }
    fn getTypeOfAssignmentPattern(&self, pattern: AssignmentPattern) -> &dyn Type { todo!() }
    fn getTypeAtLocation(&self, node: AstKind) -> &dyn Type { todo!() }
    fn getTypeFromTypeNode(&self, node: TypeNode) -> &dyn Type { todo!() }
    fn signatureToString(&self, signature: Signature, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>) -> String { todo!() }
    fn typeToString(&self, type_: &dyn Type, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>) -> String { todo!() }
    fn symbolToString(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>) -> String { todo!() }
    fn typePredicateToString(&self, predicate: TypePredicate, enclosing_declaration: Option<AstKind>, flags: Option<TypeFormatFlags>) -> String { todo!() }
    fn writeSignature(&self, signature: Signature, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>, writer: Option<EmitTextWriter>) -> String { todo!() }
    fn writeType(&self, type_: &dyn Type, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String { todo!() }
    fn writeSymbol(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>, writer: Option<EmitTextWriter>) -> String { todo!() }
    fn writeTypePredicate(&self, predicate: TypePredicate, enclosingDeclaration: Option<AstKind>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String { todo!() }
    fn getFullyQualifiedName(&self, symbol: Symbol) -> String { todo!() }
    fn getAugmentedPropertiesOfType(&self, type_: &dyn Type) -> Vec<&Symbol> { todo!() }
    fn getRootSymbols(&self, symbol: Symbol) -> Vec<Symbol> { todo!() }
    fn getSymbolOfExpando(&self, node: AstKind, allowDeclaration: bool) -> Option<Symbol> { todo!() }
    fn getContextualType(&self, node: Expression) -> Option<&dyn Type> { todo!() }
    fn getContextualTypeWithFlags(&self, node: Expression, contextFlags: Option<ContextFlags>) -> Option<&dyn Type> { todo!() }
    fn getContextualTypeForObjectLiteralElement(&self, element: ObjectLiteralElementLike) -> Option<&dyn Type> { todo!() }
    fn getContextualTypeForArgumentAtIndex(&self, call: CallLikeExpression, argIndex: usize) -> Option<&dyn Type> { todo!() }
    fn getContextualTypeForJsxAttribute(&self, attribute: JsxAttribute) -> Option<&dyn Type> { todo!() }
    fn isContextSensitive(&self, node: Expression) -> bool { todo!() }
    fn getTypeOfPropertyOfContextualType(&self, type_: &dyn Type, name: &str) -> Option<&dyn Type> { todo!() }
    fn getResolvedSignature(&self, node: CallLikeExpression, candidatesOutArray: Option<Vec<Signature>>, argumentCount: Option<usize>) -> Option<Signature> { todo!() }
    fn getResolvedSignatureForSignatureHelp(&self, node: CallLikeExpression, candidatesOutArray: Option<Vec<Signature>>, argumentCount: Option<usize>) -> Option<Signature> { todo!() }
    fn getCandidateSignaturesForStringLiteralCompletions(&self, call: CallLikeExpression, editingArgument: AstKind) -> Vec<Signature> { todo!() }
    fn getExpandedParameters(&self, sig: Signature) -> Vec<Vec<Symbol>> { todo!() }
    fn hasEffectiveRestParameter(&self, sig: Signature) -> bool { todo!() }
    fn containsArgumentsReference(&self, declaration: SignatureDeclaration) -> bool { todo!() }
    fn getSignatureFromDeclaration(&self, declaration: SignatureDeclaration) -> Option<Signature> { todo!() }
    fn isImplementationOfOverload(&self, node: SignatureDeclaration) -> Option<bool> { todo!() }
    fn isUndefinedSymbol(&self, symbol: Symbol) -> bool { todo!() }
    fn isArgumentsSymbol(&self, symbol: Symbol) -> bool { todo!() }
    fn isUnknownSymbol(&self, symbol: Symbol) -> bool { todo!() }
    fn getMergedSymbol(&self, symbol: Symbol) -> Symbol { todo!() }
    fn symbolIsValue(&self, symbol: Symbol, includeTypeOnlyMembers: Option<bool>) -> bool { todo!() }
    fn getConstantValue(&self, node: EnumMember) -> Option<String> { todo!() }
    fn isValidPropertyAccess(&self, node: PropertyAccessExpression, propertyName: &str) -> bool { todo!() }
    fn isValidPropertyAccessForCompletions(&self, node: PropertyAccessExpression, type_: &dyn Type, property: Symbol) -> bool { todo!() }
    fn getAliasedSymbol(&self, symbol: Symbol) -> Symbol { todo!() }
    fn getImmediateAliasedSymbol(&self, symbol: Symbol) -> Option<Symbol> { todo!() }
    fn getExportsOfModule(&self, moduleSymbol: Symbol) -> Vec<Symbol> { todo!() }
    fn getExportsAndPropertiesOfModule(&self, moduleSymbol: Symbol) -> Vec<Symbol> { todo!() }
    fn getJsxIntrinsicTagNamesAt(&self, location: AstKind) -> Vec<Symbol> { todo!() }
    fn isOptionalParameter(&self, node: Argument) -> bool { todo!() }
    fn getAmbientModules(&self) -> Vec<Symbol> { todo!() }
    fn tryGetMemberInModuleExports(&self, memberName: &str, moduleSymbol: Symbol) -> Option<Symbol> { todo!() }
    fn tryGetMemberInModuleExportsAndProperties(&self, memberName: &str, moduleSymbol: Symbol) -> Option<Symbol> { todo!() }
    fn getApparentType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getSuggestedSymbolForNonexistentProperty(&self, name: MemberName, containingType: &dyn Type) -> Option<Symbol> { todo!() }
    fn getSuggestedSymbolForNonexistentJsxAttribute(&self, name: Identifier, containingType: &dyn Type) -> Option<Symbol> { todo!() }
    fn getSuggestedSymbolForNonexistentSymbol(&self, location: AstKind, name: &str, meaning: SymbolFlags) -> Option<Symbol> { todo!() }
    fn getSuggestedSymbolForNonexistentModule(&self, node: Identifier, target: Symbol) -> Option<Symbol> { todo!() }
    fn getSuggestedSymbolForNonexistentClassMember(&self, name: &str, baseType: &dyn Type) -> Option<Symbol> { todo!() }
    fn getBaseConstraintOfType(&self, type_: &dyn Type) -> Option<&dyn Type> { todo!() }
    fn getDefaultFromTypeParameter(&self, type_: &dyn Type) -> Option<&dyn Type> { todo!() }
    fn getAnyType(&self) -> &dyn Type { todo!() }
    fn getStringType(&self) -> &dyn Type { todo!() }
    fn getStringLiteralType(&self, value: &str) -> StringLiteralType { todo!() }
    fn getNumberType(&self) -> &dyn Type { todo!() }
    fn getNumberLiteralType(&self, value: f64) -> NumberLiteralType { todo!() }
    fn getBigIntType(&self) -> &dyn Type { todo!() }
    fn getBigIntLiteralType(&self, value: PseudoBigInt) -> BigIntLiteralType { todo!() }
    fn getBooleanType(&self) -> &dyn Type { todo!() }
    fn getFalseType(&self, fresh: Option<bool>) -> &dyn Type { todo!() }
    fn getTrueType(&self, fresh: Option<bool>) -> &dyn Type { todo!() }
    fn getVoidType(&self) -> &dyn Type { todo!() }
    fn getUndefinedType(&self) -> &dyn Type { todo!() }
    fn getNullType(&self) -> &dyn Type { todo!() }
    fn getESSymbolType(&self) -> &dyn Type { todo!() }
    fn getNeverType(&self) -> &dyn Type { todo!() }
    fn getOptionalType(&self) -> &dyn Type { todo!() }
    fn getUnionType(&self, types: Vec<&dyn Type>, subtypeReduction: Option<UnionReduction>) -> &dyn Type { todo!() }
    fn createArrayType(&self, elementType: &dyn Type) -> &dyn Type { todo!() }
    fn getElementTypeOfArrayType(&self, arrayType: &dyn Type) -> Option<&dyn Type> { todo!() }
    fn createPromiseType(&self, type_: &dyn Type) -> &dyn Type { todo!() }
    fn getPromiseType(&self) -> &dyn Type { todo!() }
    fn getPromiseLikeType(&self) -> &dyn Type { todo!() }
    fn getAnyAsyncIterableType(&self) -> Option<&dyn Type> { todo!() }
    fn isTypeAssignableTo(&self, source: &dyn Type, target: &dyn Type) -> bool { todo!() }
    fn createAnonymousType(&self, symbol: Option<Symbol>, members: SymbolTable, callSignatures: Vec<Signature>, constructSignatures: Vec<Signature>, indexInfos: Vec<IndexInfo>) -> &dyn Type { todo!() }
    fn createSignature(
        &self, declaration: Option<SignatureDeclaration>, typeParameters: Option<Vec<TypeParameter>>, thisParameter: Option<Symbol>, parameters: Vec<Symbol>, resolvedReturnType: &dyn Type, typePredicate: Option<TypePredicate>, minArgumentCount: usize, flags: SignatureFlags,
    ) -> Signature {
        todo!()
    }
    fn createSymbol(&self, flags: SymbolFlags, name: &str) -> TransientSymbol { todo!() }
    fn createIndexInfo(&self, keyType: &dyn Type, type_: &dyn Type, isReadonly: bool, declaration: Option<SignatureDeclaration>) -> IndexInfo { todo!() }
    fn isSymbolAccessible(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: SymbolFlags, shouldComputeAliasToMarkVisible: bool) -> SymbolAccessibilityResult { todo!() }
    fn tryFindAmbientModule(&self, moduleName: &str) -> Option<Symbol> { todo!() }
    fn getSymbolWalker(&self, accept: Option<fn(Symbol) -> bool>) -> SymbolWalker { todo!() }
    fn getDiagnostics(&self, sourceFile: Option<SourceFile>, cancellationToken: Option<CancellationToken>, nodesToCheck: Option<Vec<AstKind>>) -> Vec<Diagnostic> { todo!() }
    fn getGlobalDiagnostics(&self) -> Vec<Diagnostic> { todo!() }
    fn getEmitResolver(&self, sourceFile: Option<SourceFile>, cancellationToken: Option<CancellationToken>, forceDts: Option<bool>) -> EmitResolver { todo!() }
    fn requiresAddingImplicitUndefined(&self, parameter: Argument, enclosingDeclaration: Option<AstKind>) -> bool { todo!() }
    fn getNodeCount(&self) -> usize { todo!() }
    fn getIdentifierCount(&self) -> usize { todo!() }
    fn getSymbolCount(&self) -> usize { todo!() }
    fn getTypeCount(&self) -> usize { todo!() }
    fn getInstantiationCount(&self) -> usize { todo!() }
    fn getRelationCacheSizes(&self) -> (usize, usize, usize, usize) { todo!() }
    fn getRecursionIdentity(&self, type_: &dyn Type) -> Option<&dyn Type> { todo!() }
    fn getUnmatchedProperties(&self, source: &dyn Type, target: &dyn Type, requireOptionalProperties: bool, matchDiscriminantProperties: bool) -> Box<dyn Iterator<Item = Symbol>> { todo!() }
    fn isArrayType(&self, type_: &dyn Type) -> bool { todo!() }
    fn isTupleType(&self, type_: &dyn Type) -> bool { todo!() }
    fn isArrayLikeType(&self, type_: &dyn Type) -> bool { todo!() }
    fn isTypeInvalidDueToUnionDiscriminant(&self, contextualType: &dyn Type, obj: ObjectExpression) -> bool { todo!() }
    fn getExactOptionalProperties(&self, type_: &dyn Type) -> Vec<Symbol> { todo!() }
    fn getAllPossiblePropertiesOfTypes(&self, types: Vec<&dyn Type>) -> Vec<Symbol> { todo!() }
    fn resolveName(&self, name: &str, location: Option<AstKind>, meaning: SymbolFlags, excludeGlobals: bool) -> Option<Symbol> { todo!() }
    fn getJsxNamespace(&self, location: Option<AstKind>) -> String { todo!() }
    fn getJsxFragmentFactory(&self, location: AstKind) -> Option<String> { todo!() }
    fn getAccessibleSymbolChain(&self, symbol: Symbol, enclosingDeclaration: Option<AstKind>, meaning: SymbolFlags, useOnlyExternalAliasing: bool) -> Option<Vec<Symbol>> { todo!() }
    fn getTypePredicateOfSignature(&self, signature: Signature) -> Option<TypePredicate> { todo!() }
    fn resolveExternalModuleName(&self, moduleSpecifier: Expression) -> Option<Symbol> { todo!() }
    fn resolveExternalModuleSymbol(&self, symbol: Symbol) -> Symbol { todo!() }
    fn tryGetThisTypeAt(&self, node: AstKind, includeGlobalThis: Option<bool>, container: Option<ThisContainer>) -> Option<&dyn Type> { todo!() }
    fn getTypeArgumentConstraint(&self, node: TypeNode) -> Option<&dyn Type> { todo!() }
    fn getSuggestionDiagnostics(&self, file: SourceFile, cancellationToken: Option<CancellationToken>) -> Vec<DiagnosticWithLocation> { todo!() }
    fn getLocalTypeParametersOfClassOrInterfaceOrTypeAlias(&self, symbol: Symbol) -> Option<Vec<TypeParameter>> { todo!() }
    fn isDeclarationVisible(&self, node: Declaration) -> bool { todo!() }
    fn isPropertyAccessible(&self, node: AstKind, isSuper: bool, isWrite: bool, containingType: &dyn Type, property: Symbol) -> bool { todo!() }
    fn getTypeOnlyAliasDeclaration(&self, symbol: Symbol) -> Option<TypeOnlyAliasDeclaration> { todo!() }
    fn getMemberOverrideModifierStatus(&self, node: ClassLikeDeclaration, member: ClassElement, memberSymbol: Symbol) -> MemberOverrideStatus { todo!() }
    fn isTypeParameterPossiblyReferenced(&self, tp: TypeParameter, node: AstKind) -> bool { todo!() }
    fn typeHasCallOrConstructSignatures(&self, type_: &dyn Type) -> bool { todo!() }
    fn getSymbolFlags(&self, symbol: Symbol) -> SymbolFlags { todo!() }
}
