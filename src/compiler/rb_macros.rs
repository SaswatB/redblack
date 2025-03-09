#[macro_export]
macro_rules! flag_names_impl {
    ($self:expr, $names:expr, $($flag:expr => $name:expr),* $(,)?) => {
        let mut seen_values = std::collections::HashMap::new();
        $(
            if $self.contains($flag) {
                let value = $flag.0;
                match seen_values.get_mut(&value) {
                    Some(existing) => {
                        *existing = format!("{}/{}", existing, $name);
                    }
                    None => {
                        seen_values.insert(value, String::from($name));
                    }
                }
            }
        )*
        $names.extend(seen_values.into_values());
    };
}

#[macro_export]
macro_rules! define_flags {
    ($name:ident {
        $( $variant:ident = $value:expr ),* $(,)?
    }) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub u64);

        impl $name {
            $(
                pub const $variant: $name = $name($value);
            )*

            /// Returns true if `self` contains all bits of `other`.
            pub fn contains(&self, other: $name) -> bool {
                (self.0 & other.0) == other.0
            }

            pub fn intersects(&self, flags: $name) -> bool {
                (self.0 & flags.0) != 0
            }

            pub fn flag_names(&self) -> Vec<String> {
                let mut names = Vec::new();
                flag_names_impl!(self, names, $($name::$variant => stringify!($variant)),*);
                names
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let names = self.flag_names();
                if names.is_empty() {
                    write!(f, "{}(None)", stringify!($name))
                } else {
                    write!(f, "{}({})", stringify!($name), names.join(" | "))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_string_enum {
    ($name:ident {
        $(
            $variant:ident => $str:expr, $(#[$attr:meta])*
        )*
    }) => {
        pub enum $name {
            $(
                $(#[$attr])*
                $variant,
            )*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $str,)*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! flow_node_enum {
    ($($variant:ident($inner:ty)),* $(,)?) => {
        pub enum FlowNode<'a> {
            $($variant($inner)),*
        }

        impl<'a> FlowNode<'a> {
            pub fn get_flags(&self) -> FlowFlags {
                match self {
                    $(Self::$variant(f) => f.flags),*
                }
            }

            pub fn get_id(&self) -> usize {
                match self {
                    $(Self::$variant(f) => f.id),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! rc_cell {
    ($type:ty) => {
        std::rc::Rc<std::cell::RefCell<$type>>
    };
}

#[macro_export]
macro_rules! new_rc_cell {
    ($type:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($type))
    };
}

#[macro_export]
macro_rules! opt_rc_cell {
    ($type:ty) => {
        Option<std::rc::Rc<std::cell::RefCell<$type>>>
    };
}

#[macro_export]
macro_rules! generate_extended_enum {
    (
        $new_name:ident from $master_name:ident
        direct: [$($dir_var:ident),* $(,)?],
        nl: [$($nl_var:ident),* $(,)?],
        subs:   [$($sub_ty:ident),* $(,)?]
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $new_name<'a> {
            // Direct variants with lifetime
            $($dir_var(&'a $dir_var<'a>),)*
            // Direct variants without lifetime
            $($nl_var(&'a $nl_var),)*
            // Each sub-enum is a single variant that wraps it
            $($sub_ty($sub_ty<'a>),)*
        }

        paste::paste! {
            impl<'a> $new_name<'a> {
                pub fn [<to_ $master_name:snake>](&self) -> $master_name<'a> {
                    match self {
                        $(Self::$dir_var(inner) => $master_name::$dir_var(inner),)*
                        $(Self::$nl_var(inner) => $master_name::$nl_var(inner),)*
                        $(Self::$sub_ty(inner) => inner.[<to_ $master_name:snake>](),)*
                    }
                }
                pub fn [<from_ $master_name:snake>](value: &$master_name<'a>) -> Option<Self> {
                    // match directs
                    let wrapped = match value {
                        $($master_name::$dir_var(inner) => Some(Self::$dir_var(inner)),)*
                        $($master_name::$nl_var(inner) => Some(Self::$nl_var(inner)),)*
                        _ => None
                    };
                    if let Some(wrapped) = wrapped {
                        return Some(wrapped)
                    }
                    // match subs
                    $(if let Some(wrapped) = $sub_ty::[<from_ $master_name:snake>](value) {
                        return Some(Self::$sub_ty(wrapped))
                    })*
                    None
                }
            }
        }
    };
}

#[macro_export]
macro_rules! parse_variants {
    // 1) "End of list": no more tokens => call the generator
    (
        $new_name:ident, $master_name:ident,
        ($($directs:ident),*),
        ($($nls:ident),*),
        ($($subs:ident),*),
        =>
    ) => {
        crate::generate_extended_enum! {
            $new_name from $master_name
            direct: [$($directs),*],
            nl: [$($nls),*],
            subs:   [$($subs),*]
        }
    };

    // 2) Found `Sub(Xyz)`, parse that out as a sub-enum name
    (
        $new_name:ident, $master_name:ident,
        ($($directs:ident),*),
        ($($nls:ident),*),
        ($($subs:ident),*),
        =>
        Sub($sub_ty:ident)
        $($rest:tt)*
    ) => {
        crate::parse_variants! {
            $new_name, $master_name,
            ($($directs),*),
            ($($nls),*),
            ($($subs,)* $sub_ty),
            =>
            $($rest)*
        }
    };

    // 3) Found a no-lifetime variant
    (
        $new_name:ident, $master_name:ident,
        ($($directs:ident),*),
        ($($nls:ident),*),
        ($($subs:ident),*),
        =>
        NL($variant:ident)
        $($rest:tt)*
    ) => {
        crate::parse_variants! {
            $new_name, $master_name,
            ($($directs),*),
            ($($nls,)* $variant),
            ($($subs),*),
            =>
            $($rest)*
        }
    };

    // 4) Found a direct variant
    (
        $new_name:ident, $master_name:ident,
        ($($directs:ident),*),
        ($($nls:ident),*),
        ($($subs:ident),*),
        =>
        $variant:ident
        $($rest:tt)*
    ) => {
        crate::parse_variants! {
            $new_name, $master_name,
            ($($directs,)* $variant),
            ($($nls),*),
            ($($subs),*),
            =>
            $($rest)*
        }
    };

    // 5) Skip commas
    (
        $new_name:ident, $master_name:ident,
        ($($directs:ident),*),
        ($($nls:ident),*),
        ($($subs:ident),*),
        =>
        ,
        $($rest:tt)*
    ) => {
        crate::parse_variants! {
            $new_name, $master_name,
            ($($directs),*),
            ($($nls),*),
            ($($subs),*),
            =>
            $($rest)*
        }
    };
}

#[macro_export]
macro_rules! define_subset_enum {
    (
        $new_name:ident from $master_name:ident { $($body:tt)* }
    ) => {
        crate::parse_variants! {
            $new_name, $master_name,
            (), // start with empty direct list
            (), // start with empty nl list
            (), // start with empty subs list
            =>
            $($body)*
        }
    };
}
