use std::collections::HashSet;

fn find_missing_letter(chars: &[char]) -> char {
    let chars_set: HashSet<char> = chars.iter().clone().map(|x| {*x}).collect();

    let start = format!("{}", chars[0]).as_bytes().to_vec()[0];
    let end = format!("{}", chars[chars.len() - 1]).as_bytes().to_vec()[0];
    let v = (start .. end).map(|c| c as char).collect::<HashSet<char>>();

    let result = v.difference(&chars_set).collect::<Vec<&char>>();

    *result[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}