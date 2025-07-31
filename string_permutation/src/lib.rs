pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_count = 0;
    let mut s2_count = 0;
    for c in s1.chars() {
        for i in s1.chars() {
            if i == c {
                s1_count += 1;
            }
        }
        
        for j in s2.chars() {
            if j == c {
                s2_count += 1;
            }
        }
    }
    s1_count == s2_count
}
