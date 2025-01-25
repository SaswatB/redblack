use crate::compiler::types::*;

impl<'a> Type for TypeObject<'a> {
    fn getFlags(&self) -> TypeFlags { self.flags }
    fn getSymbol(&self) -> Option<&Symbol> { Some(&self.symbol) }
    fn getProperties(&self) -> Vec<&Symbol> { self.checker.getPropertiesOfType(self) }
    fn getProperty(&self, propertyName: &str) -> Option<&Symbol> { self.checker.getPropertyOfType(self, propertyName) }
    fn getApparentProperties(&self) -> Vec<&Symbol> { self.checker.getAugmentedPropertiesOfType(self) }
    fn getCallSignatures(&self) -> Vec<&Signature> { self.checker.getSignaturesOfType(self, SignatureKind::Call) }
    fn getConstructSignatures(&self) -> Vec<&Signature> { self.checker.getSignaturesOfType(self, SignatureKind::Construct) }
    fn getStringIndexType(&self) -> Option<&dyn Type> { self.checker.getIndexTypeOfType(self, IndexKind::String) }
    fn getNumberIndexType(&self) -> Option<&dyn Type> { self.checker.getIndexTypeOfType(self, IndexKind::Number) }
    fn getBaseTypes(&self) -> Option<Vec<BaseType>> {
        if self.isClassOrInterface() {
            Some(self.checker.getBaseTypes(self))
        } else {
            None
        }
    }
    fn getNonNullableType(&self) -> &dyn Type { self.checker.getNonNullableType(self) }
    fn getNonOptionalType(&self) -> &dyn Type { self.checker.getNonOptionalType(self) }
    fn isNullableType(&self) -> bool { self.checker.isNullableType(self) }
    fn getConstraint(&self) -> Option<&dyn Type> { self.checker.getBaseConstraintOfType(self) }
    fn getDefault(&self) -> Option<&dyn Type> { self.checker.getDefaultFromTypeParameter(self) }
    fn isUnion(&self) -> bool { self.flags.contains(TypeFlags::Union) }
    fn isIntersection(&self) -> bool { self.flags.contains(TypeFlags::Intersection) }
    fn isUnionOrIntersection(&self) -> bool { self.flags.contains(TypeFlags::UnionOrIntersection) }
    fn isLiteral(&self) -> bool { self.flags.intersects(TypeFlags::StringLiteral | TypeFlags::NumberLiteral | TypeFlags::BigIntLiteral) }
    fn isStringLiteral(&self) -> bool { self.flags.contains(TypeFlags::StringLiteral) }
    fn isNumberLiteral(&self) -> bool { self.flags.contains(TypeFlags::NumberLiteral) }
    fn isTypeParameter(&self) -> bool { self.flags.contains(TypeFlags::TypeParameter) }
    fn isClassOrInterface(&self) -> bool { self.get_object_flags().contains(ObjectFlags::ClassOrInterface) }
    fn isClass(&self) -> bool { self.get_object_flags().contains(ObjectFlags::Class) }
    fn isIndexType(&self) -> bool { self.flags.contains(TypeFlags::Index) }
}

impl<'a> ObjectFlagsTrait for TypeObject<'a> {
    fn get_object_flags(&self) -> ObjectFlags { self.object_flags.unwrap_or(ObjectFlags::None) }
}

impl<'a> ObjectType for TypeObject<'a> {
    fn get_object_props(&self) -> &ObjectTypeProps { self.object_props.as_ref().unwrap() }
}

impl<'a> InterfaceType for TypeObject<'a> {
    fn get_interface_props(&self) -> &InterfaceTypeProps { self.interface_props.as_ref().unwrap() }
}
