fn maskify(cc: &str) -> String {
    let tmp = if cc.len() > 4 {
        cc.len() - 4
    } else {
        0
    };

    let last = &cc[tmp..];
    let prefix = "#".repeat(tmp);

    [prefix, last.to_string()].concat()
}


#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}