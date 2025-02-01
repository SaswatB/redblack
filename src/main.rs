#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod compiler;
mod services;
use std::mem;
use std::sync::Arc;
use std::{fs, path::Path};

use compiler::checker::TypeChecker;
use compiler::parser::createSourceFile;
use compiler::rb_extra::{cleanup_PROGRAM_INFO_MAP, AstKindExt, RB_CTX};
use compiler::rb_host::RbTypeCheckerHost;
use compiler::types::{CompilerOptions, TypeCheckerTrait};
use oxc_ast::ast::{Program, Statement};
use oxc_ast::{AstKind, GetChildren};

fn fill_parents(program: &mut Box<Program>) {
    fn dfs(node: AstKind) {
        let children = node.get_children();
        for child in children {
            child.set_parent(Some(unsafe { mem::transmute(node) }));
            dfs(child);
        }
    }

    dfs(AstKind::Program(program.as_ref()));
}

fn main() {
    // let diag = Diagnostics::Unterminated_string_literal();
    // println!("Testing diagnostic message:");
    // println!("Code: {}", diag.code);
    // println!("Category: {:?}", diag.category);
    // println!("Key: {}", diag.key);
    // println!("Message: {}", diag.message);

    let cwd = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let type_checker_host = Arc::new(RbTypeCheckerHost::new(cwd, CompilerOptions::default()));
    RB_CTX.set_type_checker_host(type_checker_host.clone());

    let name = "src/compiler/test.ts";
    let path = Path::new(&name);
    let source_text = fs::read_to_string(path).unwrap();
    // let source_type = SourceType::from_path(path).unwrap();

    // let allocator = Allocator::default();
    // let ret = Parser::new(&allocator, &source_text, source_type).with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() }).parse();
    // let program = ret.program;
    // program.set_filepath(path.to_path_buf());
    // program.set_package_json_scope(None); // todo
    let mut program = Box::new(createSourceFile(name, &source_text));
    let type_checker = TypeChecker::new(type_checker_host);
    fill_parents(&mut program);

    println!("AST:");
    println!("{program:#?}");

    if let Statement::VariableDeclaration(node) = &program.body[0] {
        println!("Node: {:?}", node);
        println!("Parent: {:?}", node.to_ast_kind().parent());
        let id = node.declarations[0].id.kind.get_binding_identifier().unwrap();
        println!("Id: {:?}", id);

        let tc = type_checker.borrow();
        let type_ = tc.getTypeAtLocation(id.to_ast_kind());
        println!("Type: {:?}", type_);
    }

    // if ret.errors.is_empty() {
    //     println!("Parsed Successfully.");
    // } else {
    //     for error in ret.errors {
    //         let error = error.with_source_code(source_text.clone());
    //         println!("{error:?}");
    //         println!("Parsed with Errors.");
    //     }
    // }

    cleanup_PROGRAM_INFO_MAP();
    RB_CTX.cleanup();
}
