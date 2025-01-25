#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod compiler;
use std::{fs, path::Path};

use oxc::allocator::Allocator;
use oxc::parser::{ParseOptions, Parser};
use oxc::span::SourceType;

use crate::compiler::diagnostic_information_map_generated::Diagnostics;

fn main() {
    let diag = Diagnostics::Unterminated_string_literal();
    println!("Testing diagnostic message:");
    println!("Code: {}", diag.code);
    println!("Category: {:?}", diag.category);
    println!("Key: {}", diag.key);
    println!("Message: {}", diag.message);

    let name = "src/compiler/types.ts";
    let path = Path::new(&name);
    let source_text = fs::read_to_string(path).unwrap();
    let source_type = SourceType::from_path(path).unwrap();

    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();
    let mut program = ret.program;

    println!("AST:");
    println!("{program:#?}");

    if ret.errors.is_empty() {
        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
            println!("Parsed with Errors.");
        }
    }
}
