pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for name in &names {
        let mut chars = String::new();
        chars.push(name.chars().nth(0).unwrap());
        chars.push_str(". ");
        let index = name.find(" ").unwrap();
        chars.push(name.chars().nth(index + 1).unwrap());
        chars.push_str(".");
        res.push(chars);
    }
    res
}