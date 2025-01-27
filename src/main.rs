#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod compiler;
mod services;
use std::sync::Arc;
use std::{fs, path::Path};

use compiler::checker::TypeChecker;
use compiler::parser::createSourceFile;
use compiler::rb_extra::{cleanup_PROGRAM_INFO_MAP, RB_CTX};
use compiler::rb_host::RbTypeCheckerHost;
use compiler::types::{CompilerOptions, TypeCheckerTrait};
use oxc::ast::AstKind;

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

    let name = "src/compiler/types.ts";
    let path = Path::new(&name);
    let source_text = fs::read_to_string(path).unwrap();
    // let source_type = SourceType::from_path(path).unwrap();

    // let allocator = Allocator::default();
    // let ret = Parser::new(&allocator, &source_text, source_type).with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() }).parse();
    // let program = ret.program;
    // program.set_filepath(path.to_path_buf());
    // program.set_package_json_scope(None); // todo
    let program = createSourceFile(name, &source_text);
    let type_checker = TypeChecker::new(type_checker_host);

    let tc = type_checker.borrow();
    let type_ = tc.getTypeAtLocation(AstKind::Program(&program));
    println!("Type: {:?}", type_);

    // println!("AST:");
    // println!("{program:#?}");

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
