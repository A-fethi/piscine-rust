pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut res = None;
    for (i, &v) in array.iter().enumerate() {
        if v == key {
            res = Some(i);
            continue;
        }
    }
    Some(res).unwrap()
}