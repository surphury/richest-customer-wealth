#[cfg(test)]
use crate::maximum_wealth;

#[test]
fn test_1() {
    let result = maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    assert_eq!(result, 6);
}

#[test]
fn test_2() {
    let result = maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]);

    assert_eq!(result, 10);
}

#[test]
fn test_3() {
    let result = maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]);

    assert_eq!(result, 17);
}
