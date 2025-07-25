pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let float = c as f64;
    (c, float.exp(), float.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for (i, f) in a.chars().enumerate() {
        let int = f.to_digit(10).unwrap();
        let float = int as f64;
        if f != ' ' {
            res.push_str(&float.exp().to_string());
            if i != a.chars().count() - 1 {
                res.push(' ');
            }
        }
    }
    (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res_vec: Vec<f64> = Vec::new();
    for ele in &b {
        let float = *ele as f64;
        res_vec.push(float.abs().ln());
    }
    (b, res_vec)
}
