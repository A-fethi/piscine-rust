pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            let uc = c.to_uppercase();
            res.push_str(&uc.to_string());
        } else {
            res.push(c);
        }
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let mut start = true;

    for i in input.chars() {
        if i.is_whitespace() {
            start = true;
            res.push(i);
        } else if start {
            let uc = i.to_uppercase().to_string();
            res.push_str(&uc);
            start = false;
        } else {
            res.push(i);
        }
    }
    res
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            if c.is_uppercase() {
                let lc = c.to_lowercase();
                res.push_str(&lc.to_string());
            } else {
                let uc = c.to_uppercase();
                res.push_str(&uc.to_string());
            }
        } else {
            res.push(c);
        }
    }
    res
}
