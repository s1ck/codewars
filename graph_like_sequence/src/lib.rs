pub fn solution(mut degrees: Vec<usize>) -> bool {
    fn havel_hakimi(degrees: &mut [usize]) -> bool {
        // sort in descending order
        degrees.sort_unstable_by(|a, b| a.cmp(b).reverse());

        if degrees.is_empty() || degrees[0] == 0 {
            return true;
        }

        let head = degrees[0];

        if head >= degrees.len() {
            return false;
        }

        // decrease degree of `head` many following nodes
        for e in &mut degrees[1..head + 1] {
            // if we underflow there were not enough nodes left to connect to
            if let Some(x) = e.checked_sub(1) {
                *e = x;
            } else {
                return false;
            }
        }

        havel_hakimi(&mut degrees[1..])
    }

    havel_hakimi(&mut degrees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solution(vec![]), true);
        assert_eq!(solution(vec![0]), true);
        assert_eq!(solution(vec![2]), false);
        assert_eq!(solution(vec![1, 1]), true);
        assert_eq!(solution(vec![1, 1, 1]), false);
        assert_eq!(solution(vec![0, 0, 0]), true);
        assert_eq!(solution(vec![2, 2, 2]), true);
        assert_eq!(solution(vec![1, 2, 0]), false);
        assert_eq!(solution(vec![1, 2, 1]), true);
        assert_eq!(solution(vec![5, 5, 4, 3, 2, 2, 2, 1]), true);
        assert_eq!(solution(vec![5, 3, 5, 5, 2, 2, 1, 1]), true);
        assert_eq!(solution(vec![5, 5, 5, 4, 2, 1, 1, 1]), false);
        assert_eq!(solution(vec![1, 1, 1, 4, 2, 3, 1, 3, 1, 1]), true);
        assert_eq!(solution(vec![1, 1, 10, 4, 2, 3, 1, 3, 1, 1]), false);
    }
}
