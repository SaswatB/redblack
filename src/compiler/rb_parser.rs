use std::mem;

use oxc_ast::{ast::SourceFile, AstKind};

use crate::compiler::rb_extra::AstKindExt;

pub fn rb_fill_parents(source_file: &mut SourceFile) {
    // todo(RB): fill in NodeFlags
    fn dfs(node: AstKind) {
        let children = node.get_children();
        for child in children {
            child.set_parent(Some(unsafe { mem::transmute(node) }));
            dfs(child);
        }
    }

    dfs(AstKind::SourceFile(source_file));
}
