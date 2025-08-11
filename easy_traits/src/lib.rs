#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        StringValue {
            value: self.value.clone(),
        }
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self {
        let str_nb = nb_to_append.to_string();
        self.value.push_str(&str_nb);
        StringValue {
            value: self.value.clone(),
        }
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        let mut res = String::new();
        for c in self.value.chars() {
            if c == '.' || c == ',' || c == '!' || c == '?'{
                continue;
            }
            res.push(c);
        }
        self.value = res;
        StringValue { value: self.value.clone() }
    }
}