use std::collections::HashMap;

pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len() == 0 || list_cat.len() == 0 {
        return String::new();
    }

    let capacities = list_art
        .iter()
        .map(|e| e.split_once(' ').unwrap())
        .map(|(c, n)| (c.chars().nth(0).unwrap(), n.parse::<u32>().unwrap()))
        .fold(HashMap::<char, u32>::new(), |mut m, (c, n)| {
            m.entry(c).and_modify(|cnt| *cnt = *cnt + n).or_insert(n);
            m
        });

    list_cat
        .iter()
        .map(|c| c.chars().nth(0).unwrap())
        .map(|c| format!("({} : {})", c, capacities.get(&c).unwrap_or(&0)))
        .collect::<Vec<_>>()
        .join(" - ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }
}
