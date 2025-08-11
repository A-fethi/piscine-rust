pub fn get_diamond(c: char) -> Vec<String> {
    let mut arr = Vec::<String>::new();
    let index = c as u8 - b'A';
    let alphabet: Vec<char> = ('A' as u8..='Z' as u8).map(|c| c as char).collect();

    for i in 0..=index {
        let mut line = String::new();
        let diff = index - i;
        let mut k = 0;
        while k < diff {
            line.push(' ');
            k += 1;
        }
        line.push(alphabet[i as usize]);

        if i != 0 {
            let mut k = 0;
            while k < (i * 2 - 1) {
                line.push(' ');
                k += 1;
            }
            line.push(alphabet[i as usize]);
        }

        let mut k = 0;
        while k < diff {
            line.push(' ');
            k += 1;
        }
        arr.push(line);
    }

    let mut i = index;
    while i > 0 {
        i -= 1;
        arr.push(arr[i as usize].clone());
    }
    arr
}