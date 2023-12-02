use std::fs::read_to_string;

fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn extract_digits(line: &String) -> Vec<(usize, &str)> {
    line
        .match_indices(|c: char| c.is_digit(10))
        .collect()
}

fn extract_posvalue(chars: Vec<(usize, &str)>) -> i32 {
    let digits = chars
        .iter()
        .map(|c| c.1)
        .collect::<Vec<&str>>();
    let value = match digits[..] {
        [] => String::from("0"),
        [first] => format!("{first}{first}"),
        [first, .., last] => format!("{first}{last}"),
    };
    value.parse::<i32>().unwrap()
}

fn extract_digit_values(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(extract_digits)
        .map(extract_posvalue)
        .collect()
}

fn main() {
    let lines = read_lines("one/input");
    let values = extract_digit_values(&lines);
    let sum:i32 = values
        .into_iter()
        .sum();
    println!("calibration time, come on: {sum}");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn extract_digit_value(line: &String) -> i32 {
        let digits = extract_digits(&line);
        extract_posvalue(digits)
    }

    #[test]
    fn reads_lines() {
        let lines = read_lines("input");
        assert_eq!(lines.len(), 1000);
    }

    #[test]
    fn extracts_multi_13_value() {
        let value = extract_digit_value(&String::from("abc1efg2hij3klm"));
        assert_eq!(value, 13);
    }

    #[test]
    fn extracts_multi_15_value() {
        let value = extract_digit_value(&String::from("abc1efg3hij5klm"));
        assert_eq!(value, 15);
    }

    #[test]
    fn extracts_double_value() {
        let value = extract_digit_value(&String::from("abc1ef3g"));
        assert_eq!(value, 13);
    }

    #[test]
    fn extracts_single_value() {
        let value = extract_digit_value(&String::from("abc1efg"));
        assert_eq!(value, 11);
    }

    #[test]
    fn extracts_no_value() {
        let value = extract_digit_value(&String::from("abcefg"));
        assert_eq!(value, 0);
    }
}
