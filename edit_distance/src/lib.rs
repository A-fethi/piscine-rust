pub fn edit_distance(source: &str, target: &str) -> usize {
    let src: Vec<char> = source.chars().collect();
    let tgt: Vec<char> = target.chars().collect();

    let mut dp = vec![vec![0; tgt.len() + 1]; src.len() + 1];

    for i in 0..=src.len() {
        dp[i][0] = i;
    }
    for j in 0..=tgt.len() {
        dp[0][j] = j;
    }

    for i in 1..=src.len() {
        for j in 1..=tgt.len() {
            if src[i - 1] == tgt[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }

    dp[src.len()][tgt.len()]
}