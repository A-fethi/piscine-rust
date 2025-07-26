pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort_by_key(|word| {
        word.chars().filter(|c| c.is_numeric()).collect::<String>()
    });
    let mut res = String::new();
    for (i, word) in words.iter().enumerate() {
        for c in word.chars() {
            if c.is_numeric() {
                continue;
            } else {
                res.push(c);
            }
        }
        if i != words.len() - 1 {
            res.push_str(" ");
        }
    }
    res
}
