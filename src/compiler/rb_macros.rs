#[macro_export]
macro_rules! flag_names_impl {
    ($self:expr, $names:expr, $($flag:expr => $name:expr),* $(,)?) => {
        $(
            if $self.contains($flag) {
                $names.push($name);
            }
        )*
    };
}
