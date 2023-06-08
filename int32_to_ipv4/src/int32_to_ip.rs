fn int32_to_ip(int: u32) -> String {
    let mut result: String = "".to_string();

    for i in (0..25).step_by(8).rev() {
        let tmp: u32 = (int >> i) & 0xff;
        result = format!("{}.{}", result, usize::from_str_radix(format!("{:08b}", tmp).as_str(), 2).unwrap().to_string());
    }

    result[1..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
