fn solution(s: &str) -> String {
    let mut result_vec: Vec<String> =  vec![];
    for i in s.chars() {
        if i.is_uppercase() {
            result_vec.push(format!(" {}", i).to_string())
        }
        else {
            result_vec.push(i.to_string());
        }
    }
    result_vec.join("")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
