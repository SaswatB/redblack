use oxc_ast::AstKind;

// region: 434
pub fn isClassStaticBlockDeclaration(node: Option<&AstKind>) -> bool { matches!(node, Some(AstKind::StaticBlock(_))) }
// endregion: 436
