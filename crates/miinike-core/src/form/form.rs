use crate::fields::{Field, FieldType};

pub struct Form {
    pub name: String,
    pub description: String,
}

impl Form {
    pub fn from_yaml(definition: String) -> Self {
        unimplemented!();
    }

    pub fn from_json(definition: String) -> Self {
        unimplemented!();
    }

    pub fn from_toml(definition: String) -> Self {
        unimplemented!();
    }
}
