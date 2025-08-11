pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty() || i == 0 {
        return String::new();
    }
    
    let mut res = String::new();
    let chars: Vec<_> = message.chars().collect();
    let chars_len = chars.len();
    let rows = (chars_len + i - 1) / i;

    for row in 0..i {
        for col in 0..rows {
            let index = col * i + row;
            if index < chars_len {
                res.push(chars[index]);
            } else {
                res.push(' ');
            }
        }
    }
    res.trim_end().to_string()
}