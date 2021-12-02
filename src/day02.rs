fn getPositionAndResult() -> (u32, u32, u32) {
    (1, 2, 3)
}

#[cfg(test)]
#[test]
fn test_count_increases() {
    let count = count_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    assert_eq!(7, count);
}
