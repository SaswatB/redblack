use super::path::{combinePaths, forEachAncestorDirectory};
use crate::compiler::types::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

// region: 312
/** @internal */
pub struct ModuleResolutionState<'a> {
    pub host: &'a dyn ModuleResolutionHost,
    pub compilerOptions: &'a CompilerOptions,
    pub traceEnabled: bool,
    pub failedLookupLocations: Option<Vec<String>>,
    pub affectingLocations: Option<Vec<String>>,
    //pub resultFromCache: Option<ResolvedModuleWithFailedLookupLocations>,
    pub packageJsonInfoCache: Option<&'a mut dyn PackageJsonInfoCache>,
    pub features: NodeResolutionFeatures,
    pub conditions: Vec<String>,
    pub requestContainingDirectory: Option<String>,
    //pub reportDiagnostic: DiagnosticReporter,
    pub isConfigLookup: bool,
    pub candidateIsFromPackageJsonField: bool,
    pub resolvedPackageDirectory: bool,
}

/** Just the fields that we use for module resolution.
 *
 * @internal
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJsonPathFields {
    pub typings: Option<String>,
    pub types: Option<String>,
    pub typesVersions: Option<HashMap<String, HashMap<String, Vec<String>>>>,
    pub main: Option<String>,
    pub tsconfig: Option<String>,
    pub r#type: Option<String>,
    // pub imports: Option<Box<dyn std::any::Any>>,
    // pub exports: Option<Box<dyn std::any::Any>>,
    pub name: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
    pub peerDependencies: Option<HashMap<String, String>>,
    pub optionalDependencies: Option<HashMap<String, String>>,

    pub version: Option<String>, // from PackageJson
}

// I just merged these 2
pub type PackageJson = PackageJsonPathFields;
// endregion: 352

// region: 416
/** @internal */
#[derive(Debug, Clone)]
pub struct VersionPaths {
    pub version: String,
    pub paths: HashMap<String, Vec<String>>,
}
// endregion: 420

// region: 908
/** @internal */
#[derive(Debug)]
pub struct MissingPackageJsonInfo {
    pub packageDirectory: String,
    pub directoryExists: bool,
}

/** @internal */
#[derive(Debug)]
pub enum PackageJsonInfoCacheEntry {
    PackageJsonInfo(PackageJsonInfo),
    MissingPackageJsonInfo(MissingPackageJsonInfo),
}

pub fn get_package_json_info_cache_entry_package_directory(entry: &PackageJsonInfoCacheEntry) -> &str {
    match entry {
        PackageJsonInfoCacheEntry::PackageJsonInfo(info) => &info.packageDirectory,
        PackageJsonInfoCacheEntry::MissingPackageJsonInfo(info) => &info.packageDirectory,
    }
}

/** @internal */
pub fn isPackageJsonInfo(entry: Option<&PackageJsonInfoCacheEntry>) -> bool { matches!(entry, Some(PackageJsonInfoCacheEntry::PackageJsonInfo(_))) }

/** @internal */
pub fn isMissingPackageJsonInfo(entry: Option<&PackageJsonInfoCacheEntry>) -> bool { matches!(entry, Some(PackageJsonInfoCacheEntry::MissingPackageJsonInfo(_))) }

/** @internal */
pub trait PackageJsonInfoCache {
    /** @internal */
    fn getPackageJsonInfo(&self, packageJsonPath: &str) -> Option<PackageJsonInfoCacheEntry>;
    /** @internal */
    fn setPackageJsonInfo(&mut self, packageJsonPath: &str, info: PackageJsonInfoCacheEntry);
    /** @internal */
    fn getInternalMap(&self) -> Option<HashMap<String, PackageJsonInfoCacheEntry>>;
    fn clear(&mut self);
    /** @internal */
    fn isReadonly(&self) -> bool;
}
// endregion: 933

// region: 1679
/** @internal */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeResolutionFeatures(isize);

impl NodeResolutionFeatures {
    pub const None: NodeResolutionFeatures = NodeResolutionFeatures(0);
    // resolving `#local` names in your own package.json
    pub const Imports: NodeResolutionFeatures = NodeResolutionFeatures(1 << 1);
    // resolving `your-own-name` from your own package.json
    pub const SelfName: NodeResolutionFeatures = NodeResolutionFeatures(1 << 2);
    // respecting the `.exports` member of packages' package.json files and its (conditional) mappings of export names
    pub const Exports: NodeResolutionFeatures = NodeResolutionFeatures(1 << 3);
    // allowing `*` in the LHS of an export to be followed by more content, eg `"./whatever/*.js"`
    // not supported in node 12 - https://github.com/nodejs/Release/issues/690
    pub const ExportsPatternTrailers: NodeResolutionFeatures = NodeResolutionFeatures(1 << 4);
    pub const AllFeatures: NodeResolutionFeatures = NodeResolutionFeatures(Self::Imports.0 | Self::SelfName.0 | Self::Exports.0 | Self::ExportsPatternTrailers.0);
    pub const Node16Default: NodeResolutionFeatures = NodeResolutionFeatures(Self::Imports.0 | Self::SelfName.0 | Self::Exports.0 | Self::ExportsPatternTrailers.0);
    pub const NodeNextDefault: NodeResolutionFeatures = Self::AllFeatures;
    pub const BundlerDefault: NodeResolutionFeatures = NodeResolutionFeatures(Self::Imports.0 | Self::SelfName.0 | Self::Exports.0 | Self::ExportsPatternTrailers.0);
    pub const EsmMode: NodeResolutionFeatures = NodeResolutionFeatures(1 << 5);
}

impl std::ops::BitOr for NodeResolutionFeatures {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { NodeResolutionFeatures(self.0 | rhs.0) }
}

impl std::ops::BitAnd for NodeResolutionFeatures {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { NodeResolutionFeatures(self.0 & rhs.0) }
}
// endregion: 1700

// region: 2009
/** @internal */
pub const nodeModulesPathPart: &str = "/node_modules/";
/** @internal */
pub fn pathContainsNodeModules(path: &str) -> bool { path.contains(nodeModulesPathPart) }
// endregion: 2014

// region: 2345
/** @internal */
pub fn getTemporaryModuleResolutionState<'a>(packageJsonInfoCache: Option<&'a mut dyn PackageJsonInfoCache>, host: &'a dyn ModuleResolutionHost, options: &'a CompilerOptions) -> ModuleResolutionState<'a> {
    ModuleResolutionState {
        host,
        compilerOptions: options,
        traceEnabled: false, // isTraceEnabled(options, host),
        failedLookupLocations: None,
        affectingLocations: None,
        // resultFromCache: None,
        packageJsonInfoCache,
        features: NodeResolutionFeatures::None,
        conditions: Vec::new(),
        requestContainingDirectory: None,
        // reportDiagnostic: None,
        isConfigLookup: false,
        candidateIsFromPackageJsonField: false,
        resolvedPackageDirectory: false,
    }
}

/** @internal */
#[derive(Debug, Clone)]
pub struct PackageJsonInfo {
    pub packageDirectory: String,
    pub contents: PackageJsonInfoContents,
}
/** @internal */
#[derive(Debug, Clone)]
pub struct PackageJsonInfoContents {
    pub packageJsonContent: PackageJsonPathFields,
    pub versionPaths: Option<VersionPaths>,
    pub versionPathsResolved: bool,
    pub resolvedEntrypoints: Option<Vec<String>>,
    pub resolvedEntrypointsResolved: bool,
    pub peerDependencies: Option<String>,
    pub peerDependenciesResolved: bool,
}

/**
 * A function for locating the package.json scope for a given path
 *
 * @internal
 */
pub fn getPackageScopeForPath(directory: &str, state: &mut ModuleResolutionState) -> Option<PackageJsonInfo> {
    forEachAncestorDirectory(directory, |dir| getPackageJsonInfo(dir, /*onlyRecordFailures*/ false, state))
}

// fn getVersionPathsOfPackageJsonInfo(packageJsonInfo: &PackageJsonInfo, state: &ModuleResolutionState) -> Option<VersionPaths> {
//     if packageJsonInfo.contents.versionPaths.is_none() {
//         packageJsonInfo.contents.versionPaths = readPackageJsonTypesVersionPaths(&packageJsonInfo.contents.packageJsonContent, state).or(Some(false));
//     }
//     packageJsonInfo.contents.versionPaths
// }

// fn getPeerDependenciesOfPackageJsonInfo(packageJsonInfo: &PackageJsonInfo, state: &ModuleResolutionState) -> Option<String> {
//     if packageJsonInfo.contents.peerDependencies.is_none() {
//         packageJsonInfo.contents.peerDependencies = readPackageJsonPeerDependencies(packageJsonInfo, state).or(Some(false));
//     }
//     packageJsonInfo.contents.peerDependencies
// }

// fn readPackageJsonPeerDependencies(packageJsonInfo: &PackageJsonInfo, state: &ModuleResolutionState) -> Option<String> {
//     let peerDependencies = readPackageJsonField(&packageJsonInfo.contents.packageJsonContent, "peerDependencies", "object", state);
//     if peerDependencies.is_none() {
//         return None;
//     }
//     // if state.traceEnabled {
//     //     trace(state.host, Diagnostics::package_json_has_a_peerDependencies_field);
//     // }
//     let packageDirectory = realPath(&packageJsonInfo.packageDirectory, state.host, state.traceEnabled);
//     let nodeModules = format!("{}/", packageDirectory[..packageDirectory.rfind("node_modules").unwrap() + "node_modules".len()]);
//     let mut result = String::new();
//     for (key, _) in peerDependencies.iter() {
//         if hasProperty(peerDependencies, key) {
//             let peerPackageJson = getPackageJsonInfo(&(nodeModules.clone() + key), /*onlyRecordFailures*/ false, state);
//             if let Some(peerPackageJson) = peerPackageJson {
//                 let version = peerPackageJson.contents.packageJsonContent.version.as_ref().unwrap();
//                 result.push_str(&format!("+{}@{}", key, version));
//                 // if state.traceEnabled {
//                 //     trace(state.host, Diagnostics::Found_peerDependency_0_with_1_version, key, version);
//                 // }
//             } else {
//                 // Read the dependency version
//                 // if state.traceEnabled {
//                 //     trace(state.host, Diagnostics::Failed_to_find_peerDependency_0, key);
//                 // }
//             }
//         }
//     }
//     Some(result)
// }

fn getPackageJsonInfo(packageDirectory: &str, onlyRecordFailures: bool, state: &mut ModuleResolutionState) -> Option<PackageJsonInfo> {
    let host = state.host;
    // let traceEnabled = state.traceEnabled;
    let packageJsonPath = combinePaths(packageDirectory, &[Some("package.json")]);
    if onlyRecordFailures {
        if let Some(locations) = &mut state.failedLookupLocations {
            locations.push(packageJsonPath.to_string());
        }
        return None;
    }

    let existing = state.packageJsonInfoCache.as_ref().and_then(|cache| cache.getPackageJsonInfo(&packageJsonPath));
    if let Some(existing) = existing {
        if let PackageJsonInfoCacheEntry::PackageJsonInfo(existing) = existing {
            // if traceEnabled {
            //     trace(host, Diagnostics::File_0_exists_according_to_earlier_cached_lookups, &packageJsonPath);
            // }
            if let Some(locations) = &mut state.affectingLocations {
                locations.push(packageJsonPath.to_string());
            }
            if existing.packageDirectory == packageDirectory {
                return Some(existing);
            }
            return Some(PackageJsonInfo { packageDirectory: packageDirectory.to_string(), contents: existing.contents });
        } else {
            // if existing.directoryExists && traceEnabled {
            //     trace(host, Diagnostics::File_0_does_not_exist_according_to_earlier_cached_lookups, &packageJsonPath);
            // }
            if let Some(locations) = &mut state.failedLookupLocations {
                locations.push(packageJsonPath.to_string());
            }
            return None;
        }
    }
    let directoryExists = host.directoryExists(packageDirectory).unwrap_or(true); // if host does not support 'directoryExists' assume that directory will exist
    if directoryExists && host.fileExists(&packageJsonPath) {
        let contents = host.readFile(&packageJsonPath).unwrap();
        let packageJsonContent: PackageJsonPathFields = serde_json::from_str(&contents.as_str()).unwrap();
        // let packageJsonContent = readJson(&packageJsonPath, host) as PackageJson;
        // if traceEnabled {
        //     trace(host, Diagnostics::Found_package_json_at_0, &packageJsonPath);
        // }
        let result = PackageJsonInfo {
            packageDirectory: packageDirectory.to_string(),
            contents: PackageJsonInfoContents { packageJsonContent, versionPaths: None, versionPathsResolved: false, resolvedEntrypoints: None, resolvedEntrypointsResolved: false, peerDependencies: None, peerDependenciesResolved: false },
        };
        if let Some(cache) = &mut state.packageJsonInfoCache {
            if !cache.isReadonly() {
                cache.setPackageJsonInfo(&packageJsonPath, PackageJsonInfoCacheEntry::PackageJsonInfo(result.clone()));
            }
        }
        if let Some(locations) = &mut state.affectingLocations {
            locations.push(packageJsonPath.to_string());
        }
        Some(result)
    } else {
        // if directoryExists && traceEnabled {
        //     trace(host, Diagnostics::File_0_does_not_exist, &packageJsonPath);
        // }
        if let Some(cache) = &mut state.packageJsonInfoCache {
            if !cache.isReadonly() {
                cache.setPackageJsonInfo(&packageJsonPath, PackageJsonInfoCacheEntry::MissingPackageJsonInfo(MissingPackageJsonInfo { packageDirectory: packageDirectory.to_string(), directoryExists }));
            }
        }
        // record package json as one of failed lookup locations - in the future if this file will appear it will invalidate resolution results
        if let Some(locations) = &mut state.failedLookupLocations {
            locations.push(packageJsonPath.to_string());
        }
        None
    }
}
// endregion: 2473
