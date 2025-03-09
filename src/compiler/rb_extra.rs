use oxc_ast::ast::SourceFile;
use oxc_ast::AstKind;
use oxc_ast::GetChildren;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

use crate::opt_rc_cell;

use super::moduleNameResolver::PackageJsonInfo;
use super::rb_host::RbTypeCheckerHost;
use super::types::CompilerOptions;
use super::types::Diagnostic;
use super::types::FlowNode;
use super::types::HasLocals;
use super::types::HasLocalsExt;
use super::types::ModifierFlags;
use super::types::NodeFlags;
use super::types::ResolutionMode;
use super::types::Symbol;
use super::types::SymbolTable;

use std::cell::RefCell;
use std::collections::HashMap;
use std::thread_local;

/**
 * Add extra properties to the given struct, with getters and setters.
 */
#[macro_export]
macro_rules! entity_properties {
    ($entity:ident, {
        $($name:ident: $type:ty = $default:expr),* $(,)?
    }) => {
        paste::paste! {
            #[derive(Clone)]
            struct [<$entity Info>] {
                $($name: $type,)*
                _phantom: std::marker::PhantomData<$entity<'static>>,
            }

            impl [<$entity Info>] {
                fn default() -> Self {
                    Self {
                        $($name: $default,)*
                        _phantom: std::marker::PhantomData,
                    }
                }
            }

            thread_local! {
                static [<$entity:upper _INFO_MAP>]: RefCell<HashMap<u32, [<$entity Info>]>> = RefCell::new(HashMap::new());
            }

            pub trait [<$entity Ext>]<'a> {
                $(
                    fn [<set_ $name>](&self, value: rb_macros::replace_lifetime!($type, 'a));
                    fn $name(&self) -> rb_macros::replace_lifetime!($type, 'a);
                )*
            }

            impl<'a> [<$entity Ext>]<'a> for $entity<'a> {
                $(
                    fn [<set_ $name>](&self, value: rb_macros::replace_lifetime!($type, 'a)) {
                        let ptr = self.get_node_id();
                        [<$entity:upper _INFO_MAP>].with(|map| {
                            let mut map = map.borrow_mut();
                            map.entry(ptr).or_insert_with(|| [<$entity Info>]::default()).$name = unsafe { std::mem::transmute(value) };
                        });
                    }

                    fn $name(&self) -> rb_macros::replace_lifetime!($type, 'a) {
                        let ptr = self.get_node_id();
                        [<$entity:upper _INFO_MAP>].with(|map| {
                            let mut map = map.borrow_mut();
                            let info = map.entry(ptr).or_insert_with(|| [<$entity Info>]::default());
                            unsafe { std::mem::transmute_copy(&info.$name) }
                        })
                    }
                )*
            }

            pub fn [<cleanup_ $entity:upper _INFO_MAP>]() {
                [<$entity:upper _INFO_MAP>].with(|map| map.borrow_mut().clear());
            }
        }

    };
}

// Add extra properties to various structs

entity_properties!(SourceFile, {
    filepath: PathBuf = PathBuf::new(),
    packageJsonScope: Option<PackageJsonInfo> = None,
    externalModuleIndicator: bool = false,
    jsGlobalAugmentations: Option<Rc<RefCell<SymbolTable<'static>>>> = None,
    impliedNodeFormat: ResolutionMode = ResolutionMode::Undefined,
    // locals: stored on HasLocals
    symbolCount: usize = 0,
    parseDiagnostics: RefCell<Vec<Diagnostic<'static>>> = RefCell::new(Vec::new()),
    bindDiagnostics: RefCell<Vec<Diagnostic<'static>>> = RefCell::new(Vec::new()),
    classifiableNames: Option<Rc<RefCell<HashSet<String>>>> = None,
});
pub trait SourceFilePassthrough<'a> {
    fn locals<'b>(&'b self) -> opt_rc_cell!(SymbolTable<'b>);
}
impl<'a> SourceFilePassthrough<'a> for SourceFile<'a> {
    fn locals<'b>(&'b self) -> opt_rc_cell!(SymbolTable<'b>) { HasLocals::SourceFile(self).locals().clone() }
}

entity_properties!(AstKind, {
    parent: Option<AstKind<'static>> = None,
    flowNode: Option<Rc<RefCell<FlowNode<'static>>>> = None,
    symbol: Option<Rc<RefCell<Symbol<'static>>>> = None,
    flags: NodeFlags = NodeFlags::None,
    modifierFlagsCache: ModifierFlags = ModifierFlags::None,
});

/**
 * Create a thread-local store.
 */
macro_rules! thread_local_store {
    ($store_name:ident, {
        $($field:ident: $type:ty = $default:expr),* $(,)?
    }) => {
        paste::paste! {
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
    type_checker_host: Rc<RbTypeCheckerHost> = Rc::new(RbTypeCheckerHost::new(String::new(), CompilerOptions::default())),
});
