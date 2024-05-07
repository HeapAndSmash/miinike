#[macro_export]
macro_rules! impl_field {
    ($field_name:ident, $field_type:expr) => {
        impl Field for $field_name {
            fn get_name(&self) -> &str {
                &self.name
            }

            fn get_type(&self) -> FieldType {
                $field_type
            }
        }
    };
}
