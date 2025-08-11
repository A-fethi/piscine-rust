pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut res = Vec::new();

    for n in s.split_whitespace() {
        let mut value = 0;
        if n.ends_with('k') {
            let cleaned = &n[..n.len() - 1];
            if let Ok(num) = cleaned.parse::<f32>() {
                value = (num * 1000.0) as u32;
            }
        } else {
            if let Ok(num) = n.parse::<f32>() {
                value = num as u32;
            }
        }
        res.push(Box::new(value));
    }
    res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut new = Vec::new();
    for i in a {
        new.push(*i);
    }
    new
}