pub mod client;
pub mod transaction;

#[macro_export]
macro_rules! new_string_type {
    ($type:ident, max_length = $max_length:expr, error = $error_message:expr, min_length = $min_length:expr, error_min = $error_min:expr) => {
        #[derive(Clone, Serialize, Deserialize, FromRow, sqlx::Type)]
        #[serde(try_from = "String")]
        #[sqlx(type_name = "VARCHAR")]
        pub struct $type(String);

        impl $type {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $type {
            type Error = &'static str;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.len() < $min_length {
                    Err($error_min)
                } else if value.len() <= $max_length {
                    Ok($type(value))
                } else {
                    Err($error_message)
                }
            }
        }

        impl From<$type> for String {
            fn from(value: $type) -> Self {
                value.0
            }
        }
    };
}
