use super::parser::CreateSourceFileOptions;
use super::rb_extra::ProgramExt;
use crate::compiler::moduleNameResolver::*;
use crate::compiler::path::*;
use crate::compiler::types::*;
use crate::compiler::utilities::*;
use oxc::ast::ast::Program;

// region: 1372
/**
 * A function for determining if a given file is esm or cjs format, assuming modern node module resolution rules, as configured by the
 * `options` parameter.
 *
 * @param fileName The file name to check the format of (it need not exist on disk)
 * @param [packageJsonInfoCache] A cache for package file lookups - it's best to have a cache when this function is called often
 * @param host The ModuleResolutionHost which can perform the filesystem lookups for package json data
 * @param options The compiler options to perform the analysis under - relevant options are `moduleResolution` and `traceResolution`
 * @returns `undefined` if the path has no relevant implied format, `ModuleKind.ESNext` for esm format, and `ModuleKind.CommonJS` for cjs format
 */
pub fn getImpliedNodeFormatForFile(file_name: &str, host: &dyn ModuleResolutionHost, options: &CompilerOptions) -> ResolutionMode {
    let result = getImpliedNodeFormatForFileWorker(file_name, host, options);
    match result {
        Some(options) => options.impliedNodeFormat.unwrap_or(ResolutionMode::Undefined),
        None => ResolutionMode::Undefined,
    }
}

/** @internal */
pub fn getImpliedNodeFormatForFileWorker(file_name: &str, host: &dyn ModuleResolutionHost, options: &CompilerOptions) -> Option<CreateSourceFileOptions> {
    let module_resolution = getEmitModuleResolutionKind(options);
    let should_lookup_from_package_json = ModuleResolutionKind::Node16 <= module_resolution && module_resolution <= ModuleResolutionKind::NodeNext || pathContainsNodeModules(file_name);

    if fileExtensionIsOneOf(file_name, vec![Extension::Dmts.as_str(), Extension::Mts.as_str(), Extension::Mjs.as_str()]) {
        return Some(CreateSourceFileOptions { impliedNodeFormat: Some(ResolutionMode::ESNext), ..Default::default() });
    }

    if fileExtensionIsOneOf(file_name, vec![Extension::Dcts.as_str(), Extension::Cts.as_str(), Extension::Cjs.as_str()]) {
        return Some(CreateSourceFileOptions { impliedNodeFormat: Some(ResolutionMode::CommonJS), ..Default::default() });
    }

    if should_lookup_from_package_json && fileExtensionIsOneOf(file_name, vec![Extension::Dts.as_str(), Extension::Ts.as_str(), Extension::Tsx.as_str(), Extension::Js.as_str(), Extension::Jsx.as_str()]) {
        return lookupFromPackageJson(file_name, host, options);
    }

    // other extensions, like `json` or `tsbuildinfo`, are set as `None` here but they should never be fed through the transformer pipeline
    None
}

fn lookupFromPackageJson(file_name: &str, host: &dyn ModuleResolutionHost, options: &CompilerOptions) -> Option<CreateSourceFileOptions> {
    let mut state = getTemporaryModuleResolutionState(None, host, options); // todo consider a thread local cache
    let package_json_locations: Vec<String> = Vec::new();
    state.failedLookupLocations = Some(package_json_locations.clone());
    state.affectingLocations = Some(package_json_locations.clone());
    let package_json_scope = getPackageScopeForPath(&getDirectoryPath(file_name), &mut state);

    let implied_node_format = if package_json_scope.as_ref().and_then(|scope| scope.contents.packageJsonContent.r#type.as_ref()).map_or(false, |t| t == "module") { ResolutionMode::ESNext } else { ResolutionMode::CommonJS };

    Some(CreateSourceFileOptions { languageVersion: ScriptTarget::ESNext, impliedNodeFormat: Some(implied_node_format), packageJsonLocations: Some(package_json_locations), packageJsonScope: package_json_scope })
}
// endregion: 1397

// region: 5190
/** @internal Prefer `program.getImpliedNodeFormatForEmit` when possible. */
pub fn getImpliedNodeFormatForEmitWorker(sourceFile: &Program, options: &CompilerOptions) -> ResolutionMode {
    let moduleKind = getEmitModuleKind(options);
    if ModuleKind::Node16 <= moduleKind && moduleKind <= ModuleKind::NodeNext {
        return sourceFile.implied_node_format();
    }
    let packageJsonContentType = sourceFile.package_json_scope().map(|scope| scope.contents.packageJsonContent.r#type).flatten();
    if sourceFile.implied_node_format() == ResolutionMode::CommonJS && (packageJsonContentType.as_deref() == Some("commonjs") || fileExtensionIsOneOf(&sourceFile.filepath().to_string_lossy(), [Extension::Cjs.as_str(), Extension::Cts.as_str()].to_vec())) {
        return ResolutionMode::CommonJS;
    }
    if sourceFile.implied_node_format() == ResolutionMode::ESNext && (packageJsonContentType.as_deref() == Some("module") || fileExtensionIsOneOf(&sourceFile.filepath().to_string_lossy(), [Extension::Mjs.as_str(), Extension::Mts.as_str()].to_vec())) {
        return ResolutionMode::ESNext;
    }
    return ResolutionMode::Undefined;
}
// endregion: 5211
