#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(unused_doc_comments)]
mod compiler;
mod services;
use std::cell::UnsafeCell;
use std::rc::Rc;

use compiler::checker::TypeChecker;
use compiler::rb_extra::{cleanup_SOURCEFILE_INFO_MAP, AstKindExt, RB_CTX};
use compiler::rb_host::RbTypeCheckerHost;
use compiler::types::{CompilerOptions, TypeCheckerTrait};
use oxc_ast::ast::{SourceFile, Statement};
use oxc_ast::{AstKind, GetChildren};

fn main() {
    let cwd = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let type_checker_host = Rc::new(UnsafeCell::new(RbTypeCheckerHost::new(cwd, Rc::new(CompilerOptions::default()))));
    RB_CTX.set_type_checker_host(type_checker_host.clone());

    let name = "src/snips/identifier.ts";
    let source_file = unsafe { (&mut *type_checker_host.as_ref().get()).addSourceFile(name.to_string()) };

    let type_checker = TypeChecker::new(type_checker_host);

    println!("AST:");
    println!("{source_file:#?}");

    if let Statement::VariableDeclarationList(node) = &source_file.borrow().body[0] {
        println!("Node: {:?}", node);
        println!("Parent: {:?}", node.to_ast_kind().parent());
        let declarator = &node.declarations[0];
        let id = declarator.id.kind.get_binding_identifier().unwrap();
        println!("Id: {:?}", id);

        let tc = type_checker.borrow();
        let type_ = tc.getTypeAtLocation(declarator.to_ast_kind());
        println!("Type: {:?}", type_);
    }

    cleanup_SOURCEFILE_INFO_MAP();
    RB_CTX.cleanup();
}
