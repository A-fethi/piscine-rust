pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let num_len = s.len() as u32;
    let mut res = 0;

    for n in s.chars() {
        res += n.to_digit(10).unwrap().pow(num_len)
    }
    res == num
}