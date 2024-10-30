pub struct Validator;

impl Validator {
    pub fn validate_empty_field(field: Option<String>, field_name: &str) -> Result<String, String> {
        field
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| format!("{field_name} cannot be empty"))
    }
}
