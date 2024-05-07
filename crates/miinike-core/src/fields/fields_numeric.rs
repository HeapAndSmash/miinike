use crate::fields::{Field, FieldType};
use crate::impl_field;

pub struct IntegerField {
    name: String,
    value: i32,
}

impl_field!(IntegerField, FieldType::IntegerField);

pub struct FloatField {
    name: String,
    value: f32,
    min_value: Option<f32>,
    max_value: Option<f32>,
    step: Option<f32>,
}

impl_field!(FloatField, FieldType::FloatField);
