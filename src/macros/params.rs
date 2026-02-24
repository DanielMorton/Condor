/// Macro for defining common parameter sets
#[macro_export]
macro_rules! define_param_group {
    (
        $name:ident {
            $(
                $param:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        pub struct $name {
            $(
                pub $param: Option<$type>,
            )*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $(
                        $param: None,
                    )*
                }
            }

            $(
                pub fn $param(mut self, value: $type) -> Self {
                    self.$param = Some(value);
                    self
                }
            )*

            pub fn to_params(&self) -> std::collections::HashMap<String, String> {
                let mut params = std::collections::HashMap::new();
                $(
                    if let Some(ref value) = self.$param {
                        params.insert(
                            stringify!($param).to_string(),
                            value.to_string()
                        );
                    }
                )*
                params
            }
        }
    };
}