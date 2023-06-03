fn find_short(s: &str) -> u32 {
    let mut result: u32 = u32::MAX;
    for i in s.split_whitespace() {
        if i.len() as u32 <= result {
            result = i.len() as u32;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::find_short;

    fn dotest(s: &str, expected: u32) {
        let actual = find_short(s);
        assert!(actual == expected,
                "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("bitcoin take over the world maybe who knows perhaps", 3);
        dotest("turns out random test cases are easier than writing out basic ones", 3);
        dotest("lets talk about javascript the best language", 3);
        dotest("i want to travel the world writing code one day", 1);
        dotest("Lets all go on holiday somewhere very cold", 2);
        dotest("Let's travel abroad shall we", 2);
    }
}