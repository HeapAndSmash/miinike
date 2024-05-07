pub mod field_macros;
pub mod fields_geographic;
pub mod fields_numeric;
pub mod fields_repeater;
pub mod fields_selection;
pub mod fields_temporal;
pub mod fields_text;

pub enum FieldType {
    DateField,
    DateTimeField,
    TimeField,
    YearField,
    ISOWeekField,
    TextField,
    IntegerField,
    FloatField,
    SliderField,
    SelectField,
    MultiSelectField,
}

pub trait Field {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> FieldType;
}
