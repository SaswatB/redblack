use crate::compiler::types::*;
use oxc::ast::{
    ast::{Argument, Expression, ObjectExpression},
    AstKind,
};

#[derive(Debug)]
pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::compiler::types::TypeChecker for TypeChecker {
    fn get_type_of_symbol_at_location(&self, _symbol: Symbol, _node: AstKind) -> Type {
        todo!("get_type_of_symbol_at_location")
    }

    fn get_type_of_symbol(&self, _symbol: Symbol) -> Type {
        todo!("get_type_of_symbol")
    }

    fn get_declared_type_of_symbol(&self, _symbol: Symbol) -> Type {
        todo!("get_declared_type_of_symbol")
    }

    fn get_properties_of_type(&self, _type_: Type) -> Vec<Symbol> {
        todo!("get_properties_of_type")
    }

    fn get_property_of_type(&self, _type_: Type, _property_name: &str) -> Option<Symbol> {
        todo!("get_property_of_type")
    }

    fn get_private_identifier_property_of_type(&self, _left_type: Type, _name: &str, _location: AstKind) -> Option<Symbol> {
        todo!("get_private_identifier_property_of_type")
    }

    fn get_type_of_property_of_type(&self, _type_: Type, _property_name: &str) -> Option<Type> {
        todo!("get_type_of_property_of_type")
    }

    fn get_index_info_of_type(&self, _type_: Type, _kind: IndexKind) -> Option<IndexInfo> {
        todo!("get_index_info_of_type")
    }

    fn get_index_infos_of_type(&self, _type_: Type) -> Vec<IndexInfo> {
        todo!("get_index_infos_of_type")
    }

    fn get_index_infos_of_index_symbol(&self, _index_symbol: Symbol) -> Vec<IndexInfo> {
        todo!("get_index_infos_of_index_symbol")
    }

    fn get_signatures_of_type(&self, _type_: Type, _kind: SignatureKind) -> Vec<Signature> {
        todo!("get_signatures_of_type")
    }

    fn get_index_type_of_type(&self, _type_: Type, _kind: IndexKind) -> Option<Type> {
        todo!("get_index_type_of_type")
    }

    fn get_index_type(&self, _type_: Type) -> Type {
        todo!("get_index_type")
    }

    fn get_base_types(&self, _type_: InterfaceType) -> Vec<BaseType> {
        todo!("get_base_types")
    }

    fn get_base_type_of_literal_type(&self, _type_: Type) -> Type {
        todo!("get_base_type_of_literal_type")
    }

    fn get_widened_type(&self, _type_: Type) -> Type {
        todo!("get_widened_type")
    }

    fn get_widened_literal_type(&self, _type_: Type) -> Type {
        todo!("get_widened_literal_type")
    }

    fn get_promised_type_of_promise(&self, _promise: Type, _error_node: Option<AstKind>) -> Option<Type> {
        todo!("get_promised_type_of_promise")
    }

    fn get_awaited_type(&self, _type_: Type) -> Option<Type> {
        todo!("get_awaited_type")
    }

    fn is_empty_anonymous_object_type(&self, _type_: Type) -> bool {
        todo!("is_empty_anonymous_object_type")
    }

    fn get_return_type_of_signature(&self, _signature: Signature) -> Type {
        todo!("get_return_type_of_signature")
    }

    fn get_parameter_type(&self, _signature: Signature, _parameter_index: usize) -> Type {
        todo!("get_parameter_type")
    }

    fn get_parameter_identifier_info_at_position(&self, _signature: Signature, _parameter_index: usize) -> Option<(Identifier, &str, bool)> {
        todo!("get_parameter_identifier_info_at_position")
    }

    fn get_nullable_type(&self, _type_: Type, _flags: TypeFlags) -> Type {
        todo!("get_nullable_type")
    }

    fn get_non_nullable_type(&self, _type_: Type) -> Type {
        todo!("get_non_nullable_type")
    }

    fn get_non_optional_type(&self, _type_: Type) -> Type {
        todo!("get_non_optional_type")
    }

    fn is_nullable_type(&self, _type_: Type) -> bool {
        todo!("is_nullable_type")
    }

    fn get_type_arguments(&self, _type_: TypeReference) -> Vec<Type> {
        todo!("get_type_arguments")
    }

    fn get_symbols_in_scope(&self, _location: AstKind, _meaning: SymbolFlags) -> Vec<Symbol> {
        todo!("get_symbols_in_scope")
    }

    fn get_symbol_at_location(&self, _node: AstKind) -> Option<Symbol> {
        todo!("get_symbol_at_location")
    }

    fn get_index_infos_at_location(&self, _node: AstKind) -> Option<Vec<IndexInfo>> {
        todo!("get_index_infos_at_location")
    }

    fn get_symbols_of_parameter_property_declaration(&self, _parameter: Argument, _parameter_name: &str) -> Vec<Symbol> {
        todo!("get_symbols_of_parameter_property_declaration")
    }

    fn get_shorthand_assignment_value_symbol(&self, _location: Option<AstKind>) -> Option<Symbol> {
        todo!("get_shorthand_assignment_value_symbol")
    }

    fn get_export_specifier_local_target_symbol(&self, _location: ExportSpecifier) -> Option<Symbol> {
        todo!("get_export_specifier_local_target_symbol")
    }

    fn get_export_symbol_of_symbol(&self, _symbol: Symbol) -> Symbol {
        todo!("get_export_symbol_of_symbol")
    }

    fn get_property_symbol_of_destructuring_assignment(&self, _location: Identifier) -> Option<Symbol> {
        todo!("get_property_symbol_of_destructuring_assignment")
    }

    fn get_type_of_assignment_pattern(&self, _pattern: AssignmentPattern) -> Type {
        todo!("get_type_of_assignment_pattern")
    }

    fn get_type_at_location(&self, _node: AstKind) -> Type {
        todo!("get_type_at_location")
    }

    fn get_type_from_type_node(&self, _node: TypeNode) -> Type {
        todo!("get_type_from_type_node")
    }

    fn signature_to_string(&self, _signature: Signature, _enclosing_declaration: Option<AstKind>, _flags: Option<TypeFormatFlags>, _kind: Option<SignatureKind>) -> String {
        todo!("signature_to_string")
    }

    fn type_to_string(&self, _type_: Type, _enclosing_declaration: Option<AstKind>, _flags: Option<TypeFormatFlags>) -> String {
        todo!("type_to_string")
    }

    fn symbol_to_string(&self, _symbol: Symbol, _enclosing_declaration: Option<AstKind>, _meaning: Option<SymbolFlags>, _flags: Option<SymbolFormatFlags>) -> String {
        todo!("symbol_to_string")
    }

    fn type_predicate_to_string(&self, _predicate: TypePredicate, _enclosing_declaration: Option<AstKind>, _flags: Option<TypeFormatFlags>) -> String {
        todo!("type_predicate_to_string")
    }

    fn write_signature(&self, _signature: Signature, _enclosing_declaration: Option<AstKind>, _flags: Option<TypeFormatFlags>, _kind: Option<SignatureKind>, _writer: Option<EmitTextWriter>) -> String {
        todo!("write_signature")
    }

    fn write_type(&self, _type_: Type, _enclosing_declaration: Option<AstKind>, _flags: Option<TypeFormatFlags>, _writer: Option<EmitTextWriter>) -> String {
        todo!("write_type")
    }

    fn write_symbol(&self, _symbol: Symbol, _enclosing_declaration: Option<AstKind>, _meaning: Option<SymbolFlags>, _flags: Option<SymbolFormatFlags>, _writer: Option<EmitTextWriter>) -> String {
        todo!("write_symbol")
    }

    fn write_type_predicate(&self, _predicate: TypePredicate, _enclosing_declaration: Option<AstKind>, _flags: Option<TypeFormatFlags>, _writer: Option<EmitTextWriter>) -> String {
        todo!("write_type_predicate")
    }

    fn get_fully_qualified_name(&self, _symbol: Symbol) -> String {
        todo!("get_fully_qualified_name")
    }

    fn get_augmented_properties_of_type(&self, _type_: Type) -> Vec<Symbol> {
        todo!("get_augmented_properties_of_type")
    }

    fn get_root_symbols(&self, _symbol: Symbol) -> Vec<Symbol> {
        todo!("get_root_symbols")
    }

    fn get_symbol_of_expando(&self, _node: AstKind, _allow_declaration: bool) -> Option<Symbol> {
        todo!("get_symbol_of_expando")
    }

    fn get_contextual_type(&self, _node: Expression) -> Option<Type> {
        todo!("get_contextual_type")
    }

    fn get_contextual_type_with_flags(&self, _node: Expression, _context_flags: Option<ContextFlags>) -> Option<Type> {
        todo!("get_contextual_type_with_flags")
    }

    fn get_contextual_type_for_object_literal_element(&self, _element: ObjectLiteralElementLike) -> Option<Type> {
        todo!("get_contextual_type_for_object_literal_element")
    }

    fn get_contextual_type_for_argument_at_index(&self, _call: CallLikeExpression, _arg_index: usize) -> Option<Type> {
        todo!("get_contextual_type_for_argument_at_index")
    }

    fn get_contextual_type_for_jsx_attribute(&self, _attribute: JsxAttribute) -> Option<Type> {
        todo!("get_contextual_type_for_jsx_attribute")
    }

    fn is_context_sensitive(&self, _node: Expression) -> bool {
        todo!("is_context_sensitive")
    }

    fn get_type_of_property_of_contextual_type(&self, _type_: Type, _name: &str) -> Option<Type> {
        todo!("get_type_of_property_of_contextual_type")
    }

    fn get_resolved_signature(&self, _node: CallLikeExpression, _candidates_out_array: Option<Vec<Signature>>, _argument_count: Option<usize>) -> Option<Signature> {
        todo!("get_resolved_signature")
    }

    fn get_resolved_signature_for_signature_help(&self, _node: CallLikeExpression, _candidates_out_array: Option<Vec<Signature>>, _argument_count: Option<usize>) -> Option<Signature> {
        todo!("get_resolved_signature_for_signature_help")
    }

    fn get_candidate_signatures_for_string_literal_completions(&self, _call: CallLikeExpression, _editing_argument: AstKind) -> Vec<Signature> {
        todo!("get_candidate_signatures_for_string_literal_completions")
    }

    fn get_expanded_parameters(&self, _sig: Signature) -> Vec<Vec<Symbol>> {
        todo!("get_expanded_parameters")
    }

    fn has_effective_rest_parameter(&self, _sig: Signature) -> bool {
        todo!("has_effective_rest_parameter")
    }

    fn contains_arguments_reference(&self, _declaration: SignatureDeclaration) -> bool {
        todo!("contains_arguments_reference")
    }

    fn get_signature_from_declaration(&self, _declaration: SignatureDeclaration) -> Option<Signature> {
        todo!("get_signature_from_declaration")
    }

    fn is_implementation_of_overload(&self, _node: SignatureDeclaration) -> Option<bool> {
        todo!("is_implementation_of_overload")
    }

    fn is_undefined_symbol(&self, _symbol: Symbol) -> bool {
        todo!("is_undefined_symbol")
    }

    fn is_arguments_symbol(&self, _symbol: Symbol) -> bool {
        todo!("is_arguments_symbol")
    }

    fn is_unknown_symbol(&self, _symbol: Symbol) -> bool {
        todo!("is_unknown_symbol")
    }

    fn get_merged_symbol(&self, _symbol: Symbol) -> Symbol {
        todo!("get_merged_symbol")
    }

    fn symbol_is_value(&self, _symbol: Symbol, _include_type_only_members: Option<bool>) -> bool {
        todo!("symbol_is_value")
    }

    fn get_constant_value(&self, _node: EnumMember) -> Option<String> {
        todo!("get_constant_value")
    }

    fn is_valid_property_access(&self, _node: PropertyAccessExpression, _property_name: &str) -> bool {
        todo!("is_valid_property_access")
    }

    fn is_valid_property_access_for_completions(&self, _node: PropertyAccessExpression, _type_: Type, _property: Symbol) -> bool {
        todo!("is_valid_property_access_for_completions")
    }

    fn get_aliased_symbol(&self, _symbol: Symbol) -> Symbol {
        todo!("get_aliased_symbol")
    }

    fn get_immediate_aliased_symbol(&self, _symbol: Symbol) -> Option<Symbol> {
        todo!("get_immediate_aliased_symbol")
    }

    fn get_exports_of_module(&self, _module_symbol: Symbol) -> Vec<Symbol> {
        todo!("get_exports_of_module")
    }

    fn get_exports_and_properties_of_module(&self, _module_symbol: Symbol) -> Vec<Symbol> {
        todo!("get_exports_and_properties_of_module")
    }

    fn get_jsx_intrinsic_tag_names_at(&self, _location: AstKind) -> Vec<Symbol> {
        todo!("get_jsx_intrinsic_tag_names_at")
    }

    fn is_optional_parameter(&self, _node: Argument) -> bool {
        todo!("is_optional_parameter")
    }

    fn get_ambient_modules(&self) -> Vec<Symbol> {
        todo!("get_ambient_modules")
    }

    fn try_get_member_in_module_exports(&self, _member_name: &str, _module_symbol: Symbol) -> Option<Symbol> {
        todo!("try_get_member_in_module_exports")
    }

    fn try_get_member_in_module_exports_and_properties(&self, _member_name: &str, _module_symbol: Symbol) -> Option<Symbol> {
        todo!("try_get_member_in_module_exports_and_properties")
    }

    fn get_apparent_type(&self, _type_: Type) -> Type {
        todo!("get_apparent_type")
    }

    fn get_suggested_symbol_for_nonexistent_property(&self, _name: MemberName, _containing_type: Type) -> Option<Symbol> {
        todo!("get_suggested_symbol_for_nonexistent_property")
    }

    fn get_suggested_symbol_for_nonexistent_jsx_attribute(&self, _name: Identifier, _containing_type: Type) -> Option<Symbol> {
        todo!("get_suggested_symbol_for_nonexistent_jsx_attribute")
    }

    fn get_suggested_symbol_for_nonexistent_symbol(&self, _location: AstKind, _name: &str, _meaning: SymbolFlags) -> Option<Symbol> {
        todo!("get_suggested_symbol_for_nonexistent_symbol")
    }

    fn get_suggested_symbol_for_nonexistent_module(&self, _node: Identifier, _target: Symbol) -> Option<Symbol> {
        todo!("get_suggested_symbol_for_nonexistent_module")
    }

    fn get_suggested_symbol_for_nonexistent_class_member(&self, _name: &str, _base_type: Type) -> Option<Symbol> {
        todo!("get_suggested_symbol_for_nonexistent_class_member")
    }

    fn get_base_constraint_of_type(&self, _type_: Type) -> Option<Type> {
        todo!("get_base_constraint_of_type")
    }

    fn get_default_from_type_parameter(&self, _type_: Type) -> Option<Type> {
        todo!("get_default_from_type_parameter")
    }

    fn get_any_type(&self) -> Type {
        todo!("get_any_type")
    }

    fn get_string_type(&self) -> Type {
        todo!("get_string_type")
    }

    fn get_string_literal_type(&self, _value: &str) -> StringLiteralType {
        todo!("get_string_literal_type")
    }

    fn get_number_type(&self) -> Type {
        todo!("get_number_type")
    }

    fn get_number_literal_type(&self, _value: f64) -> NumberLiteralType {
        todo!("get_number_literal_type")
    }

    fn get_big_int_type(&self) -> Type {
        todo!("get_big_int_type")
    }

    fn get_big_int_literal_type(&self, _value: PseudoBigInt) -> BigIntLiteralType {
        todo!("get_big_int_literal_type")
    }

    fn get_boolean_type(&self) -> Type {
        todo!("get_boolean_type")
    }

    fn get_false_type(&self, _fresh: Option<bool>) -> Type {
        todo!("get_false_type")
    }

    fn get_true_type(&self, _fresh: Option<bool>) -> Type {
        todo!("get_true_type")
    }

    fn get_void_type(&self) -> Type {
        todo!("get_void_type")
    }

    fn get_undefined_type(&self) -> Type {
        todo!("get_undefined_type")
    }

    fn get_null_type(&self) -> Type {
        todo!("get_null_type")
    }

    fn get_es_symbol_type(&self) -> Type {
        todo!("get_es_symbol_type")
    }

    fn get_never_type(&self) -> Type {
        todo!("get_never_type")
    }

    fn get_optional_type(&self) -> Type {
        todo!("get_optional_type")
    }

    fn get_union_type(&self, _types: Vec<Type>, _subtype_reduction: Option<UnionReduction>) -> Type {
        todo!("get_union_type")
    }

    fn create_array_type(&self, _element_type: Type) -> Type {
        todo!("create_array_type")
    }

    fn get_element_type_of_array_type(&self, _array_type: Type) -> Option<Type> {
        todo!("get_element_type_of_array_type")
    }

    fn create_promise_type(&self, _type_: Type) -> Type {
        todo!("create_promise_type")
    }

    fn get_promise_type(&self) -> Type {
        todo!("get_promise_type")
    }

    fn get_promise_like_type(&self) -> Type {
        todo!("get_promise_like_type")
    }

    fn get_any_async_iterable_type(&self) -> Option<Type> {
        todo!("get_any_async_iterable_type")
    }

    fn is_type_assignable_to(&self, _source: Type, _target: Type) -> bool {
        todo!("is_type_assignable_to")
    }

    fn create_anonymous_type(&self, _symbol: Option<Symbol>, _members: SymbolTable, _call_signatures: Vec<Signature>, _construct_signatures: Vec<Signature>, _index_infos: Vec<IndexInfo>) -> Type {
        todo!("create_anonymous_type")
    }

    fn create_signature(
        &self, _declaration: Option<SignatureDeclaration>, _type_parameters: Option<Vec<TypeParameter>>, _this_parameter: Option<Symbol>, _parameters: Vec<Symbol>, _resolved_return_type: Type, _type_predicate: Option<TypePredicate>, _min_argument_count: usize, _flags: SignatureFlags,
    ) -> Signature {
        todo!("create_signature")
    }

    fn create_symbol(&self, _flags: SymbolFlags, _name: &str) -> TransientSymbol {
        todo!("create_symbol")
    }

    fn create_index_info(&self, _key_type: Type, _type_: Type, _is_readonly: bool, _declaration: Option<SignatureDeclaration>) -> IndexInfo {
        todo!("create_index_info")
    }

    fn is_symbol_accessible(&self, _symbol: Symbol, _enclosing_declaration: Option<AstKind>, _meaning: SymbolFlags, _should_compute_alias_to_mark_visible: bool) -> SymbolAccessibilityResult {
        todo!("is_symbol_accessible")
    }

    fn try_find_ambient_module(&self, _module_name: &str) -> Option<Symbol> {
        todo!("try_find_ambient_module")
    }

    fn get_symbol_walker(&self, _accept: Option<fn(Symbol) -> bool>) -> SymbolWalker {
        todo!("get_symbol_walker")
    }

    fn get_diagnostics(&self, _source_file: Option<SourceFile>, _cancellation_token: Option<CancellationToken>, _nodes_to_check: Option<Vec<AstKind>>) -> Vec<Diagnostic> {
        todo!("get_diagnostics")
    }

    fn get_global_diagnostics(&self) -> Vec<Diagnostic> {
        todo!("get_global_diagnostics")
    }

    fn get_emit_resolver(&self, _source_file: Option<SourceFile>, _cancellation_token: Option<CancellationToken>, _force_dts: Option<bool>) -> EmitResolver {
        todo!("get_emit_resolver")
    }

    fn requires_adding_implicit_undefined(&self, _parameter: Argument, _enclosing_declaration: Option<AstKind>) -> bool {
        todo!("requires_adding_implicit_undefined")
    }

    fn get_node_count(&self) -> usize {
        todo!("get_node_count")
    }

    fn get_identifier_count(&self) -> usize {
        todo!("get_identifier_count")
    }

    fn get_symbol_count(&self) -> usize {
        todo!("get_symbol_count")
    }

    fn get_type_count(&self) -> usize {
        todo!("get_type_count")
    }

    fn get_instantiation_count(&self) -> usize {
        todo!("get_instantiation_count")
    }

    fn get_relation_cache_sizes(&self) -> (usize, usize, usize, usize) {
        todo!("get_relation_cache_sizes")
    }

    fn get_recursion_identity(&self, _type_: Type) -> Option<Type> {
        todo!("get_recursion_identity")
    }

    fn get_unmatched_properties(&self, _source: Type, _target: Type, _require_optional_properties: bool, _match_discriminant_properties: bool) -> Box<dyn Iterator<Item = Symbol>> {
        todo!("get_unmatched_properties")
    }

    fn is_array_type(&self, _type_: Type) -> bool {
        todo!("is_array_type")
    }

    fn is_tuple_type(&self, _type_: Type) -> bool {
        todo!("is_tuple_type")
    }

    fn is_array_like_type(&self, _type_: Type) -> bool {
        todo!("is_array_like_type")
    }

    fn is_type_invalid_due_to_union_discriminant(&self, _contextual_type: Type, _obj: ObjectExpression) -> bool {
        todo!("is_type_invalid_due_to_union_discriminant")
    }

    fn get_exact_optional_properties(&self, _type_: Type) -> Vec<Symbol> {
        todo!("get_exact_optional_properties")
    }

    fn get_all_possible_properties_of_types(&self, _types: Vec<Type>) -> Vec<Symbol> {
        todo!("get_all_possible_properties_of_types")
    }

    fn resolve_name(&self, _name: &str, _location: Option<AstKind>, _meaning: SymbolFlags, _exclude_globals: bool) -> Option<Symbol> {
        todo!("resolve_name")
    }

    fn get_jsx_namespace(&self, _location: Option<AstKind>) -> String {
        todo!("get_jsx_namespace")
    }

    fn get_jsx_fragment_factory(&self, _location: AstKind) -> Option<String> {
        todo!("get_jsx_fragment_factory")
    }

    fn get_accessible_symbol_chain(&self, _symbol: Symbol, _enclosing_declaration: Option<AstKind>, _meaning: SymbolFlags, _use_only_external_aliasing: bool) -> Option<Vec<Symbol>> {
        todo!("get_accessible_symbol_chain")
    }

    fn get_type_predicate_of_signature(&self, _signature: Signature) -> Option<TypePredicate> {
        todo!("get_type_predicate_of_signature")
    }

    fn resolve_external_module_name(&self, _module_specifier: Expression) -> Option<Symbol> {
        todo!("resolve_external_module_name")
    }

    fn resolve_external_module_symbol(&self, _symbol: Symbol) -> Symbol {
        todo!("resolve_external_module_symbol")
    }

    fn try_get_this_type_at(&self, _node: AstKind, _include_global_this: Option<bool>, _container: Option<ThisContainer>) -> Option<Type> {
        todo!("try_get_this_type_at")
    }

    fn get_type_argument_constraint(&self, _node: TypeNode) -> Option<Type> {
        todo!("get_type_argument_constraint")
    }

    fn get_suggestion_diagnostics(&self, _file: SourceFile, _cancellation_token: Option<CancellationToken>) -> Vec<DiagnosticWithLocation> {
        todo!("get_suggestion_diagnostics")
    }

    fn get_local_type_parameters_of_class_or_interface_or_type_alias(&self, _symbol: Symbol) -> Option<Vec<TypeParameter>> {
        todo!("get_local_type_parameters_of_class_or_interface_or_type_alias")
    }

    fn is_declaration_visible(&self, _node: Declaration) -> bool {
        todo!("is_declaration_visible")
    }

    fn is_property_accessible(&self, _node: AstKind, _is_super: bool, _is_write: bool, _containing_type: Type, _property: Symbol) -> bool {
        todo!("is_property_accessible")
    }

    fn get_type_only_alias_declaration(&self, _symbol: Symbol) -> Option<TypeOnlyAliasDeclaration> {
        todo!("get_type_only_alias_declaration")
    }

    fn get_member_override_modifier_status(&self, _node: ClassLikeDeclaration, _member: ClassElement, _member_symbol: Symbol) -> MemberOverrideStatus {
        todo!("get_member_override_modifier_status")
    }

    fn is_type_parameter_possibly_referenced(&self, _tp: TypeParameter, _node: AstKind) -> bool {
        todo!("is_type_parameter_possibly_referenced")
    }

    fn type_has_call_or_construct_signatures(&self, _type_: Type) -> bool {
        todo!("type_has_call_or_construct_signatures")
    }

    fn get_symbol_flags(&self, _symbol: Symbol) -> SymbolFlags {
        todo!("get_symbol_flags")
    }
}
