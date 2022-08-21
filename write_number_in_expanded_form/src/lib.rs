pub fn expanded_form_part_1(n: u64) -> String {
    n.to_string()
        .bytes()
        .rev()
        .map(|b| (b - b'0') as u64)
        .zip(0..)
        .fold(vec![], |mut nums, (b, exp)| {
            if b > 0 {
                nums.push((b * 10_u64.pow(exp)).to_string());
            }
            nums
        })
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" + ")
}

pub fn expanded_form_part_2(num: f64) -> String {
    if let Some((_, decimals)) = num.to_string().split_once('.') {
        let decimals = decimals
            .bytes()
            .map(|b| (b - b'0') as u8)
            .zip(1..)
            .fold(vec![], |mut nums, (b, exp)| {
                if b > 0 {
                    nums.push(format!("{}/{}", b, 10_u64.pow(exp)));
                }
                nums
            })
            .into_iter()
            .collect::<Vec<_>>()
            .join(" + ");

        format!(
            "{} + {}",
            expanded_form_part_1(num.floor() as u64),
            decimals
        )
    } else {
        expanded_form_part_1(num as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form_part_1(12), "10 + 2");
        assert_eq!(expanded_form_part_1(42), "40 + 2");
        assert_eq!(expanded_form_part_1(70304), "70000 + 300 + 4");
    }

    #[test]
    fn test_add() {
        assert_eq!(
            expanded_form_part_2(1568.156),
            "1000 + 500 + 60 + 8 + 1/10 + 5/100 + 6/1000"
        );
        assert_eq!(
            expanded_form_part_2(1278.8766),
            "1000 + 200 + 70 + 8 + 8/10 + 7/100 + 6/1000 + 6/10000"
        );
        assert_eq!(
            expanded_form_part_2(4982.342),
            "4000 + 900 + 80 + 2 + 3/10 + 4/100 + 2/1000"
        );
    }
}
