pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    
    if is_vowel(text.chars().nth(0).unwrap()) {
        res.push_str(&(text.to_string() + "ay"));
    } else if is_consonant(text.chars().nth(0).unwrap()) {
        let mut cons = String::new();
        let mut rem = String::new();
        let mut vowel = false;

        for c in text.chars() {
            if !vowel {
                if text.chars().nth(1).unwrap() == 'q' && c == 'u' {
                    cons.push(c);
                    continue;
                }
                if !is_vowel(c) {
                    cons.push(c);
                } else {
                    vowel = true;
                    rem.push(c);
                }
            } else {
                rem.push(c);
            }
        }
        res.push_str(&rem);
        res.push_str(&(cons + "ay"));
    }
    res
}

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn is_consonant(c: char) -> bool {
    c.is_ascii_alphabetic() && !is_vowel(c)
}