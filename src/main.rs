#![allow(dead_code)]
#[allow(non_snake_case)] 
mod compiler;

use crate::compiler::diagnostic_information_map_generated::Diagnostics;

fn main() {
    let diag = Diagnostics::Unterminated_string_literal();
    println!("Testing diagnostic message:");
    println!("Code: {}", diag.code);
    println!("Category: {:?}", diag.category);
    println!("Key: {}", diag.key);
    println!("Message: {}", diag.message);
}
