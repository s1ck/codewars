mod solution {

    pub fn range_extraction(a: &[i32]) -> String {
        a[1..]
            .iter()
            .enumerate()
            .fold(
                (vec![], a[0], a[0]),
                |(mut ranges, range_start, prev), (i, curr)| {
                    if curr.abs_diff(prev) == 1 {
                        if i == a.len() - 2 {
                            // last element
                            ranges.push((range_start, *curr));
                        }
                        (ranges, range_start, *curr)
                    } else {
                        ranges.push((range_start, prev));
                        if i == a.len() - 2 {
                            // last element
                            ranges.push((*curr, *curr));
                        }
                        (ranges, *curr, *curr)
                    }
                },
            )
            .0
            .iter()
            .map(|(start, end)| {
                if start == end {
                    format!("{start}")
                } else if start.abs_diff(*end) == 1 {
                    format!("{start},{end}")
                } else {
                    format!("{start}-{end}")
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
