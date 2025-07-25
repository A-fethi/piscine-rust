pub fn first_subword(mut s: String) -> String {
    let taking_ownership = s;
    let mut res = String::new();
    for (i, c) in taking_ownership.chars().enumerate() {
        if i == 0 || (!c.is_uppercase() && c != '_') {
            res.push(c);
        } else {
            break;
        }
    }
    res
}
