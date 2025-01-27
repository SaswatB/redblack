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
