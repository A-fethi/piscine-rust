pub fn spell(n: u64) -> String {
    if n == 1_000_000 {
        return "one million".to_string();
    }

    if n < 100 {
        return less_than_hundred(n);
    }

    if n < 1000 {
        return less_than_thousand(n);
    } else {
        match n {
        0..=99 => less_than_hundred(n),
        1000..=999_999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;

            if remainder == 0 {
                return format!("{} thousand", less_than_thousand(thousands))
            } else {
                return format!("{} thousand {}", less_than_thousand(thousands), less_than_thousand(remainder))
            }
        }
        _ => "".to_string(),
        }
    }
}

pub fn less_than_thousand(n: u64) -> String {
    return match n {
        0..=99 => less_than_hundred(n),
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;

            if remainder == 0 {
                format!("{} hundred", units(hundreds))
            } else {
                format!("{} hundred {}", units(hundreds), less_than_hundred(remainder))
            }
        }
        _ => "".to_string(),
    }
}

pub fn less_than_hundred(n: u64) -> String {
    return match n {
        0..=9 => units(n),
        10..=19 => teens(n),
        20..=99 => {
            let ten = n / 10;
            let unit = n % 10;
            if unit == 0 {
                tens(ten).to_string()
            } else {
                format!("{}-{}", tens(ten), units(unit))
            }
        }
        _ => "".to_string(),
    }
}

pub fn units(n: u64) -> String {
    return match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => "".to_string(),
    }
}

pub fn tens(n: u64) -> String {
    return match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "".to_string(),
    }
}

pub fn teens(n: u64) -> String {
    return match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    }
}