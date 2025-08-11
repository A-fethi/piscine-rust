#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

// equation used is E(x) = (m - 1) - x where m is the number of letters in the alphabet (26) and x is the numerical position of the letter (A=0, B=1, ... Z=25).
pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let m: u8 = 26;
    let mut expected = String::new();
    
    for c in original.chars() {
        if c.is_ascii_uppercase() {
            let x = c as u8 - b'A';
            let e = (m - 1) - x;
            expected.push((b'A' + e) as char);
        } else if c.is_ascii_lowercase() {
            let x = c as u8 - b'a';
            let e = (m - 1) - x;
            expected.push((b'a' + e) as char);
        } else {
            expected.push(c);
        }
    }

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}