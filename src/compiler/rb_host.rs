use oxc_ast::ast::SourceFile;
use std::path::Path;
use std::{fs, rc::Rc};

use crate::{new_rc_cell, rc_cell};

use super::parser::createSourceFile;
use super::types::{CompilerOptions, ModuleResolutionHost, ModuleSpecifierResolutionHost, TypeCheckerHost};

#[derive(Debug, Clone)]
pub struct RbTypeCheckerHost<'a> {
    pub current_directory: String,
    pub compiler_options: Rc<CompilerOptions>,
    pub source_files: rc_cell!(Vec<rc_cell!(SourceFile<'a>)>),

    string_arena: Vec<Box<str>>, // allocator for strings
}

impl<'a> RbTypeCheckerHost<'a> {
    pub fn new(current_directory: String, compiler_options: Rc<CompilerOptions>) -> Self { Self { current_directory, compiler_options, source_files: new_rc_cell!(vec![]), string_arena: vec![] } }

    // Add a string to the arena and get a reference with lifetime 'a
    fn allocate_str(&mut self, s: String) -> &'a str {
        let boxed = s.into_boxed_str();
        // Safety: we're storing the string in self.string_arena which has the same lifetime
        // as self, so this reference will be valid as long as self is valid
        let str_ref = unsafe { std::mem::transmute::<&str, &'a str>(boxed.as_ref()) };
        self.string_arena.push(boxed);
        str_ref
    }

    pub fn addSourceFile(&mut self, name: String) -> rc_cell!(SourceFile<'a>) {
        let name = self.allocate_str(name);
        let source_text = self.allocate_str(fs::read_to_string(Path::new(name)).unwrap());
        let source_file = new_rc_cell!(createSourceFile(name, source_text));
        self.source_files.borrow_mut().push(source_file.clone());
        source_file
    }
}

impl<'a> ModuleResolutionHost for RbTypeCheckerHost<'a> {
    fn fileExists(&self, fileName: &str) -> bool { Path::new(fileName).exists() }

    fn readFile(&self, fileName: &str) -> Option<String> { fs::read_to_string(fileName).ok() }

    fn trace(&self, _s: &str) -> Option<()> { None }

    fn directoryExists(&self, directoryName: &str) -> Option<bool> { Some(Path::new(directoryName).is_dir()) }

    fn realpath(&self, path: &str) -> Option<String> { fs::canonicalize(path).ok().and_then(|p| p.to_str().map(|s| s.to_string())) }

    fn getCurrentDirectory(&self) -> Option<String> { Some(self.current_directory.clone()) }

    fn getDirectories(&self, path: &str) -> Option<Vec<String>> { fs::read_dir(path).ok().map(|entries| entries.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()).filter_map(|e| e.path().to_str().map(|s| s.to_string())).collect()) }

    fn useCaseSensitiveFileNames(&self) -> Option<bool> {
        #[cfg(target_os = "windows")]
        return Some(false);
        #[cfg(not(target_os = "windows"))]
        return Some(true);
    }
}

#[allow(unused_variables)]
impl<'a> ModuleSpecifierResolutionHost for RbTypeCheckerHost<'a> {
    fn getPackageJsonInfoCache(&self) -> Option<&dyn super::moduleNameResolver::PackageJsonInfoCache> { todo!() }

    fn getGlobalTypingsCacheLocation(&self) -> Option<String> { todo!() }

    fn getNearestAncestorDirectoryWithPackageJson(&self, file_name: &str, root_dir: Option<&str>) -> Option<String> { todo!() }

    fn getProjectReferenceRedirect(&self, file_name: &str) -> Option<String> { todo!() }

    fn isSourceOfProjectReferenceRedirect(&self, file_name: &str) -> bool { todo!() }

    fn getCommonSourceDirectory(&self) -> String { todo!() }
}

#[allow(unused_variables)]
impl<'a> TypeCheckerHost<'a> for RbTypeCheckerHost<'a> {
    fn getCompilerOptions(&self) -> Rc<CompilerOptions> { self.compiler_options.clone() }

    fn getSourceFiles(&self) -> rc_cell!(Vec<rc_cell!(SourceFile<'a>)>) { self.source_files.clone() }

    fn getSourceFile(&self, file_name: &str) -> Option<&SourceFile> { todo!() }

    fn getProjectReferenceRedirect(&self, file_name: &str) -> Option<String> { todo!() }

    fn isSourceOfProjectReferenceRedirect(&self, file_name: &str) -> bool { todo!() }

    fn typesPackageExists(&self, package_name: &str) -> bool { todo!() }

    fn packageBundlesTypes(&self, package_name: &str) -> bool { todo!() }
}
