pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut count = 0;
    for ele in s.chars() {
        if count > 0 && ele != '-' && ele != '+' {
           count -= 1;
           continue; 
        }
        if ele == '-' {
            res.pop();
        } else if ele == '+' {
            count += 1;
        } else {
            res.push(ele);
        }
    }
    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    for i in v {
        if i.contains("+") {
            let parts: Vec<_> = i.split("+").collect();
            let sum = parts[0].parse::<i32>().unwrap() + parts[1].parse::<i32>().unwrap();
            *i = sum.to_string();
        } else if i.contains("-") {
            let parts: Vec<_> = i.split("-").collect();
            let diff = parts[0].parse::<i32>().unwrap() - parts[1].parse::<i32>().unwrap();
            *i = diff.to_string();
        }
    }
}