use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut map: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            map.insert(c.to_lowercase().next().unwrap());
        }
    }
    println!("{:?}", map);
    map.len() == 26
}