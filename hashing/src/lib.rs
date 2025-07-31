pub fn mean(list: &[i32]) -> f64 {
    let mut res = 0.0;
    for i in list {
        res += *i as f64;
    }
    res / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut vec = list.to_vec();
    vec.sort();
    if vec.len() % 2 != 0 {
        vec[vec.len() / 2]
    } else {
        (vec[vec.len() / 2 - 1] + vec[vec.len() / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    use std::collections::HashMap;

    let mut res = HashMap::new();

    for &num in list {
        *res.entry(num).or_insert(0) += 1;
    }

    let mut max_num = 0;
    let mut max_count = 0;

    for (n, count) in res {
        if count > max_count {
            max_num = n;
            max_count = count;
        }
    }
    max_num
}
