#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(unused_doc_comments)]
mod compiler;
mod services;
use std::mem;
use std::rc::Rc;
use std::{fs, path::Path};

use compiler::checker::TypeChecker;
use compiler::parser::createSourceFile;
use compiler::rb_extra::{cleanup_SOURCEFILE_INFO_MAP, AstKindExt, RB_CTX};
use compiler::rb_host::RbTypeCheckerHost;
use compiler::types::{CompilerOptions, TypeCheckerTrait};
use oxc_ast::ast::{SourceFile, Statement};
use oxc_ast::{AstKind, GetChildren};

fn fill_parents(source_file: &mut Box<SourceFile>) {
    fn dfs(node: AstKind) {
        let children = node.get_children();
        for child in children {
            child.set_parent(Some(unsafe { mem::transmute(node) }));
            dfs(child);
        }
    }

    dfs(AstKind::SourceFile(source_file.as_ref()));
}

fn main() {
    // let diag = Diagnostics::Unterminated_string_literal();
    // println!("Testing diagnostic message:");
    // println!("Code: {}", diag.code);
    // println!("Category: {:?}", diag.category);
    // println!("Key: {}", diag.key);
    // println!("Message: {}", diag.message);

    let cwd = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let type_checker_host = Rc::new(RbTypeCheckerHost::new(cwd, CompilerOptions::default()));
    RB_CTX.set_type_checker_host(type_checker_host.clone());

    let name = "src/snips/module_declaration.ts";
    let path = Path::new(&name);
    let source_text = fs::read_to_string(path).unwrap();
    // let source_type = SourceType::from_path(path).unwrap();

    // let allocator = Allocator::default();
    // let ret = Parser::new(&allocator, &source_text, source_type).with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() }).parse();
    // let source_file = ret.source_file;
    // source_file.set_filepath(path.to_path_buf());
    // source_file.set_package_json_scope(None); // todo
    let mut source_file = Box::new(createSourceFile(name, &source_text));
    let type_checker = TypeChecker::new(type_checker_host);
    fill_parents(&mut source_file);

    println!("AST:");
    println!("{source_file:#?}");

    if let Statement::VariableDeclaration(node) = &source_file.body[0] {
        println!("Node: {:?}", node);
        println!("Parent: {:?}", node.to_ast_kind().parent());
        let declarator = &node.declarations[0];
        let id = declarator.id.kind.get_binding_identifier().unwrap();
        println!("Id: {:?}", id);

        let tc = type_checker.borrow();
        let type_ = tc.getTypeAtLocation(declarator.to_ast_kind());
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

    cleanup_SOURCEFILE_INFO_MAP();
    RB_CTX.cleanup();
}
