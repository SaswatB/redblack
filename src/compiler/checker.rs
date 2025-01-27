use crate::compiler::parser::*;
use crate::compiler::types::*;
use oxc::ast::{
    ast::{Argument, Expression, ObjectExpression},
    AstKind,
};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct TypeChecker<'a> {
    host: Arc<dyn TypeCheckerHost>,

    typeCount: usize,
    seenIntrinsicNames: HashSet<String>,
    anyType: Arc<dyn IntrinsicType + 'a>,
    autoType: Arc<dyn IntrinsicType + 'a>,
    wildcardType: Arc<dyn IntrinsicType + 'a>,
    blockedStringType: Arc<dyn IntrinsicType + 'a>,
    errorType: Arc<dyn IntrinsicType + 'a>,
    unresolvedType: Arc<dyn IntrinsicType + 'a>,
    nonInferrableAnyType: Arc<dyn IntrinsicType + 'a>,
    intrinsicMarkerType: Arc<dyn IntrinsicType + 'a>,
    unknownType: Arc<dyn IntrinsicType + 'a>,
    undefinedType: Arc<dyn IntrinsicType + 'a>,
    undefinedWideningType: Arc<dyn IntrinsicType + 'a>,
    missingType: Arc<dyn IntrinsicType + 'a>,
    undefinedOrMissingType: Arc<dyn IntrinsicType + 'a>,
    optionalType: Arc<dyn IntrinsicType + 'a>,
    nullType: Arc<dyn IntrinsicType + 'a>,
    nullWideningType: Arc<dyn IntrinsicType + 'a>,
    stringType: Arc<dyn IntrinsicType + 'a>,
    numberType: Arc<dyn IntrinsicType + 'a>,
    bigintType: Arc<dyn IntrinsicType + 'a>,
    falseType: Arc<dyn FreshableIntrinsicType + 'a>,
    regularFalseType: Arc<dyn FreshableIntrinsicType + 'a>,
    trueType: Arc<dyn FreshableIntrinsicType + 'a>,
    regularTrueType: Arc<dyn FreshableIntrinsicType + 'a>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(host: Arc<dyn TypeCheckerHost>) -> Arc<RefCell<Self>> {
        let checker = Arc::new(RefCell::new(Self {
            host,
            typeCount: 0,
            seenIntrinsicNames: HashSet::new(),

            // Initialize with empty types that will be properly set in init_intrinsic_types
            anyType: Arc::new(TypeObject::new(TypeFlags::Any)),
            autoType: Arc::new(TypeObject::new(TypeFlags::Any)),
            wildcardType: Arc::new(TypeObject::new(TypeFlags::Any)),
            blockedStringType: Arc::new(TypeObject::new(TypeFlags::Any)),
            errorType: Arc::new(TypeObject::new(TypeFlags::Any)),
            unresolvedType: Arc::new(TypeObject::new(TypeFlags::Any)),
            nonInferrableAnyType: Arc::new(TypeObject::new(TypeFlags::Any)),
            intrinsicMarkerType: Arc::new(TypeObject::new(TypeFlags::Any)),
            unknownType: Arc::new(TypeObject::new(TypeFlags::Any)),
            undefinedType: Arc::new(TypeObject::new(TypeFlags::Any)),
            undefinedWideningType: Arc::new(TypeObject::new(TypeFlags::Any)),
            missingType: Arc::new(TypeObject::new(TypeFlags::Any)),
            undefinedOrMissingType: Arc::new(TypeObject::new(TypeFlags::Any)),
            optionalType: Arc::new(TypeObject::new(TypeFlags::Any)),
            nullType: Arc::new(TypeObject::new(TypeFlags::Any)),
            nullWideningType: Arc::new(TypeObject::new(TypeFlags::Any)),
            stringType: Arc::new(TypeObject::new(TypeFlags::Any)),
            numberType: Arc::new(TypeObject::new(TypeFlags::Any)),
            bigintType: Arc::new(TypeObject::new(TypeFlags::Any)),
            falseType: Arc::new(TypeObject::new(TypeFlags::Any)),
            regularFalseType: Arc::new(TypeObject::new(TypeFlags::Any)),
            trueType: Arc::new(TypeObject::new(TypeFlags::Any)),
            regularTrueType: Arc::new(TypeObject::new(TypeFlags::Any)),
        }));

        checker.borrow_mut().init_intrinsic_types();
        checker
    }

    fn init_intrinsic_types(&mut self) {
        let exactOptionalPropertyTypes = self.host.getCompilerOptions().exactOptionalPropertyTypes.unwrap_or(false);
        let strictNullChecks = self.host.getCompilerOptions().strictNullChecks.unwrap_or(false);

        // region: 2046
        self.anyType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "any", ObjectFlags::None, None));
        self.autoType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "any", ObjectFlags::NonInferrableType, Some("auto")));
        self.wildcardType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "any", ObjectFlags::None, Some("wildcard")));
        self.blockedStringType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "any", ObjectFlags::None, Some("blocked string")));
        self.errorType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "error", ObjectFlags::None, None));
        self.unresolvedType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "unresolved", ObjectFlags::None, None));
        self.nonInferrableAnyType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "any", ObjectFlags::ContainsWideningType, Some("non-inferrable")));
        self.intrinsicMarkerType = Arc::new(self.createIntrinsicType(TypeFlags::Any, "intrinsic", ObjectFlags::None, None));
        self.unknownType = Arc::new(self.createIntrinsicType(TypeFlags::Unknown, "unknown", ObjectFlags::None, None));
        self.undefinedType = Arc::new(self.createIntrinsicType(TypeFlags::Undefined, "undefined", ObjectFlags::None, None));
        self.undefinedWideningType = Arc::new(self.createIntrinsicType(TypeFlags::Undefined, "undefined", ObjectFlags::ContainsWideningType, Some("widening")));
        self.missingType = Arc::new(self.createIntrinsicType(TypeFlags::Undefined, "undefined", ObjectFlags::None, Some("missing")));
        self.undefinedOrMissingType = if exactOptionalPropertyTypes { Arc::clone(&self.missingType) } else { Arc::clone(&self.undefinedType) };
        self.optionalType = Arc::new(self.createIntrinsicType(TypeFlags::Undefined, "undefined", ObjectFlags::None, Some("optional")));
        self.nullType = Arc::new(self.createIntrinsicType(TypeFlags::Null, "null", ObjectFlags::None, None));
        self.nullWideningType = if strictNullChecks { Arc::clone(&self.nullType) } else { Arc::new(self.createIntrinsicType(TypeFlags::Null, "null", ObjectFlags::ContainsWideningType, Some("widening"))) };
        self.stringType = Arc::new(self.createIntrinsicType(TypeFlags::String, "string", ObjectFlags::None, None));
        self.numberType = Arc::new(self.createIntrinsicType(TypeFlags::Number, "number", ObjectFlags::None, None));
        self.bigintType = Arc::new(self.createIntrinsicType(TypeFlags::BigInt, "bigint", ObjectFlags::None, None));
        let mut false_type = self.createIntrinsicType(TypeFlags::BooleanLiteral, "false", ObjectFlags::None, Some("fresh"));
        let mut regular_false_type = self.createIntrinsicType(TypeFlags::BooleanLiteral, "false", ObjectFlags::None, None);
        let false_type_freshable_props = FreshableTypeProps { freshType: unsafe { &*(&false_type as *const _ as *const dyn FreshableType) }, regularType: unsafe { &*(&regular_false_type as *const _ as *const dyn FreshableType) } };
        false_type.freshable_props = Some(false_type_freshable_props.clone());
        regular_false_type.freshable_props = Some(false_type_freshable_props.clone());
        self.falseType = Arc::new(false_type);
        self.regularFalseType = Arc::new(regular_false_type);
        let mut true_type = self.createIntrinsicType(TypeFlags::BooleanLiteral, "true", ObjectFlags::None, Some("fresh"));
        let mut regular_true_type = self.createIntrinsicType(TypeFlags::BooleanLiteral, "true", ObjectFlags::None, None);
        let true_type_freshable_props = FreshableTypeProps { freshType: unsafe { &*(&true_type as *const _ as *const dyn FreshableType) }, regularType: unsafe { &*(&regular_true_type as *const _ as *const dyn FreshableType) } };
        true_type.freshable_props = Some(true_type_freshable_props.clone());
        regular_true_type.freshable_props = Some(true_type_freshable_props.clone());
        self.trueType = Arc::new(true_type);
        self.regularTrueType = Arc::new(regular_true_type);
        // endregion: 2068
    }

    // region: 5337
    fn createType(&mut self, flags: TypeFlags) -> TypeObject<'a> {
        let mut result: TypeObject<'a> = TypeObject::new(flags);
        self.typeCount += 1;
        result.id = self.typeCount;
        // tracing?.recordType(result);
        result
    }

    fn createTypeWithSymbol(&mut self, flags: TypeFlags, symbol: Symbol) -> TypeObject<'a> {
        let mut result = self.createType(flags);
        result.symbol = symbol;
        result
    }

    fn createOriginType(&self, flags: TypeFlags) -> TypeObject<'a> { TypeObject::new(flags) }

    fn createIntrinsicType(&mut self, kind: TypeFlags, intrinsicName: &str, objectFlags: ObjectFlags, debugIntrinsicName: Option<&str>) -> TypeObject<'a> {
        self.checkIntrinsicName(intrinsicName, debugIntrinsicName);
        let mut result = self.createType(kind);
        result.intrinsic_props = Some(IntrinsicTypeProps { intrinsicName: intrinsicName.to_owned(), debugIntrinsicName: debugIntrinsicName.map(|s| s.to_owned()) });
        result.object_flags = Some(objectFlags | ObjectFlags::CouldContainTypeVariablesComputed | ObjectFlags::IsGenericTypeComputed | ObjectFlags::IsUnknownLikeUnionComputed | ObjectFlags::IsNeverIntersectionComputed);
        return result;
    }

    fn checkIntrinsicName(&mut self, name: &str, debug: Option<&str>) {
        let key = format!("{name},{}", debug.unwrap_or(""));
        if self.seenIntrinsicNames.contains(&key) {
            // Debug.fail(`Duplicate intrinsic type name ${name}${debug ? ` (${debug})` : ""}; you may need to pass a name to createIntrinsicType.`);
        }
        self.seenIntrinsicNames.insert(key);
    }

    fn createObjectType(&mut self, objectFlags: ObjectFlags, symbol: Option<Symbol>) -> TypeObject<'a> {
        let mut result = self.createTypeWithSymbol(TypeFlags::Object, symbol.unwrap());
        result.object_flags = Some(objectFlags);
        result.object_props = Some(ObjectTypeProps { members: None, properties: None, callSignatures: None, constructSignatures: None, indexInfos: None, objectTypeWithoutAbstractConstructSignatures: None });
        result
    }
    // endregion: 5381

    // region: 49169
    fn getTypeOfNode(&self, node: &AstKind) -> &dyn Type {
        if let Some(program) = node.as_program() {
            if !isExternalModule(program) {
                return self.errorType.as_type();
            }
        }

        // if (node.flags & NodeFlags.InWithStatement) {
        //     // We cannot answer semantic questions within a with block, do not proceed any further
        //     return errorType;
        // }

        // const classDecl = tryGetClassImplementingOrExtendingExpressionWithTypeArguments(node);
        // const classType = classDecl && getDeclaredTypeOfClassOrInterface(getSymbolOfDeclaration(classDecl.class));
        // if (isPartOfTypeNode(node)) {
        //     const typeFromTypeNode = getTypeFromTypeNode(node as TypeNode);
        //     return classType ? getTypeWithThisArgument(typeFromTypeNode, classType.thisType) : typeFromTypeNode;
        // }

        // if let Some(expression) = as_expression(node) {
        //     return getRegularTypeOfExpression(expression);
        // }

        // if (classType && !classDecl.isImplements) {
        //     // A SyntaxKind.ExpressionWithTypeArguments is considered a type node, except when it occurs in the
        //     // extends clause of a class. We handle that case here.
        //     const baseType = firstOrUndefined(getBaseTypes(classType));
        //     return baseType ? getTypeWithThisArgument(baseType, classType.thisType) : errorType;
        // }

        // if (isTypeDeclaration(node)) {
        //     // In this case, we call getSymbolOfNode instead of getSymbolAtLocation because it is a declaration
        //     const symbol = getSymbolOfDeclaration(node);
        //     return getDeclaredTypeOfSymbol(symbol);
        // }

        // if (isTypeDeclarationName(node)) {
        //     const symbol = getSymbolAtLocation(node);
        //     return symbol ? getDeclaredTypeOfSymbol(symbol) : errorType;
        // }

        // if let Some(binding_identifier) = node.as_binding_identifier() {
        //     return getTypeForVariableLikeDeclaration(node, /*includeOptionality*/ true, CheckMode.Normal) || self.errorType.as_type();
        // }

        // if (isDeclaration(node)) {
        //     // In this case, we call getSymbolOfNode instead of getSymbolAtLocation because it is a declaration
        //     const symbol = getSymbolOfDeclaration(node);
        //     return symbol ? getTypeOfSymbol(symbol) : errorType;
        // }

        // if (isDeclarationNameOrImportPropertyName(node)) {
        //     const symbol = getSymbolAtLocation(node);
        //     if (symbol) {
        //         return getTypeOfSymbol(symbol);
        //     }
        //     return errorType;
        // }

        // if (isBindingPattern(node)) {
        //     return getTypeForVariableLikeDeclaration(node.parent, /*includeOptionality*/ true, CheckMode.Normal) || errorType;
        // }

        // if (isInRightSideOfImportOrExportAssignment(node as Identifier)) {
        //     const symbol = getSymbolAtLocation(node);
        //     if (symbol) {
        //         const declaredType = getDeclaredTypeOfSymbol(symbol);
        //         return !isErrorType(declaredType) ? declaredType : getTypeOfSymbol(symbol);
        //     }
        // }

        // if (isMetaProperty(node.parent) && node.parent.keywordToken === node.kind) {
        //     return checkMetaPropertyKeyword(node.parent);
        // }

        // if (isImportAttributes(node)) {
        //     return getGlobalImportAttributesType(/*reportErrors*/ false);
        // }

        return self.errorType.as_type();
    }
    // endregion: 49247
}

#[allow(unused_variables)]
impl<'a> TypeCheckerTrait for TypeChecker<'a> {
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
    fn getTypeAtLocation(&self, node: AstKind) -> &dyn Type { self.getTypeOfNode(&node) }
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
