use oxc_ast::{
    ast::{BindingPattern, BindingRestElement},
    AstKind,
};

use crate::define_subset_enum;

use super::types::{EntityNameExpression, IsContainer};

define_subset_enum!(IsContainerOrEntityNameExpression from AstKind {
    Sub(IsContainer),
    Sub(EntityNameExpression),
});

define_subset_enum!(BindingElement from AstKind {
    BindingPattern,
    BindingRestElement,
});
