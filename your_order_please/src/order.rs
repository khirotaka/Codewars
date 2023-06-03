use std::collections::HashMap;

fn order(sentence: &str) -> String {
    let mut result_dict: HashMap<usize, String> = HashMap::new();

    for word in sentence.split_whitespace() {
        let tmp: String = word.chars().filter(|ch| ch.is_digit(10)).collect();
        let key: usize = tmp.parse().unwrap();

        result_dict.insert(key, word.to_string());
    }

    let mut v: Vec<(usize, String)> = result_dict.into_iter().collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));

    let result: String = v.into_iter().map(|(_, s)| s).collect::<Vec<String>>().join(" ");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
