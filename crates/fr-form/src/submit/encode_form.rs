use crate::submit::form_field::FormField;
use crate::submit::percent_encode::percent_encode;

pub fn encode_form(fields: &[FormField]) -> String {
    fields
        .iter()
        .map(|field| {
            format!(
                "{}={}",
                percent_encode(&field.name),
                percent_encode(&field.value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}
