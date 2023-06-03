fn validate_pin(pin: &str) -> bool {
    let mut flag: bool = false;
    let mut pin_v: Vec<usize> = vec![];
    for p in pin.chars() {
        match p.to_string().parse::<usize>() {
            Ok(v) => {
                pin_v.push(v);
            },
            Err(_) => {
                flag = true;
            },
        }
    }

    if flag {
        false
    }
    else {
        let pin_v_len = pin_v.len();
        println!("{:?}", pin_v);
        if pin_v_len == 4 || pin_v_len == 6 {
            true
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validate_pin;

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }

    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
        assert_eq!(validate_pin("+111"), false);
        assert_eq!(validate_pin("1234x"), false);

    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
}