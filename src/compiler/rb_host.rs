use oxc_ast::ast::SourceFile;
use std::fs;
use std::path::Path;

use super::types::{CompilerOptions, ModuleResolutionHost, ModuleSpecifierResolutionHost, TypeCheckerHost};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RbTypeCheckerHost {
    pub current_directory: String,
    pub compiler_options: CompilerOptions,
}

impl RbTypeCheckerHost {
    pub fn new(current_directory: String, compiler_options: CompilerOptions) -> Self { Self { current_directory, compiler_options } }
}

impl ModuleResolutionHost for RbTypeCheckerHost {
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
impl ModuleSpecifierResolutionHost for RbTypeCheckerHost {
    fn getPackageJsonInfoCache(&self) -> Option<&dyn super::moduleNameResolver::PackageJsonInfoCache> { todo!() }

    fn getGlobalTypingsCacheLocation(&self) -> Option<String> { todo!() }

    fn getNearestAncestorDirectoryWithPackageJson(&self, file_name: &str, root_dir: Option<&str>) -> Option<String> { todo!() }

    fn getProjectReferenceRedirect(&self, file_name: &str) -> Option<String> { todo!() }

    fn isSourceOfProjectReferenceRedirect(&self, file_name: &str) -> bool { todo!() }

    fn getCommonSourceDirectory(&self) -> String { todo!() }
}

#[allow(unused_variables)]
impl TypeCheckerHost for RbTypeCheckerHost {
    fn getCompilerOptions(&self) -> &CompilerOptions { &self.compiler_options }

    fn getSourceFiles(&self) -> Vec<&SourceFile> { todo!() }

    fn getSourceFile(&self, file_name: &str) -> Option<&SourceFile> { todo!() }

    fn getProjectReferenceRedirect(&self, file_name: &str) -> Option<String> { todo!() }

    fn isSourceOfProjectReferenceRedirect(&self, file_name: &str) -> bool { todo!() }

    fn typesPackageExists(&self, package_name: &str) -> bool { todo!() }

    fn packageBundlesTypes(&self, package_name: &str) -> bool { todo!() }
}
