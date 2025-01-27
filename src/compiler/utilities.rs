use std::fmt::Debug;

use crate::compiler::parser::*;
use crate::compiler::path::*;
use crate::compiler::program::*;
use crate::compiler::types::*;
use oxc::ast::{ast::Program, AstKind, Visit};

use super::rb_extra::ProgramExt;

// region: 8737
/**
 * This is a somewhat unavoidable full tree walk to locate a JSX tag - `import.meta` requires the same,
 * but we avoid that walk (or parts of it) if at all possible using the `PossiblyContainsImportMeta` node flag.
 * Unfortunately, there's no `NodeFlag` space to do the same for JSX.
 */
// walkTreeForJSXTags
#[derive(Debug, Default)]
struct FindJSX {
    has_jsx: bool,
}

impl<'a> Visit<'a> for FindJSX {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        if kind.is_jsx() {
            self.has_jsx = true;
        }
    }
}

pub fn isFileModuleFromUsingJSXTag(file: &Program) -> bool {
    // Excludes declaration files - they still require an explicit `export {}` or the like
    // for back compat purposes. (not that declaration files should contain JSX tags!)
    if !file.source_type.is_typescript_definition() {
        let mut finder = FindJSX { has_jsx: false };
        finder.visit_program(file);
        return finder.has_jsx;
    }
    false
}

/**
* Note that this requires file.impliedNodeFormat be set already; meaning it must be set very early on
* in SourceFile construction.
*/
pub fn isFileForcedToBeModuleByFormat(file: &Program, options: &CompilerOptions) -> bool {
    // Excludes declaration files - they still require an explicit `export {}` or the like
    // for back compat purposes. The only non-declaration files _not_ forced to be a module are `.js` files
    // that aren't esm-mode (meaning not in a `type: module` scope).
    if (getImpliedNodeFormatForEmitWorker(file, options) == ResolutionMode::ESNext || (fileExtensionIsOneOf(&file.filepath().to_string_lossy(), [Extension::Cjs.as_str(), Extension::Cts.as_str(), Extension::Mjs.as_str(), Extension::Mts.as_str()].to_vec())))
        && !file.source_type.is_typescript_definition()
    {
        true
    } else {
        false
    }
}

// getSetExternalModuleIndicator
/** @internal */
pub fn getExternalModuleIndicator(options: &CompilerOptions, file: &Program) -> bool {
    match getEmitModuleDetectionKind(options) {
        ModuleDetectionKind::Force => {
            // All non-declaration files are modules, declaration files still do the usual isFileProbablyExternalModule
            isFileProbablyExternalModule(file) || !file.source_type.is_typescript_definition()
        }
        ModuleDetectionKind::Legacy => {
            // Files are modules if they have imports, exports, or import.meta
            isFileProbablyExternalModule(file)
        }
        ModuleDetectionKind::Auto => {
            // If module is nodenext or node16, all esm format files are modules
            // If jsx is react-jsx or react-jsxdev then jsx tags force module-ness
            // otherwise, the presence of import or export statments (or import.meta) implies module-ness
            let mut is_module = isFileProbablyExternalModule(file);

            if options.jsx.is_some() && (options.jsx.unwrap() == JsxEmit::ReactJSX || options.jsx.unwrap() == JsxEmit::ReactJSXDev) {
                is_module = is_module || isFileModuleFromUsingJSXTag(file);
            }
            is_module = is_module || isFileForcedToBeModuleByFormat(file, options);

            is_module
        }
    }
}
// endregion: 8791

// region: 8917
/** @internal */
pub fn getEmitScriptTarget(compiler_options: &CompilerOptions) -> ScriptTarget {
    let target = if compiler_options.target == Some(ScriptTarget::ES3) { None } else { compiler_options.target };

    target.unwrap_or_else(|| {
        if compiler_options.module == Some(ModuleKind::Node16) {
            ScriptTarget::ES2022
        } else if compiler_options.module == Some(ModuleKind::NodeNext) {
            ScriptTarget::ESNext
        } else {
            ScriptTarget::ES5
        }
    })
}

/** @internal */
pub fn getEmitModuleKind(compiler_options: &CompilerOptions) -> ModuleKind {
    match compiler_options.module {
        Some(module) => module,
        None => {
            if getEmitScriptTarget(compiler_options) >= ScriptTarget::ES2015 {
                ModuleKind::ES2015
            } else {
                ModuleKind::CommonJS
            }
        }
    }
}

/** @internal */
pub fn getEmitModuleResolutionKind(compiler_options: &CompilerOptions) -> ModuleResolutionKind {
    match compiler_options.moduleResolution {
        Some(resolution) => resolution,
        None => match getEmitModuleKind(compiler_options) {
            ModuleKind::CommonJS => ModuleResolutionKind::Node10,
            ModuleKind::Node16 => ModuleResolutionKind::Node16,
            ModuleKind::NodeNext => ModuleResolutionKind::NodeNext,
            ModuleKind::Preserve => ModuleResolutionKind::Bundler,
            _ => ModuleResolutionKind::Classic,
        },
    }
}

/** @internal */
pub fn getEmitModuleDetectionKind(compiler_options: &CompilerOptions) -> ModuleDetectionKind {
    compiler_options.moduleDetection.unwrap_or_else(|| if getEmitModuleKind(compiler_options) == ModuleKind::Node16 || getEmitModuleKind(compiler_options) == ModuleKind::NodeNext { ModuleDetectionKind::Force } else { ModuleDetectionKind::Auto })
}

/** @internal */
pub fn getIsolatedModules(compiler_options: &CompilerOptions) -> bool { compiler_options.isolatedModules.unwrap_or(false) || compiler_options.verbatimModuleSyntax.unwrap_or(false) }

/** @internal */
pub fn getESModuleInterop(compiler_options: &CompilerOptions) -> bool {
    match compiler_options.esModuleInterop {
        Some(value) => value,
        None => matches!(getEmitModuleKind(compiler_options), ModuleKind::Node16 | ModuleKind::NodeNext | ModuleKind::Preserve),
    }
}

/** @internal */
pub fn getAllowSyntheticDefaultImports(compiler_options: &CompilerOptions) -> bool {
    if let Some(value) = compiler_options.allowSyntheticDefaultImports {
        return value;
    }

    getESModuleInterop(compiler_options) || getEmitModuleKind(compiler_options) == ModuleKind::System || getEmitModuleResolutionKind(compiler_options) == ModuleResolutionKind::Bundler
}

/** @internal */
pub fn getResolvePackageJsonExports(compiler_options: &CompilerOptions) -> bool {
    let module_resolution = getEmitModuleResolutionKind(compiler_options);

    if !moduleResolutionSupportsPackageJsonExportsAndImports(module_resolution) {
        return false;
    }

    match compiler_options.resolvePackageJsonExports {
        Some(value) => value,
        None => matches!(module_resolution, ModuleResolutionKind::Node16 | ModuleResolutionKind::NodeNext | ModuleResolutionKind::Bundler),
    }
}

/** @internal */
pub fn getResolvePackageJsonImports(compiler_options: &CompilerOptions) -> bool {
    let module_resolution = getEmitModuleResolutionKind(compiler_options);

    if !moduleResolutionSupportsPackageJsonExportsAndImports(module_resolution) {
        return false;
    }

    match compiler_options.resolvePackageJsonExports {
        Some(value) => value,
        None => matches!(module_resolution, ModuleResolutionKind::Node16 | ModuleResolutionKind::NodeNext | ModuleResolutionKind::Bundler),
    }
}

/** @internal */
pub fn getResolveJsonModule(compiler_options: &CompilerOptions) -> bool { compiler_options.resolveJsonModule.unwrap_or_else(|| getEmitModuleResolutionKind(compiler_options) == ModuleResolutionKind::Bundler) }

/** @internal */
pub fn getEmitDeclarations(compiler_options: &CompilerOptions) -> bool { compiler_options.declaration.unwrap_or(false) || compiler_options.composite.unwrap_or(false) }

/** @internal */
pub fn shouldPreserveConstEnums(compiler_options: &CompilerOptions) -> bool { compiler_options.preserveConstEnums.unwrap_or(false) || getIsolatedModules(compiler_options) }

/** @internal */
pub fn isIncrementalCompilation(compiler_options: &CompilerOptions) -> bool { compiler_options.incremental.unwrap_or(false) || compiler_options.composite.unwrap_or(false) }

/** @internal */
pub fn getAreDeclarationMapsEnabled(compiler_options: &CompilerOptions) -> bool { compiler_options.declarationMap.unwrap_or(false) && getEmitDeclarations(compiler_options) }

/** @internal */
pub fn getAllowJSCompilerOption(compiler_options: &CompilerOptions) -> bool { compiler_options.allowJs.unwrap_or_else(|| compiler_options.checkJs.unwrap_or(false)) }

/** @internal */
pub fn getUseDefineForClassFields(compiler_options: &CompilerOptions) -> bool { compiler_options.useDefineForClassFields.unwrap_or_else(|| getEmitScriptTarget(compiler_options) >= ScriptTarget::ES2022) }

/** @internal */
pub fn emitModuleKindIsNonNodeESM(module_kind: ModuleKind) -> bool { module_kind >= ModuleKind::ES2015 && module_kind <= ModuleKind::ESNext }

/** @internal */
pub fn hasJsonModuleEmitEnabled(options: &CompilerOptions) -> bool {
    match getEmitModuleKind(options) {
        ModuleKind::None | ModuleKind::System | ModuleKind::UMD => false,
        _ => true,
    }
}

/** @internal */
pub fn unreachableCodeIsError(options: &CompilerOptions) -> bool { options.allowUnreachableCode == Some(false) }

/** @internal */
pub fn unusedLabelIsError(options: &CompilerOptions) -> bool { options.allowUnusedLabels == Some(false) }

/** @internal */
pub fn moduleResolutionSupportsPackageJsonExportsAndImports(module_resolution: ModuleResolutionKind) -> bool { (module_resolution >= ModuleResolutionKind::Node16 && module_resolution <= ModuleResolutionKind::NodeNext) || module_resolution == ModuleResolutionKind::Bundler }

/** @internal */
pub enum StrictOptionName {
    NoImplicitAny,
    NoImplicitThis,
    StrictNullChecks,
    StrictFunctionTypes,
    StrictBindCallApply,
    StrictPropertyInitialization,
    StrictBuiltinIteratorReturn,
    AlwaysStrict,
    UseUnknownInCatchVariables,
}

/** @internal */
pub fn getStrictOptionValue(compiler_options: &CompilerOptions, flag: StrictOptionName) -> bool {
    match flag {
        StrictOptionName::NoImplicitAny => compiler_options.noImplicitAny.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::NoImplicitThis => compiler_options.noImplicitThis.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictNullChecks => compiler_options.strictNullChecks.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictFunctionTypes => compiler_options.strictFunctionTypes.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictBindCallApply => compiler_options.strictBindCallApply.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictPropertyInitialization => compiler_options.strictPropertyInitialization.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::StrictBuiltinIteratorReturn => compiler_options.strictBuiltinIteratorReturn.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::AlwaysStrict => compiler_options.alwaysStrict.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
        StrictOptionName::UseUnknownInCatchVariables => compiler_options.useUnknownInCatchVariables.unwrap_or_else(|| compiler_options.strict.unwrap_or(false)),
    }
}

// /** @internal */
// export function getNameOfScriptTarget(scriptTarget: ScriptTarget): string | undefined {
//   return forEachEntry(targetOptionDeclaration.type, (value, key) => value === scriptTarget ? key : undefined);
// }

/** @internal */
pub fn getEmitStandardClassFields(compiler_options: &CompilerOptions) -> bool { compiler_options.useDefineForClassFields != Some(false) && getEmitScriptTarget(compiler_options) >= ScriptTarget::ES2022 }

// /** @internal */
// export function compilerOptionsAffectSemanticDiagnostics(newOptions: CompilerOptions, oldOptions: CompilerOptions): boolean {
//   return optionsHaveChanges(oldOptions, newOptions, semanticDiagnosticsOptionDeclarations);
// }

// /** @internal */
// export function compilerOptionsAffectEmit(newOptions: CompilerOptions, oldOptions: CompilerOptions): boolean {
//   return optionsHaveChanges(oldOptions, newOptions, affectsEmitOptionDeclarations);
// }

// /** @internal */
// export function compilerOptionsAffectDeclarationPath(newOptions: CompilerOptions, oldOptions: CompilerOptions): boolean {
//   return optionsHaveChanges(oldOptions, newOptions, affectsDeclarationPathOptionDeclarations);
// }

// /** @internal */
// export function getCompilerOptionValue(options: CompilerOptions, option: CommandLineOption): unknown {
//   return option.strictFlag ? getStrictOptionValue(options, option.name as StrictOptionName) :
//       option.allowJsFlag ? getAllowJSCompilerOption(options) :
//       options[option.name];
// }

/** @internal */
pub fn getJSXTransformEnabled(options: &CompilerOptions) -> bool { matches!(options.jsx, Some(JsxEmit::React | JsxEmit::ReactJSX | JsxEmit::ReactJSXDev)) }

// /** @internal */
// export function getJSXImplicitImportBase(compilerOptions: CompilerOptions, file?: Program): string | undefined {
//   const jsxImportSourcePragmas = file?.pragmas.get("jsximportsource");
//   const jsxImportSourcePragma = isArray(jsxImportSourcePragmas) ? jsxImportSourcePragmas[jsxImportSourcePragmas.length - 1] : jsxImportSourcePragmas;
//   const jsxRuntimePragmas = file?.pragmas.get("jsxruntime");
//   const jsxRuntimePragma = isArray(jsxRuntimePragmas) ? jsxRuntimePragmas[jsxRuntimePragmas.length - 1] : jsxRuntimePragmas;
//   if (jsxRuntimePragma?.arguments.factory === "classic") {
//       return undefined;
//   }
//   return compilerOptions.jsx === JsxEmit.ReactJSX ||
//           compilerOptions.jsx === JsxEmit.ReactJSXDev ||
//           compilerOptions.jsxImportSource ||
//           jsxImportSourcePragma ||
//           jsxRuntimePragma?.arguments.factory === "automatic" ?
//       jsxImportSourcePragma?.arguments.factory || compilerOptions.jsxImportSource || "react" :
//       undefined;
// }

/** @internal */
pub fn getJSXRuntimeImport(base: Option<&str>, options: &CompilerOptions) -> Option<String> { base.map(|b| format!("{}/{}", b, if options.jsx == Some(JsxEmit::ReactJSXDev) { "jsx-dev-runtime" } else { "jsx-runtime" })) }

/** @internal */
pub fn hasZeroOrOneAsteriskCharacter(str: &str) -> bool {
    let mut seen_asterisk = false;
    for c in str.chars() {
        if c == '*' {
            if !seen_asterisk {
                seen_asterisk = true;
            } else {
                // have already seen asterisk
                return false;
            }
        }
    }
    true
}
// endregion: 9202
