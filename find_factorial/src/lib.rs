pub fn factorial(num: u64) -> u64 {
    let mut i = 1;
    let mut res = 1;

    while i <= num {
        res *= i;
        i = i + 1;
    }
    res
}