use chrono::Local;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        FormError {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }
        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.clone(), "Password should be at least 8 characters long"));
        }
        
        let mut has_alpha = false;
        let mut has_digit = false;
        let mut has_symbol = false;

        for c in self.password.chars() {
            if c.is_ascii_alphabetic() {
                has_alpha = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            } else if c.is_ascii_punctuation() || c.is_ascii_graphic() {
                has_symbol = true;
            }
        }

        if !(has_alpha && has_digit && has_symbol) {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        }
        Ok(())
    }
}