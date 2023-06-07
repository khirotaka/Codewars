fn as_usize_vec(isbn: &str) -> Result<Vec<usize>, &str> {
    let mut result: Vec<usize> = vec![];

    if isbn.len() != 10 {
        return Err("Bad Input")
    }
    else {
        for (idx, s) in isbn.chars().enumerate() {
            if idx != 9 {
                if s.is_ascii_digit() {
                    result.push(s.to_string().parse::<usize>().unwrap());
                }
                else {
                    return Err("Bad Input")
                }
            }
            else {
                if s.is_ascii_digit() {
                    result.push(s.to_string().parse::<usize>().unwrap());
                }
                else if s == 'X' {
                    result.push(10);
                }
                else {
                    return Err("Bad Input")
                }
            }
        }
    }

    Ok(result)
}

fn valid_isbn10(isbn: &str) -> bool {
    let usize_isbn = match as_usize_vec(isbn) {
        Ok(v) => v,
        Err(_) => {
            return false
        }
    };

    let position: Vec<usize> = (1..11).collect();

    let flag = usize_isbn.clone().into_iter().zip(position).map(|(lhs, rhs)| lhs * rhs).sum::<usize>() % 11;

    flag == 0
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(actual == expected, "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}
