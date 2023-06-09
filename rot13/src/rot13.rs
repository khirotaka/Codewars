fn rot13(message: &str) -> String {
    let mut result: Vec<char> = vec![];
    for m in message.chars() {
        if m.is_alphabetic() {
            let a = if m.is_lowercase() {
                'a' as u32
            }
            else {
                'A' as u32
            };
            let tmp = char::from_u32((m as u32 - a + 13) % 26 + a).unwrap();
            result.push(tmp);
        }
        else {
            result.push(m);
        }
    }

    String::from_iter(result)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rot13;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
    }
}
