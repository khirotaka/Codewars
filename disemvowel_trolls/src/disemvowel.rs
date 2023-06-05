fn disemvowel(s: &str) -> String {
    let vowel: &[&str; 5] = &["a", "i", "u", "e", "o"];
    let mut s_str = s.to_string();
    for v in vowel {
        s_str = s_str.replace(v, "").replace(v.to_uppercase().as_str(), "");
    }
    s_str
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}