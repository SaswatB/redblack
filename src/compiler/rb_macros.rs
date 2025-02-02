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
