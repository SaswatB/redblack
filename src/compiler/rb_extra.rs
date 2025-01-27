use oxc::ast::ast::Program;
// use std::cell::RefCell;
// use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
// use std::thread_local;

use super::moduleNameResolver::PackageJsonInfo;
use super::rb_host::RbTypeCheckerHost;
use super::types::{CompilerOptions, ResolutionMode};

// #[derive(Clone)]
// struct ProgramInfo {
//     path: PathBuf,
//     package_json_scope: Option<PackageJsonInfo>,
//     external_module_indicator: bool,
//     implied_node_format: ResolutionMode,
// }

// impl ProgramInfo {
//     fn default() -> Self { Self { path: PathBuf::new(), package_json_scope: None, external_module_indicator: false, implied_node_format: ResolutionMode::Undefined } }
// }

// thread_local! {
//     static PROGRAM_INFO_MAP: RefCell<HashMap<usize, ProgramInfo>> = RefCell::new(HashMap::new());
// }

// pub trait ProgramExt {
//     fn set_filepath(&self, filepath: String);
//     fn filepath(&self) -> Option<String>;
//     fn set_package_json_scope(&self, scope: Option<PackageJsonInfo>);
//     fn package_json_scope(&self) -> Option<PackageJsonInfo>;
//     fn set_external_module_indicator(&self, indicator: bool);
//     fn external_module_indicator(&self) -> bool;
//     fn set_implied_node_format(&self, implied_node_format: ResolutionMode);
//     fn implied_node_format(&self) -> ResolutionMode;
//     fn cleanup_thread_local(&self);
// }

// impl<'a> ProgramExt for Program<'a> {
//     fn set_filepath(&self, filepath: String) {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| {
//             let mut map = map.borrow_mut();
//             map.entry(ptr).or_insert_with(|| ProgramInfo::default()).path = PathBuf::from(filepath);
//         });
//     }

//     fn filepath(&self) -> Option<String> {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow().get(&ptr).map(|info| info.path.to_string_lossy().into_owned()))
//     }

//     fn set_package_json_scope(&self, scope: Option<PackageJsonInfo>) {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| {
//             let mut map = map.borrow_mut();
//             map.entry(ptr).or_insert_with(|| ProgramInfo::default()).package_json_scope = scope;
//         });
//     }

//     fn package_json_scope(&self) -> Option<PackageJsonInfo> {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow().get(&ptr).and_then(|info| info.package_json_scope.clone()))
//     }

//     fn set_external_module_indicator(&self, indicator: bool) {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow_mut().entry(ptr).or_insert_with(|| ProgramInfo::default()).external_module_indicator = indicator);
//     }

//     fn external_module_indicator(&self) -> bool {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow().get(&ptr).map(|info| info.external_module_indicator).unwrap_or(false))
//     }

//     fn set_implied_node_format(&self, implied_node_format: ResolutionMode) {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow_mut().entry(ptr).or_insert_with(|| ProgramInfo::default()).implied_node_format = implied_node_format);
//     }

//     fn implied_node_format(&self) -> ResolutionMode {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow().get(&ptr).map(|info| info.implied_node_format).unwrap_or(ResolutionMode::Undefined))
//     }

//     fn cleanup_thread_local(&self) {
//         let ptr = self as *const _ as usize;
//         PROGRAM_INFO_MAP.with(|map| map.borrow_mut().remove(&ptr));
//     }
// }

use paste::paste;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::thread_local;

/**
 * Add extra properties to the given struct, with getters and setters.
 */
macro_rules! entity_properties {
    ($entity:ident, {
        $($name:ident: $type:ty = $default:expr),* $(,)?
    }) => {
        paste! {
            #[derive(Clone)]
            struct [<$entity Info>] {
                $($name: $type,)*
                _phantom: PhantomData<$entity<'static>>,
            }

            impl [<$entity Info>] {
                fn default() -> Self {
                    Self {
                        $($name: $default,)*
                        _phantom: PhantomData,
                    }
                }
            }

            thread_local! {
                static [<$entity:upper _INFO_MAP>]: RefCell<HashMap<usize, [<$entity Info>]>> = RefCell::new(HashMap::new());
            }

            pub trait [<$entity Ext>] {
                $(
                    fn [<set_ $name>](&self, value: $type);
                    fn $name(&self) -> $type;
                )*
            }

            impl<'a> [<$entity Ext>] for $entity<'a> {
                $(
                    fn [<set_ $name>](&self, value: $type) {
                        let ptr = self as *const _ as usize;
                        [<$entity:upper _INFO_MAP>].with(|map| {
                            let mut map = map.borrow_mut();
                            map.entry(ptr).or_insert_with(|| [<$entity Info>]::default()).$name = value;
                        });
                    }

                    fn $name(&self) -> $type {
                        let ptr = self as *const _ as usize;
                        [<$entity:upper _INFO_MAP>].with(|map| map.borrow().get(&ptr).map(|info| info.$name.clone()).unwrap_or($default))
                    }
                )*
            }

            pub fn [<cleanup_ $entity:upper _INFO_MAP>]() {
                [<$entity:upper _INFO_MAP>].with(|map| map.borrow_mut().clear());
            }
        }
    };
}

// Add extra properties to the Program struct.
entity_properties!(Program, {
    filepath: PathBuf = PathBuf::new(),
    package_json_scope: Option<PackageJsonInfo> = None,
    external_module_indicator: bool = false,
    implied_node_format: ResolutionMode = ResolutionMode::Undefined,
});

macro_rules! thread_local_store {
    ($store_name:ident, {
        $($field:ident: $type:ty = $default:expr),* $(,)?
    }) => {
        paste! {
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            struct [<$store_name Data>] {
                $($field: $type,)*
            }

            thread_local! {
                static [<$store_name Store>]: RefCell<[<$store_name Data>]> = RefCell::new([<$store_name Data>] {
                    $($field: $default,)*
                });
            }

            pub static $store_name: [<$store_name Accessor>] = [<$store_name Accessor>] {};

            #[allow(non_camel_case_types)]
            pub struct [<$store_name Accessor>];

            impl [<$store_name Accessor>] {
                $(
                    pub fn [<get_ $field>](&self) -> $type {
                        [<$store_name Store>].with(|store| store.borrow().$field.clone())
                    }

                    pub fn [<set_ $field>](&self, value: $type) {
                        [<$store_name Store>].with(|store| store.borrow_mut().$field = value);
                    }
                )*

                pub fn cleanup(&self) {
                    [<$store_name Store>].with(|store| {
                        *store.borrow_mut() = [<$store_name Data>] {
                            $($field: $default,)*
                        };
                    });
                }
            }
        }
    };
}

thread_local_store!(RB_CTX, {
    type_checker_host: Arc<RbTypeCheckerHost> = Arc::new(RbTypeCheckerHost::new(String::new(), CompilerOptions::default())),
});
