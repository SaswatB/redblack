use oxc_ast::ast::SourceFile;
use oxc_ast::AstKind;
use oxc_ast::GetChildren;
use std::path::PathBuf;
use std::sync::Arc;

use super::moduleNameResolver::PackageJsonInfo;
use super::rb_host::RbTypeCheckerHost;
use super::types::{CompilerOptions, ResolutionMode};

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
                static [<$entity:upper _INFO_MAP>]: RefCell<HashMap<u32, [<$entity Info>]>> = RefCell::new(HashMap::new());
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
                        let ptr = self.get_node_id();
                        [<$entity:upper _INFO_MAP>].with(|map| {
                            let mut map = map.borrow_mut();
                            map.entry(ptr).or_insert_with(|| [<$entity Info>]::default()).$name = value;
                        });
                    }

                    fn $name(&self) -> $type {
                        let ptr = self.get_node_id();
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

// Add extra properties to the SourceFile struct.
entity_properties!(SourceFile, {
    filepath: PathBuf = PathBuf::new(),
    package_json_scope: Option<PackageJsonInfo> = None,
    external_module_indicator: bool = false,
    implied_node_format: ResolutionMode = ResolutionMode::Undefined,
});

entity_properties!(AstKind, {
    parent: Option<AstKind<'static>> = None,
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
