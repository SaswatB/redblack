use oxc_ast::AstKind;

use crate::define_subset_enum;

use super::types::{EntityNameExpression, IsContainer};

define_subset_enum!(IsContainerOrEntityNameExpression from AstKind {
    Sub(IsContainer),
    Sub(EntityNameExpression),
});
