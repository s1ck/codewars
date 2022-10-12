pub fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.sort_unstable_by_key(|i| i.abs());
    b.sort_unstable_by_key(|i| i.abs());
    a.iter().zip(b.iter()).all(|(l, r)| l * l == *r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }

    #[test]
    fn tests_comp() {
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 11,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 21,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing(a1, a2, false);
    }
}
