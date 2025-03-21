use std::{cell::RefCell, rc::Rc};

use crate::{compiler::types::*, new_rc_cell, opt_rc_cell};

impl<'a> Symbol<'a> {
    pub fn new(flags: SymbolFlags, name: &str) -> Self {
        Self {
            flags,
            escapedName: name.to_owned(),
            declarations: None,
            valueDeclaration: None,
            members: None,
            exports: None,
            globalExports: None,
            id: 0,
            mergeId: 0,
            parent: None,
            exportSymbol: None,
            constEnumOnlyModule: None,
            isReferenced: None,
            lastAssignmentPos: None,
            isReplaceableByMethod: None,
            assignmentDeclarationMembers: None,
        }
    }
}

impl<'a> TypeObject<'a> {
    pub fn new(flags: TypeFlags) -> Self {
        Self {
            flags,
            id: 0,
            // checker,
            symbol: Some(new_rc_cell!(Symbol::new(SymbolFlags::None, ""))),
            pattern: None,
            aliasSymbol: None,
            aliasTypeArguments: None,
            permissiveInstantiation: None,
            restrictiveInstantiation: None,
            immediateBaseConstraint: None,
            widened: None,
            object_flags: None,
            intrinsic_props: None,
            freshable_props: None,
            object_props: None,
            interface_props: None,
        }
    }
}

impl<'a> Type<'a> for TypeObject<'a> {
    fn getFlags(&self) -> TypeFlags { self.flags }
    fn getSymbol(&self) -> opt_rc_cell!(Symbol<'a>) { self.symbol.clone() }
    // fn getProperties(&self) -> Vec<&Symbol> { self.checker.getPropertiesOfType(self) }
    // fn getProperty(&self, propertyName: &str) -> Option<&Symbol> { self.checker.getPropertyOfType(self, propertyName) }
    // fn getApparentProperties(&self) -> Vec<&Symbol> { self.checker.getAugmentedPropertiesOfType(self) }
    // fn getCallSignatures(&self) -> Vec<&Signature> { self.checker.getSignaturesOfType(self, SignatureKind::Call) }
    // fn getConstructSignatures(&self) -> Vec<&Signature> { self.checker.getSignaturesOfType(self, SignatureKind::Construct) }
    // fn getStringIndexType(&self) -> Option<&dyn Type> { self.checker.getIndexTypeOfType(self, IndexKind::String) }
    // fn getNumberIndexType(&self) -> Option<&dyn Type> { self.checker.getIndexTypeOfType(self, IndexKind::Number) }
    // fn getBaseTypes(&self) -> Option<Vec<BaseType>> {
    //     if self.isClassOrInterface() {
    //         Some(self.checker.getBaseTypes(self))
    //     } else {
    //         None
    //     }
    // }
    // fn getNonNullableType(&self) -> &dyn Type { self.checker.getNonNullableType(self) }
    // fn getNonOptionalType(&self) -> &dyn Type { self.checker.getNonOptionalType(self) }
    // fn isNullableType(&self) -> bool { self.checker.isNullableType(self) }
    // fn getConstraint(&self) -> Option<&dyn Type> { self.checker.getBaseConstraintOfType(self) }
    // fn getDefault(&self) -> Option<&dyn Type> { self.checker.getDefaultFromTypeParameter(self) }
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

    fn as_type(&self) -> &dyn Type<'a> { self }
}

impl<'a> IntrinsicType<'a> for TypeObject<'a> {
    fn get_intrinsic_props(&self) -> &IntrinsicTypeProps { self.intrinsic_props.as_ref().unwrap() }
}

impl<'a> NullableType<'a> for TypeObject<'a> {}

impl<'a> FreshableType<'a> for TypeObject<'a> {
    fn get_freshable_type_props(&self) -> &FreshableTypeProps<'a> { self.freshable_props.as_ref().unwrap() }
}

impl<'a> FreshableIntrinsicType<'a> for TypeObject<'a> {}

impl<'a> ObjectFlagsTrait<'a> for TypeObject<'a> {
    fn get_object_flags(&self) -> ObjectFlags { self.object_flags.unwrap_or(ObjectFlags::None) }
}

impl<'a> ObjectType<'a> for TypeObject<'a> {
    fn get_object_props(&self) -> &ObjectTypeProps<'a> { self.object_props.as_ref().unwrap() }
}

impl<'a> InterfaceType<'a> for TypeObject<'a> {
    fn get_interface_props(&self) -> &InterfaceTypeProps<'a> { self.interface_props.as_ref().unwrap() }
}
