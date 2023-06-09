fn exp_sum(n: u64) -> u64 {
    let mut table = vec![1; (n + 1) as usize];
    for k in 2..(n+1) {
        for m in k..(n+1) {
            table[m as usize] += table[(m - k) as usize]
        }
    }

    table[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sample_tests() {
        assert_eq!(exp_sum(1), 1);
        assert_eq!(exp_sum(2), 2);
        assert_eq!(exp_sum(3), 3);
        assert_eq!(exp_sum(4), 5);
        assert_eq!(exp_sum(5), 7);
        assert_eq!(exp_sum(10), 42);
    }
}
