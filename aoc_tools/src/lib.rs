pub mod input;

#[cfg(test)]
mod tests {
    use input::input::read_lines;

    use super::*;

    #[test]
    fn test_read_lines_strings() {
        let expected_vec: Vec<String> = ["a", "b", "c", "d", "e", "f", "g", "h"]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let fn_out = read_lines::<String>("test_files\\test_str.txt").unwrap();
        assert_eq!(fn_out, expected_vec);
    }
}
