use oxc_allocator::Allocator;
use oxc_ast::{
    ast::{MetaProperty, SourceFile, Statement},
    match_module_declaration, Visit,
};
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use std::path::Path;
use std::sync::LazyLock;

use super::{
    moduleNameResolver::PackageJsonInfo,
    program::getImpliedNodeFormatForFile,
    rb_extra::{SourceFileExt, RB_CTX},
    types::{ResolutionMode, ScriptTarget, TypeCheckerHost},
};

static ALLOCATOR: LazyLock<Allocator> = LazyLock::new(|| Allocator::default());

// region: 460
/** @internal */
pub fn isJSDocLikeText(text: &str, start: usize) -> bool { text.chars().nth(start + 1) == Some('*') && text.chars().nth(start + 2) == Some('*') && text.chars().nth(start + 3) != Some('/') }

/** @internal */
pub fn isFileProbablyExternalModule(source_file: &SourceFile) -> bool {
    // Try to use the first top-level import/export when available, then
    // fall back to looking for an 'import.meta' somewhere in the tree if necessary.
    source_file.body.iter().find(|s| isAnExternalModuleIndicatorNode(s)).is_some() || getImportMetaIfNecessary(source_file)
}

fn isAnExternalModuleIndicatorNode(node: &Statement) -> bool {
    // (canHaveModifiers(node) && hasModifierOfKind(node, SyntaxKind::ExportKeyword)) ||
    // (isImportEqualsDeclaration(node) && isExternalModuleReference(&node.moduleReference)) || isImportDeclaration(node) || isExportAssignment(node) || isExportDeclaration(node)

    match node {
        match_module_declaration!(Statement) => true,
        _ => false, // todo handle isExternalModuleReference (use of require())
    }
}

fn getImportMetaIfNecessary(source_file: &SourceFile) -> bool {
    let mut finder = FindImportMeta { has_import_meta: false };
    finder.visit_source_file(source_file);
    return finder.has_import_meta;
}

// walkTreeForImportMeta
#[derive(Debug, Default)]
struct FindImportMeta {
    has_import_meta: bool,
}

impl<'a> Visit<'a> for FindImportMeta {
    fn visit_meta_property(&mut self, node: &MetaProperty) {
        if node.meta.name == "import" && node.property.name == "meta" {
            self.has_import_meta = true;
        }
    }
}

//** Do not use hasModifier inside the parser; it relies on parent pointers. Use this instead. */
// fn hasModifierOfKind(node: &HasModifiers, kind: SyntaxKind) -> bool { node.modifiers.iter().any(|m| m.kind == kind) }

// fn isImportMeta(node: &Node) -> bool { isMetaProperty(node) && node.keywordToken == SyntaxKind::ImportKeyword && node.name.escapedText == "meta" }
// endregion: 500

// region: 1320
#[derive(Debug, Clone)]
pub struct CreateSourceFileOptions {
    pub languageVersion: ScriptTarget,
    /**
     * Controls the format the file is detected as - this can be derived from only the path
     * and files on disk, but needs to be done with a module resolution cache in scope to be performant.
     * This is usually `undefined` for compilations that do not have `moduleResolution` values of `node16` or `nodenext`.
     */
    pub impliedNodeFormat: Option<ResolutionMode>,
    /**
     * Controls how module-y-ness is set for the given file. Usually the result of calling
     * `getSetExternalModuleIndicator` on a valid `CompilerOptions` object. If not present, the default
     * check specified by `isFileProbablyExternalModule` will be used to set the field.
     */
    // pub setExternalModuleIndicator: Option<Box<dyn Fn(&SourceFile)>>,
    /** @internal */
    pub packageJsonLocations: Option<Vec<String>>,
    /** @internal */
    pub packageJsonScope: Option<PackageJsonInfo>,
    // pub jsDocParsingMode: Option<JSDocParsingMode>,
}

impl Default for CreateSourceFileOptions {
    fn default() -> Self { Self { languageVersion: ScriptTarget::ESNext, impliedNodeFormat: None, packageJsonLocations: None, packageJsonScope: None } }
}

// pub fn setExternalModuleIndicator(source_file: &mut SourceFile) { source_file.externalModuleIndicator = isFileProbablyExternalModule(source_file); }

// pub fn createSourceFile<'a>(file_name: &'a str, source_text: &'a str, language_version_or_options: CreateSourceFileOptions, set_parent_nodes: bool, script_kind: Option<ScriptKind>) -> SourceFile<'a> {
pub fn createSourceFile<'a>(file_name: &'a str, source_text: &'a str) -> SourceFile<'a> {
    // tracing.as_mut().map(|t| t.push(Phase::Parse, "createSourceFile", json!({ "path": file_name }), true));
    // performance::mark("beforeParse");
    let result: SourceFile;

    // let (language_version, override_set_external_module_indicator, format, js_doc_parsing_mode) = match language_version_or_options {
    //     ScriptTargetOrCreateSourceFileOptions::Options(opts) => (opts.language_version, opts.setExternalModuleIndicator, opts.impliedNodeFormat, opts.jsDocParsingMode),
    //     ScriptTargetOrCreateSourceFileOptions::Target(target) => (target, None, None, None),
    // };

    // if language_version == ScriptTarget::JSON {
    //     result = Parser::parseSourceFile(
    //         file_name,
    //         source_text,
    //         language_version,
    //         None, // syntaxCursor
    //         set_parent_nodes,
    //         ScriptKind::JSON,
    //         Box::new(|_| {}), // noop
    //         js_doc_parsing_mode,
    //     );
    // } else {
    //     let set_indicator = if format.is_none() {
    //         override_set_external_module_indicator
    //     } else {
    //         Some(Box::new(move |file: &mut SourceFile| {
    //             file.impliedNodeFormat = format;
    //             match &override_set_external_module_indicator {
    //                 Some(f) => f(file),
    //                 None => setExternalModuleIndicator(file),
    //             }
    //         }))
    //     };

    //     result = Parser::parseSourceFile(
    //         file_name,
    //         source_text,
    //         language_version,
    //         None, // syntaxCursor
    //         set_parent_nodes,
    //         script_kind,
    //         set_indicator,
    //         js_doc_parsing_mode,
    //     );
    // }

    let path = Path::new(file_name);
    let source_type = SourceType::from_path(&path).unwrap(); // todo merge with params?
    let ret = Parser::new(&ALLOCATOR, &source_text, source_type).with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() }).parse();
    result = ret.source_file;
    result.set_filepath(path.to_path_buf());
    result.set_packageJsonScope(None); // todo
    result.set_externalModuleIndicator(isFileProbablyExternalModule(&result));
    let tc_host = &*RB_CTX.get_type_checker_host();
    result.set_impliedNodeFormat(getImpliedNodeFormatForFile(file_name, tc_host, tc_host.getCompilerOptions()));

    // performance::mark("afterParse");
    // performance::measure("Parse", "beforeParse", "afterParse");
    // tracing.as_mut().map(|t| t.pop());
    result
}
// endregion: 1369

// region: 1384
// See also `isExternalOrCommonJsModule` in utilities.ts
pub fn isExternalModule(file: &SourceFile) -> bool { file.externalModuleIndicator() }
// endregion: 1387
