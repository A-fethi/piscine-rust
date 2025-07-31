use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res = 0;
    for num in h {
        if num.1 > 0 && num.1 > res {
            res = num.1;
        }
    }
    res
}