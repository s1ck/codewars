pub fn likes(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{a} likes this"),
        [a, b] => format!("{a} and {b} like this"),
        [a, b, c] => format!("{a}, {b} and {c} like this"),
        [a, b, r @ ..] => format!("{a}, {b} and {} others like this", r.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}
