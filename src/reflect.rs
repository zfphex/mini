/// A macro to help you with compile-time reflection. It's not real reflection but it's useful.
/// ```
/// use mini::reflect;
///
/// reflect! {
///     Key,             //enum Key
///     F1,              //Key::F1
///     Space => "SPACE" //Key::Space
/// }
///
/// assert!(Key::from_str("SPACE").is_ok());
/// assert!(Key::from_str("Space").is_ok());
/// ```
#[macro_export]
macro_rules! reflect {
    ($enum_name:ident, $($name:ident $(=> $str:expr)?),*) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $enum_name {
        $(
            $name
        ),*
        }

        impl $enum_name {
            pub fn from_str(input: &str) -> Result<Self, ()> {
                match input {
                    $(
                        stringify!($name) => Ok(Self::$name),
                        $(
                            $str => Ok(Self::$name),
                        )*
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    reflect! {
        Key,
        F1,
        Space => "SPACE"
    }

    #[test]
    fn test() {
        Key::from_str("F1").unwrap();
        Key::from_str("Space").unwrap();
        Key::from_str("SPACE").unwrap();
    }
}
