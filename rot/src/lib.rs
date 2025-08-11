pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    for i in input.chars() {
        let rotated: char;

        if i.is_ascii_lowercase() {
            rotated = ((i as u8 - b'a' + key.rem_euclid(26) as u8) % 26 + b'a') as char;
        } else if i.is_ascii_uppercase() {
            rotated = ((i as u8 - b'A' + key.rem_euclid(26) as u8) % 26 + b'A') as char;
        } else {
            rotated = i;
        }
        res.push(rotated);
    }
    res
}