pub fn talking(text: &str) -> &str {
    if text.trim() == "" {
        return "Just say something!";
    } else if is_upper(text) && text.chars().last() == Some('?') {
        return "Quiet, I am thinking!";
    } else if text.chars().last() == Some('?') {
        return "Sure.";
    } else if is_upper(text) {
        return "There is no need to yell, calm down!"; 
    } else {
        return "Interesting"
    }
}

pub fn is_upper(s: &str) -> bool {
    let mut upper = false;

    for c in s.chars() {
        if c.is_alphabetic() {
            upper = true;
            if !c.is_uppercase() {
                return false;
            }
        }
    }
    upper
}