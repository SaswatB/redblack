#[derive(Debug)]
pub struct Symbol;

#[derive(Debug)]
pub struct Node;

#[derive(Debug)]
pub struct Type;

#[derive(Debug)]
pub struct IndexInfo;

#[derive(Debug)]
pub struct Signature;

#[derive(Debug)]
pub struct InterfaceType;

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
pub struct IndexSignatureDeclaration;

#[derive(Debug)]
pub struct EntityName;

#[derive(Debug)]
pub struct Expression;

#[derive(Debug)]
pub struct TypeParameterDeclaration;

#[derive(Debug)]
pub struct ParameterDeclaration;

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

#[derive(Debug)]
pub struct CallLikeExpression;

#[derive(Debug)]
pub struct MethodDeclaration;

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
pub struct ObjectLiteralExpression;

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

#[derive(Debug)]
pub struct Declaration;

#[derive(Debug)]
pub struct StringLiteralType;

#[derive(Debug)]
pub struct NumberLiteralType;

#[derive(Debug)]
pub struct BigIntLiteralType;

#[derive(Debug)]
pub struct PseudoBigInt;

#[derive(Debug)]
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
pub enum SignatureKind {}

#[derive(Debug)]
pub enum IndexKind {}

#[derive(Debug)]
pub enum NodeBuilderFlags {}

#[derive(Debug)]
pub enum InternalNodeBuilderFlags {}

#[derive(Debug)]
pub enum SyntaxKind {}

#[derive(Debug)]
pub enum SymbolFlags {}

#[derive(Debug)]
pub enum TypeFormatFlags {}

#[derive(Debug)]
pub enum SymbolFormatFlags {}

#[derive(Debug)]
pub enum ContextFlags {}

#[derive(Debug)]
pub enum UnionReduction {}

#[derive(Debug)]
pub enum SignatureFlags {}

#[derive(Debug)]
pub enum MemberOverrideStatus {}

#[derive(Debug)]
pub struct TypeReference;

#[derive(Debug)]
pub struct SymbolTracker;

pub trait TypeChecker {
    fn get_type_of_symbol_at_location(&self, symbol: Symbol, node: Node) -> Type;
    fn get_type_of_symbol(&self, symbol: Symbol) -> Type;
    fn get_declared_type_of_symbol(&self, symbol: Symbol) -> Type;
    fn get_properties_of_type(&self, type_: Type) -> Vec<Symbol>;
    fn get_property_of_type(&self, type_: Type, property_name: &str) -> Option<Symbol>;
    fn get_private_identifier_property_of_type(&self, left_type: Type, name: &str, location: Node) -> Option<Symbol>;
    /// @internal
    fn get_type_of_property_of_type(&self, type_: Type, property_name: &str) -> Option<Type>;
    fn get_index_info_of_type(&self, type_: Type, kind: IndexKind) -> Option<IndexInfo>;
    fn get_index_infos_of_type(&self, type_: Type) -> Vec<IndexInfo>;
    fn get_index_infos_of_index_symbol(&self, index_symbol: Symbol) -> Vec<IndexInfo>;
    fn get_signatures_of_type(&self, type_: Type, kind: SignatureKind) -> Vec<Signature>;
    fn get_index_type_of_type(&self, type_: Type, kind: IndexKind) -> Option<Type>;
    /// @internal
    fn get_index_type(&self, type_: Type) -> Type;
    fn get_base_types(&self, type_: InterfaceType) -> Vec<BaseType>;
    fn get_base_type_of_literal_type(&self, type_: Type) -> Type;
    fn get_widened_type(&self, type_: Type) -> Type;
    /// @internal
    fn get_widened_literal_type(&self, type_: Type) -> Type;
    /// @internal
    fn get_promised_type_of_promise(&self, promise: Type, error_node: Option<Node>) -> Option<Type>;
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
    fn get_awaited_type(&self, type_: Type) -> Option<Type>;
    /// @internal
    fn is_empty_anonymous_object_type(&self, type_: Type) -> bool;
    fn get_return_type_of_signature(&self, signature: Signature) -> Type;
    /// Gets the type of a parameter at a given position in a signature.
    /// Returns any if the index is not valid.
    ///
    /// @internal
    fn get_parameter_type(&self, signature: Signature, parameter_index: usize) -> Type;
    /// @internal
    fn get_parameter_identifier_info_at_position(&self, signature: Signature, parameter_index: usize) -> Option<(Identifier, &str, bool)>;
    fn get_nullable_type(&self, type_: Type, flags: TypeFlags) -> Type;
    fn get_non_nullable_type(&self, type_: Type) -> Type;
    /// @internal
    fn get_non_optional_type(&self, type_: Type) -> Type;
    /// @internal
    fn is_nullable_type(&self, type_: Type) -> bool;
    fn get_type_arguments(&self, type_: TypeReference) -> Vec<Type>;

    // TODO: GH#18217 `xToDeclaration` calls are frequently asserted as defined.
    /// Note that the resulting nodes cannot be checked.
    fn type_to_type_node(&self, type_: Type, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<TypeNode>;
    /// @internal
    fn type_to_type_node_with_internal_flags(&self, type_: Type, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>, internal_flags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<TypeNode>;
    /// @internal
    fn type_predicate_to_type_predicate_node(&self, type_predicate: TypePredicate, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>, internal_flags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<TypePredicateNode>;
    /// Note that the resulting nodes cannot be checked.
    fn signature_to_signature_declaration(&self, signature: Signature, kind: SyntaxKind, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<SignatureDeclaration>;
    /// @internal
    fn signature_to_signature_declaration_with_internal_flags(&self, signature: Signature, kind: SyntaxKind, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>, internal_flags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<SignatureDeclaration>;
    /// Note that the resulting nodes cannot be checked.
    fn index_info_to_index_signature_declaration(&self, index_info: IndexInfo, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<IndexSignatureDeclaration>;
    /// @internal
    fn index_info_to_index_signature_declaration_with_internal_flags(&self, index_info: IndexInfo, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>, internal_flags: Option<InternalNodeBuilderFlags>, tracker: Option<SymbolTracker>) -> Option<IndexSignatureDeclaration>;
    /// Note that the resulting nodes cannot be checked.
    fn symbol_to_entity_name(&self, symbol: Symbol, meaning: SymbolFlags, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<EntityName>;
    /// Note that the resulting nodes cannot be checked.
    fn symbol_to_expression(&self, symbol: Symbol, meaning: SymbolFlags, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<Expression>;
    /// Note that the resulting nodes cannot be checked.
    ///
    /// @internal
    fn symbol_to_node(&self, symbol: Symbol, meaning: SymbolFlags, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>, internal_flags: Option<InternalNodeBuilderFlags>) -> Option<Node>;
    /// Note that the resulting nodes cannot be checked.
    fn symbol_to_type_parameter_declarations(&self, symbol: Symbol, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<Vec<TypeParameterDeclaration>>;
    /// Note that the resulting nodes cannot be checked.
    fn symbol_to_parameter_declaration(&self, symbol: Symbol, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<ParameterDeclaration>;
    /// Note that the resulting nodes cannot be checked.
    fn type_parameter_to_declaration(&self, parameter: TypeParameter, enclosing_declaration: Option<Node>, flags: Option<NodeBuilderFlags>) -> Option<TypeParameterDeclaration>;

    fn get_symbols_in_scope(&self, location: Node, meaning: SymbolFlags) -> Vec<Symbol>;
    fn get_symbol_at_location(&self, node: Node) -> Option<Symbol>;
    /// @internal
    fn get_index_infos_at_location(&self, node: Node) -> Option<Vec<IndexInfo>>;
    fn get_symbols_of_parameter_property_declaration(&self, parameter: ParameterDeclaration, parameter_name: &str) -> Vec<Symbol>;
    /// The function returns the value (local variable) symbol of an identifier in the short-hand property assignment.
    /// This is necessary as an identifier in short-hand property assignment can contains two meaning: property name and property value.
    fn get_shorthand_assignment_value_symbol(&self, location: Option<Node>) -> Option<Symbol>;

    fn get_export_specifier_local_target_symbol(&self, location: ExportSpecifier) -> Option<Symbol>;
    /// If a symbol is a local symbol with an associated exported symbol, returns the exported symbol.
    /// Otherwise returns its input.
    /// For example, at `export type T = number;`:
    ///     - `get_symbol_at_location` at the location `T` will return the exported symbol for `T`.
    ///     - But the result of `get_symbols_in_scope` will contain the *local* symbol for `T`, not the exported symbol.
    ///     - Calling `get_export_symbol_of_symbol` on that local symbol will return the exported symbol.
    fn get_export_symbol_of_symbol(&self, symbol: Symbol) -> Symbol;
    fn get_property_symbol_of_destructuring_assignment(&self, location: Identifier) -> Option<Symbol>;
    fn get_type_of_assignment_pattern(&self, pattern: AssignmentPattern) -> Type;
    fn get_type_at_location(&self, node: Node) -> Type;
    fn get_type_from_type_node(&self, node: TypeNode) -> Type;

    fn signature_to_string(&self, signature: Signature, enclosing_declaration: Option<Node>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>) -> String;
    fn type_to_string(&self, type_: Type, enclosing_declaration: Option<Node>, flags: Option<TypeFormatFlags>) -> String;
    fn symbol_to_string(&self, symbol: Symbol, enclosing_declaration: Option<Node>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>) -> String;
    fn type_predicate_to_string(&self, predicate: TypePredicate, enclosing_declaration: Option<Node>, flags: Option<TypeFormatFlags>) -> String;

    /// @internal
    fn write_signature(&self, signature: Signature, enclosing_declaration: Option<Node>, flags: Option<TypeFormatFlags>, kind: Option<SignatureKind>, writer: Option<EmitTextWriter>) -> String;
    /// @internal
    fn write_type(&self, type_: Type, enclosing_declaration: Option<Node>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String;
    /// @internal
    fn write_symbol(&self, symbol: Symbol, enclosing_declaration: Option<Node>, meaning: Option<SymbolFlags>, flags: Option<SymbolFormatFlags>, writer: Option<EmitTextWriter>) -> String;
    /// @internal
    fn write_type_predicate(&self, predicate: TypePredicate, enclosing_declaration: Option<Node>, flags: Option<TypeFormatFlags>, writer: Option<EmitTextWriter>) -> String;

    fn get_fully_qualified_name(&self, symbol: Symbol) -> String;
    fn get_augmented_properties_of_type(&self, type_: Type) -> Vec<Symbol>;

    fn get_root_symbols(&self, symbol: Symbol) -> Vec<Symbol>;
    fn get_symbol_of_expando(&self, node: Node, allow_declaration: bool) -> Option<Symbol>;
    fn get_contextual_type(&self, node: Expression) -> Option<Type>;
    /// @internal
    fn get_contextual_type_with_flags(&self, node: Expression, context_flags: Option<ContextFlags>) -> Option<Type>;
    /// @internal
    fn get_contextual_type_for_object_literal_element(&self, element: ObjectLiteralElementLike) -> Option<Type>;
    /// @internal
    fn get_contextual_type_for_argument_at_index(&self, call: CallLikeExpression, arg_index: usize) -> Option<Type>;
    /// @internal
    fn get_contextual_type_for_jsx_attribute(&self, attribute: JsxAttribute) -> Option<Type>;
    /// @internal
    fn is_context_sensitive(&self, node: Expression) -> bool;
    /// @internal
    fn get_type_of_property_of_contextual_type(&self, type_: Type, name: &str) -> Option<Type>;

    /// returns unknownSignature in the case of an error.
    /// returns undefined if the node is not valid.
    /// @param argument_count Apparent number of arguments, passed in case of a possibly incomplete call. This should come from an ArgumentListInfo. See `signatureHelp.ts`.
    fn get_resolved_signature(&self, node: CallLikeExpression, candidates_out_array: Option<Vec<Signature>>, argument_count: Option<usize>) -> Option<Signature>;
    /// @internal
    fn get_resolved_signature_for_signature_help(&self, node: CallLikeExpression, candidates_out_array: Option<Vec<Signature>>, argument_count: Option<usize>) -> Option<Signature>;
    /// @internal
    fn get_candidate_signatures_for_string_literal_completions(&self, call: CallLikeExpression, editing_argument: Node) -> Vec<Signature>;
    /// @internal
    fn get_expanded_parameters(&self, sig: Signature) -> Vec<Vec<Symbol>>;
    /// @internal
    fn has_effective_rest_parameter(&self, sig: Signature) -> bool;
    /// @internal
    fn contains_arguments_reference(&self, declaration: SignatureDeclaration) -> bool;

    fn get_signature_from_declaration(&self, declaration: SignatureDeclaration) -> Option<Signature>;
    fn is_implementation_of_overload(&self, node: SignatureDeclaration) -> Option<bool>;
    fn is_undefined_symbol(&self, symbol: Symbol) -> bool;
    fn is_arguments_symbol(&self, symbol: Symbol) -> bool;
    fn is_unknown_symbol(&self, symbol: Symbol) -> bool;
    fn get_merged_symbol(&self, symbol: Symbol) -> Symbol;
    /// @internal
    fn symbol_is_value(&self, symbol: Symbol, include_type_only_members: Option<bool>) -> bool;

    fn get_constant_value(&self, node: EnumMember) -> Option<String>;
    fn is_valid_property_access(&self, node: PropertyAccessExpression, property_name: &str) -> bool;
    /// Exclude accesses to private properties.
    ///
    /// @internal
    fn is_valid_property_access_for_completions(&self, node: PropertyAccessExpression, type_: Type, property: Symbol) -> bool;
    /// Follow all aliases to get the original symbol.
    fn get_aliased_symbol(&self, symbol: Symbol) -> Symbol;
    /// Follow a *single* alias to get the immediately aliased symbol.
    fn get_immediate_aliased_symbol(&self, symbol: Symbol) -> Option<Symbol>;
    fn get_exports_of_module(&self, module_symbol: Symbol) -> Vec<Symbol>;
    /// Unlike `get_exports_of_module`, this includes properties of an `export =` value.
    ///
    /// @internal
    fn get_exports_and_properties_of_module(&self, module_symbol: Symbol) -> Vec<Symbol>;
    /// @internal
    fn for_each_export_and_property_of_module(&self, module_symbol: Symbol, cb: impl Fn(Symbol, &str));
    fn get_jsx_intrinsic_tag_names_at(&self, location: Node) -> Vec<Symbol>;
    fn is_optional_parameter(&self, node: ParameterDeclaration) -> bool;
    fn get_ambient_modules(&self) -> Vec<Symbol>;

    fn try_get_member_in_module_exports(&self, member_name: &str, module_symbol: Symbol) -> Option<Symbol>;
    /// Unlike `try_get_member_in_module_exports`, this includes properties of an `export =` value.
    /// Does *not* return properties of primitive types.
    ///
    /// @internal
    fn try_get_member_in_module_exports_and_properties(&self, member_name: &str, module_symbol: Symbol) -> Option<Symbol>;
    fn get_apparent_type(&self, type_: Type) -> Type;
    /// @internal
    fn get_suggested_symbol_for_nonexistent_property(&self, name: MemberName, containing_type: Type) -> Option<Symbol>;
    /// @internal
    fn get_suggested_symbol_for_nonexistent_jsx_attribute(&self, name: Identifier, containing_type: Type) -> Option<Symbol>;
    /// @internal
    fn get_suggested_symbol_for_nonexistent_symbol(&self, location: Node, name: &str, meaning: SymbolFlags) -> Option<Symbol>;
    /// @internal
    fn get_suggested_symbol_for_nonexistent_module(&self, node: Identifier, target: Symbol) -> Option<Symbol>;
    /// @internal
    fn get_suggested_symbol_for_nonexistent_class_member(&self, name: &str, base_type: Type) -> Option<Symbol>;
    fn get_base_constraint_of_type(&self, type_: Type) -> Option<Type>;
    fn get_default_from_type_parameter(&self, type_: Type) -> Option<Type>;

    /// Gets the intrinsic `any` type. There are multiple types that act as `any` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `any`. Instead, use `type.flags & TypeFlags.Any`.
    fn get_any_type(&self) -> Type;
    fn get_string_type(&self) -> Type;
    fn get_string_literal_type(&self, value: &str) -> StringLiteralType;
    fn get_number_type(&self) -> Type;
    fn get_number_literal_type(&self, value: f64) -> NumberLiteralType;
    fn get_big_int_type(&self) -> Type;
    fn get_big_int_literal_type(&self, value: PseudoBigInt) -> BigIntLiteralType;
    fn get_boolean_type(&self) -> Type;
    /// @internal
    fn get_false_type(&self, fresh: Option<bool>) -> Type;
    /// @internal
    fn get_true_type(&self, fresh: Option<bool>) -> Type;
    fn get_void_type(&self) -> Type;
    /// Gets the intrinsic `undefined` type. There are multiple types that act as `undefined` used internally in the compiler
    /// depending on compiler options, so the type returned by this function should not be used in equality checks to determine
    /// if another type is `undefined`. Instead, use `type.flags & TypeFlags.Undefined`.
    fn get_undefined_type(&self) -> Type;
    /// Gets the intrinsic `null` type. There are multiple types that act as `null` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `null`. Instead, use `type.flags & TypeFlags.Null`.
    fn get_null_type(&self) -> Type;
    fn get_es_symbol_type(&self) -> Type;
    /// Gets the intrinsic `never` type. There are multiple types that act as `never` used internally in the compiler,
    /// so the type returned by this function should not be used in equality checks to determine if another type
    /// is `never`. Instead, use `type.flags & TypeFlags.Never`.
    fn get_never_type(&self) -> Type;
    /// @internal
    fn get_optional_type(&self) -> Type;
    /// @internal
    fn get_union_type(&self, types: Vec<Type>, subtype_reduction: Option<UnionReduction>) -> Type;
    /// @internal
    fn create_array_type(&self, element_type: Type) -> Type;
    /// @internal
    fn get_element_type_of_array_type(&self, array_type: Type) -> Option<Type>;
    /// @internal
    fn create_promise_type(&self, type_: Type) -> Type;
    /// @internal
    fn get_promise_type(&self) -> Type;
    /// @internal
    fn get_promise_like_type(&self) -> Type;
    /// @internal
    fn get_any_async_iterable_type(&self) -> Option<Type>;

    /// Returns true if the "source" type is assignable to the "target" type.
    fn is_type_assignable_to(&self, source: Type, target: Type) -> bool;
    /// @internal
    fn create_anonymous_type(&self, symbol: Option<Symbol>, members: SymbolTable, call_signatures: Vec<Signature>, construct_signatures: Vec<Signature>, index_infos: Vec<IndexInfo>) -> Type;
    /// @internal
    fn create_signature(
        &self, declaration: Option<SignatureDeclaration>, type_parameters: Option<Vec<TypeParameter>>, this_parameter: Option<Symbol>, parameters: Vec<Symbol>, resolved_return_type: Type, type_predicate: Option<TypePredicate>, min_argument_count: usize, flags: SignatureFlags,
    ) -> Signature;
    /// @internal
    fn create_symbol(&self, flags: SymbolFlags, name: &str) -> TransientSymbol;
    /// @internal
    fn create_index_info(&self, key_type: Type, type_: Type, is_readonly: bool, declaration: Option<SignatureDeclaration>) -> IndexInfo;
    /// @internal
    fn is_symbol_accessible(&self, symbol: Symbol, enclosing_declaration: Option<Node>, meaning: SymbolFlags, should_compute_alias_to_mark_visible: bool) -> SymbolAccessibilityResult;
    /// @internal
    fn try_find_ambient_module(&self, module_name: &str) -> Option<Symbol>;

    /// @internal
    fn get_symbol_walker(&self, accept: Option<fn(Symbol) -> bool>) -> SymbolWalker;

    // Should not be called directly.  Should only be accessed through the Program instance.
    /// @internal
    fn get_diagnostics(&self, source_file: Option<SourceFile>, cancellation_token: Option<CancellationToken>, nodes_to_check: Option<Vec<Node>>) -> Vec<Diagnostic>;
    /// @internal
    fn get_global_diagnostics(&self) -> Vec<Diagnostic>;
    /// @internal
    fn get_emit_resolver(&self, source_file: Option<SourceFile>, cancellation_token: Option<CancellationToken>, force_dts: Option<bool>) -> EmitResolver;
    /// @internal
    fn requires_adding_implicit_undefined(&self, parameter: ParameterDeclaration, enclosing_declaration: Option<Node>) -> bool;

    /// @internal
    fn get_node_count(&self) -> usize;
    /// @internal
    fn get_identifier_count(&self) -> usize;
    /// @internal
    fn get_symbol_count(&self) -> usize;
    /// @internal
    fn get_type_count(&self) -> usize;
    /// @internal
    fn get_instantiation_count(&self) -> usize;
    /// @internal
    fn get_relation_cache_sizes(&self) -> (usize, usize, usize, usize);
    /// @internal
    fn get_recursion_identity(&self, type_: Type) -> Option<Type>;
    /// @internal
    fn get_unmatched_properties(&self, source: Type, target: Type, require_optional_properties: bool, match_discriminant_properties: bool) -> Box<dyn Iterator<Item = Symbol>>;

    /// True if this type is the `Array` or `ReadonlyArray` type from lib.d.ts.
    /// This function will _not_ return true if passed a type which
    /// extends `Array` (for example, the TypeScript AST's `NodeArray` type).
    fn is_array_type(&self, type_: Type) -> bool;
    /// True if this type is a tuple type. This function will _not_ return true if
    /// passed a type which extends from a tuple.
    fn is_tuple_type(&self, type_: Type) -> bool;
    /// True if this type is assignable to `ReadonlyArray<any>`.
    fn is_array_like_type(&self, type_: Type) -> bool;

    /// True if `contextualType` should not be considered for completions because
    /// e.g. it specifies `kind: "a"` and obj has `kind: "b"`.
    ///
    /// @internal
    fn is_type_invalid_due_to_union_discriminant(&self, contextual_type: Type, obj: ObjectLiteralExpression) -> bool;
    /// @internal
    fn get_exact_optional_properties(&self, type_: Type) -> Vec<Symbol>;
    /// For a union, will include a property if it's defined in *any* of the member types.
    /// So for `{ a } | { b }`, this will include both `a` and `b`.
    /// Does not include properties of primitive types.
    ///
    /// @internal
    fn get_all_possible_properties_of_types(&self, types: Vec<Type>) -> Vec<Symbol>;
    fn resolve_name(&self, name: &str, location: Option<Node>, meaning: SymbolFlags, exclude_globals: bool) -> Option<Symbol>;
    /// @internal
    fn get_jsx_namespace(&self, location: Option<Node>) -> String;
    /// @internal
    fn get_jsx_fragment_factory(&self, location: Node) -> Option<String>;

    /// Note that this will return undefined in the following case:
    ///     // a.ts
    ///     export namespace N { export class C { } }
    ///     // b.ts
    ///     <<enclosingDeclaration>>
    /// Where `C` is the symbol we're looking for.
    /// This should be called in a loop climbing parents of the symbol, so we'll get `N`.
    ///
    /// @internal
    fn get_accessible_symbol_chain(&self, symbol: Symbol, enclosing_declaration: Option<Node>, meaning: SymbolFlags, use_only_external_aliasing: bool) -> Option<Vec<Symbol>>;
    fn get_type_predicate_of_signature(&self, signature: Signature) -> Option<TypePredicate>;
    /// @internal
    fn resolve_external_module_name(&self, module_specifier: Expression) -> Option<Symbol>;
    /// An external module with an 'export =' declaration resolves to the target of the 'export =' declaration,
    /// and an external module with no 'export =' declaration resolves to the module itself.
    ///
    /// @internal
    fn resolve_external_module_symbol(&self, symbol: Symbol) -> Symbol;
    /// @param node A location where we might consider accessing `this`. Not necessarily a ThisExpression.
    ///
    /// @internal
    fn try_get_this_type_at(&self, node: Node, include_global_this: Option<bool>, container: Option<ThisContainer>) -> Option<Type>;
    /// @internal
    fn get_type_argument_constraint(&self, node: TypeNode) -> Option<Type>;

    /// Does *not* get *all* suggestion diagnostics, just the ones that were convenient to report in the checker.
    /// Others are added in computeSuggestionDiagnostics.
    ///
    /// @internal
    fn get_suggestion_diagnostics(&self, file: SourceFile, cancellation_token: Option<CancellationToken>) -> Vec<DiagnosticWithLocation>;

    /// Depending on the operation performed, it may be appropriate to throw away the checker
    /// if the cancellation token is triggered. Typically, if it is used for error checking
    /// and the operation is cancelled, then it should be discarded, otherwise it is safe to keep.
    /// @internal `token = None`
    fn run_with_cancellationToken<T>(&self, token: Option<CancellationToken>, cb: impl Fn() -> T) -> T;

    /// @internal
    fn get_local_type_parameters_of_class_or_interface_or_type_alias(&self, symbol: Symbol) -> Option<Vec<TypeParameter>>;
    /// @internal
    fn is_declaration_visible(&self, node: Declaration) -> bool;
    /// @internal
    fn is_property_accessible(&self, node: Node, is_super: bool, is_write: bool, containing_type: Type, property: Symbol) -> bool;
    /// @internal
    fn get_type_only_alias_declaration(&self, symbol: Symbol) -> Option<TypeOnlyAliasDeclaration>;
    /// @internal
    fn get_member_override_modifier_status(&self, node: ClassLikeDeclaration, member: ClassElement, member_symbol: Symbol) -> MemberOverrideStatus;
    /// @internal
    fn is_type_parameter_possibly_referenced(&self, tp: TypeParameter, node: Node) -> bool;
    /// @internal
    fn type_has_call_or_construct_signatures(&self, type_: Type) -> bool;
    /// @internal
    fn get_symbol_flags(&self, symbol: Symbol) -> SymbolFlags;
}

#[derive(Debug, Clone, Copy)]
pub struct TypeFlags(pub isize);

impl TypeFlags {
    pub const ANY: TypeFlags = TypeFlags(1 << 0);
    pub const UNKNOWN: TypeFlags = TypeFlags(1 << 1);
    pub const STRING: TypeFlags = TypeFlags(1 << 2);
    pub const NUMBER: TypeFlags = TypeFlags(1 << 3);
    pub const BOOLEAN: TypeFlags = TypeFlags(1 << 4);
    pub const ENUM: TypeFlags = TypeFlags(1 << 5); // Numeric computed enum member value
    pub const BIG_INT: TypeFlags = TypeFlags(1 << 6);
    pub const STRING_LITERAL: TypeFlags = TypeFlags(1 << 7);
    pub const NUMBER_LITERAL: TypeFlags = TypeFlags(1 << 8);
    pub const BOOLEAN_LITERAL: TypeFlags = TypeFlags(1 << 9);
    pub const ENUM_LITERAL: TypeFlags = TypeFlags(1 << 10); // Always combined with StringLiteral, NumberLiteral, or Union
    pub const BIG_INT_LITERAL: TypeFlags = TypeFlags(1 << 11);
    pub const ES_SYMBOL: TypeFlags = TypeFlags(1 << 12); // Type of symbol primitive introduced in ES6
    pub const UNIQUE_ES_SYMBOL: TypeFlags = TypeFlags(1 << 13); // unique symbol
    pub const VOID: TypeFlags = TypeFlags(1 << 14);
    pub const UNDEFINED: TypeFlags = TypeFlags(1 << 15);
    pub const NULL: TypeFlags = TypeFlags(1 << 16);
    pub const NEVER: TypeFlags = TypeFlags(1 << 17); // Never type
    pub const TYPE_PARAMETER: TypeFlags = TypeFlags(1 << 18); // Type parameter
    pub const OBJECT: TypeFlags = TypeFlags(1 << 19); // Object type
    pub const UNION: TypeFlags = TypeFlags(1 << 20); // Union (T | U)
    pub const INTERSECTION: TypeFlags = TypeFlags(1 << 21); // Intersection (T & U)
    pub const INDEX: TypeFlags = TypeFlags(1 << 22); // keyof T
    pub const INDEXED_ACCESS: TypeFlags = TypeFlags(1 << 23); // T[K]
    pub const CONDITIONAL: TypeFlags = TypeFlags(1 << 24); // T extends U ? X : Y
    pub const SUBSTITUTION: TypeFlags = TypeFlags(1 << 25); // Type parameter substitution
    pub const NON_PRIMITIVE: TypeFlags = TypeFlags(1 << 26); // intrinsic object type
    pub const TEMPLATE_LITERAL: TypeFlags = TypeFlags(1 << 27); // Template literal type
    pub const STRING_MAPPING: TypeFlags = TypeFlags(1 << 28); // Uppercase/Lowercase type
    /** @internal */
    pub const RESERVED1: TypeFlags = TypeFlags(1 << 29); // Used by union/intersection type construction
    /** @internal */
    pub const RESERVED2: TypeFlags = TypeFlags(1 << 30); // Used by union/intersection type construction

    /** @internal */
    pub const ANY_OR_UNKNOWN: TypeFlags = TypeFlags(Self::ANY.0 | Self::UNKNOWN.0);
    /** @internal */
    pub const NULLABLE: TypeFlags = TypeFlags(Self::UNDEFINED.0 | Self::NULL.0);
    pub const LITERAL: TypeFlags = TypeFlags(Self::STRING_LITERAL.0 | Self::NUMBER_LITERAL.0 | Self::BIG_INT_LITERAL.0 | Self::BOOLEAN_LITERAL.0);
    pub const UNIT: TypeFlags = TypeFlags(Self::ENUM.0 | Self::LITERAL.0 | Self::UNIQUE_ES_SYMBOL.0 | Self::NULLABLE.0);
    pub const FRESHABLE: TypeFlags = TypeFlags(Self::ENUM.0 | Self::LITERAL.0);
    pub const STRING_OR_NUMBER_LITERAL: TypeFlags = TypeFlags(Self::STRING_LITERAL.0 | Self::NUMBER_LITERAL.0);
    /** @internal */
    pub const STRING_OR_NUMBER_LITERAL_OR_UNIQUE: TypeFlags = TypeFlags(Self::STRING_LITERAL.0 | Self::NUMBER_LITERAL.0 | Self::UNIQUE_ES_SYMBOL.0);
    /** @internal */
    pub const DEFINITELY_FALSY: TypeFlags = TypeFlags(Self::STRING_LITERAL.0 | Self::NUMBER_LITERAL.0 | Self::BIG_INT_LITERAL.0 | Self::BOOLEAN_LITERAL.0 | Self::VOID.0 | Self::UNDEFINED.0 | Self::NULL.0);
    pub const POSSIBLY_FALSY: TypeFlags = TypeFlags(Self::DEFINITELY_FALSY.0 | Self::STRING.0 | Self::NUMBER.0 | Self::BIG_INT.0 | Self::BOOLEAN.0);
    /** @internal */
    pub const INTRINSIC: TypeFlags = TypeFlags(Self::ANY.0 | Self::UNKNOWN.0 | Self::STRING.0 | Self::NUMBER.0 | Self::BIG_INT.0 | Self::BOOLEAN.0 | Self::BOOLEAN_LITERAL.0 | Self::ES_SYMBOL.0 | Self::VOID.0 | Self::UNDEFINED.0 | Self::NULL.0 | Self::NEVER.0 | Self::NON_PRIMITIVE.0);
    pub const STRING_LIKE: TypeFlags = TypeFlags(Self::STRING.0 | Self::STRING_LITERAL.0 | Self::TEMPLATE_LITERAL.0 | Self::STRING_MAPPING.0);
    pub const NUMBER_LIKE: TypeFlags = TypeFlags(Self::NUMBER.0 | Self::NUMBER_LITERAL.0 | Self::ENUM.0);
    pub const BIG_INT_LIKE: TypeFlags = TypeFlags(Self::BIG_INT.0 | Self::BIG_INT_LITERAL.0);
    pub const BOOLEAN_LIKE: TypeFlags = TypeFlags(Self::BOOLEAN.0 | Self::BOOLEAN_LITERAL.0);
    pub const ENUM_LIKE: TypeFlags = TypeFlags(Self::ENUM.0 | Self::ENUM_LITERAL.0);
    pub const ES_SYMBOL_LIKE: TypeFlags = TypeFlags(Self::ES_SYMBOL.0 | Self::UNIQUE_ES_SYMBOL.0);
    pub const VOID_LIKE: TypeFlags = TypeFlags(Self::VOID.0 | Self::UNDEFINED.0);
    /** @internal */
    pub const PRIMITIVE: TypeFlags = TypeFlags(Self::STRING_LIKE.0 | Self::NUMBER_LIKE.0 | Self::BIG_INT_LIKE.0 | Self::BOOLEAN_LIKE.0 | Self::ENUM_LIKE.0 | Self::ES_SYMBOL_LIKE.0 | Self::VOID_LIKE.0 | Self::NULL.0);
    /** @internal */
    pub const DEFINITELY_NON_NULLABLE: TypeFlags = TypeFlags(Self::STRING_LIKE.0 | Self::NUMBER_LIKE.0 | Self::BIG_INT_LIKE.0 | Self::BOOLEAN_LIKE.0 | Self::ENUM_LIKE.0 | Self::ES_SYMBOL_LIKE.0 | Self::OBJECT.0 | Self::NON_PRIMITIVE.0);
    /** @internal */
    pub const DISJOINT_DOMAINS: TypeFlags = TypeFlags(Self::NON_PRIMITIVE.0 | Self::STRING_LIKE.0 | Self::NUMBER_LIKE.0 | Self::BIG_INT_LIKE.0 | Self::BOOLEAN_LIKE.0 | Self::ES_SYMBOL_LIKE.0 | Self::VOID_LIKE.0 | Self::NULL.0);
    pub const UNION_OR_INTERSECTION: TypeFlags = TypeFlags(Self::UNION.0 | Self::INTERSECTION.0);
    pub const STRUCTURED_TYPE: TypeFlags = TypeFlags(Self::OBJECT.0 | Self::UNION.0 | Self::INTERSECTION.0);
    pub const TYPE_VARIABLE: TypeFlags = TypeFlags(Self::TYPE_PARAMETER.0 | Self::INDEXED_ACCESS.0);
    pub const INSTANTIABLE_NON_PRIMITIVE: TypeFlags = TypeFlags(Self::TYPE_VARIABLE.0 | Self::CONDITIONAL.0 | Self::SUBSTITUTION.0);
    pub const INSTANTIABLE_PRIMITIVE: TypeFlags = TypeFlags(Self::INDEX.0 | Self::TEMPLATE_LITERAL.0 | Self::STRING_MAPPING.0);
    pub const INSTANTIABLE: TypeFlags = TypeFlags(Self::INSTANTIABLE_NON_PRIMITIVE.0 | Self::INSTANTIABLE_PRIMITIVE.0);
    pub const STRUCTURED_OR_INSTANTIABLE: TypeFlags = TypeFlags(Self::STRUCTURED_TYPE.0 | Self::INSTANTIABLE.0);
    /** @internal */
    pub const OBJECT_FLAGS_TYPE: TypeFlags = TypeFlags(Self::ANY.0 | Self::NULLABLE.0 | Self::NEVER.0 | Self::OBJECT.0 | Self::UNION.0 | Self::INTERSECTION.0);
    /** @internal */
    pub const SIMPLIFIABLE: TypeFlags = TypeFlags(Self::INDEXED_ACCESS.0 | Self::CONDITIONAL.0);
    /** @internal */
    pub const SINGLETON: TypeFlags = TypeFlags(Self::ANY.0 | Self::UNKNOWN.0 | Self::STRING.0 | Self::NUMBER.0 | Self::BOOLEAN.0 | Self::BIG_INT.0 | Self::ES_SYMBOL.0 | Self::VOID.0 | Self::UNDEFINED.0 | Self::NULL.0 | Self::NEVER.0 | Self::NON_PRIMITIVE.0);
    // 'Narrowable' types are types where narrowing actually narrows.
    // This *should* be every type other than null, undefined, void, and never
    pub const NARROWABLE: TypeFlags = TypeFlags(Self::ANY.0 | Self::UNKNOWN.0 | Self::STRUCTURED_OR_INSTANTIABLE.0 | Self::STRING_LIKE.0 | Self::NUMBER_LIKE.0 | Self::BIG_INT_LIKE.0 | Self::BOOLEAN_LIKE.0 | Self::ES_SYMBOL.0 | Self::UNIQUE_ES_SYMBOL.0 | Self::NON_PRIMITIVE.0);
    // The following flags are aggregated during union and intersection type construction
    /** @internal */
    pub const INCLUDES_MASK: TypeFlags = TypeFlags(Self::ANY.0 | Self::UNKNOWN.0 | Self::PRIMITIVE.0 | Self::NEVER.0 | Self::OBJECT.0 | Self::UNION.0 | Self::INTERSECTION.0 | Self::NON_PRIMITIVE.0 | Self::TEMPLATE_LITERAL.0 | Self::STRING_MAPPING.0);
    // The following flags are used for different purposes during union and intersection type construction
    /** @internal */
    pub const INCLUDES_MISSING_TYPE: TypeFlags = TypeFlags(Self::TYPE_PARAMETER.0);
    /** @internal */
    pub const INCLUDES_NON_WIDENING_TYPE: TypeFlags = TypeFlags(Self::INDEX.0);
    /** @internal */
    pub const INCLUDES_WILDCARD: TypeFlags = TypeFlags(Self::INDEXED_ACCESS.0);
    /** @internal */
    pub const INCLUDES_EMPTY_OBJECT: TypeFlags = TypeFlags(Self::CONDITIONAL.0);
    /** @internal */
    pub const INCLUDES_INSTANTIABLE: TypeFlags = TypeFlags(Self::SUBSTITUTION.0);
    /** @internal */
    pub const INCLUDES_CONSTRAINED_TYPE_VARIABLE: TypeFlags = TypeFlags(Self::RESERVED1.0);
    /** @internal */
    pub const INCLUDES_ERROR: TypeFlags = TypeFlags(Self::RESERVED2.0);
    /** @internal */
    pub const NOT_PRIMITIVE_UNION: TypeFlags = TypeFlags(Self::ANY.0 | Self::UNKNOWN.0 | Self::VOID.0 | Self::NEVER.0 | Self::OBJECT.0 | Self::INTERSECTION.0 | Self::INCLUDES_INSTANTIABLE.0);

    pub fn contains(&self, flags: TypeFlags) -> bool {
        (self.0 & flags.0) == flags.0
    }
}

impl std::ops::BitOr for TypeFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        TypeFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for TypeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        TypeFlags(self.0 & rhs.0)
    }
}

#[derive(Debug)]
pub enum DiagnosticCategory {
    Error,
    Warning,
    Suggestion,
    Message,
}

#[derive(Debug)]
pub struct DiagnosticMessage {
    pub code: i32,
    pub category: DiagnosticCategory,
    pub key: String,
    pub message: String,
    pub reports_unnecessary: Option<bool>,
    pub elided_in_compatibility_pyramid: Option<bool>,
    pub reports_deprecated: Option<bool>,
}
