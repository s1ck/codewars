pub fn unique_in_order_vec<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut vec = sequence.into_iter().collect::<Vec<_>>();
    vec.dedup();
    vec
}

pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence
        .into_iter()
        .fold(vec![], |mut agg, next| match &agg[..] {
            [.., last] if last.eq(&next) => agg,
            _ => {
                agg.push(next);
                agg
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }
}
