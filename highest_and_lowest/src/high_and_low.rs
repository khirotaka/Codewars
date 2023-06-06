fn high_and_low(numbers: &str) -> String {
    let number_int: Vec<i64> = numbers.split_whitespace().map(|x| {x.parse().unwrap()}).collect();
    let max = number_int.iter().max().unwrap();
    let min = number_int.iter().min().unwrap();

    format!("{} {}", max, min).to_string()
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
