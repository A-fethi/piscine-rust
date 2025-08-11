pub fn num_to_ordinal(x: u32) -> String {
    let last_two_digits = x % 100;
    let last_digit = x % 10;
    let mut res = String::new();
    let mut ordinal = "";

    if last_two_digits == 13 || last_two_digits == 12 || last_two_digits == 11 {
        ordinal = "th"
    } else {
        ordinal = match last_digit {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
    }
    res.push_str(&x.to_string());
    res.push_str(ordinal);
    res
}