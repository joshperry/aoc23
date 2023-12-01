use std::fs::read_to_string;

fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn extract_value(line: &String) -> i32 {
    let digits = line
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>();
    let value = match digits[..] {
        [] => String::from("0"),
        [first] => format!("{first}{first}"),
        [first, .., last] => format!("{first}{last}"),
    };
    value.parse::<i32>().unwrap()
}

fn extract_values(lines: &Vec<String>) -> Vec<i32> {
    lines
        .into_iter()
        .map(extract_value)
        .collect()
}

fn main() {
    let lines = read_lines("one/input");
    let values = extract_values(&lines);
    let sum:i32 = values
        .into_iter()
        .sum();
    println!("calibration time, come on: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_lines() {
        let lines = read_lines("input");
        assert_eq!(lines.len(), 1000);
    }

    #[test]
    fn extracts_multi_13_value() {
        let value = extract_value(&String::from("abc1efg2hij3klm"));
        assert_eq!(value, 13);
    }

    #[test]
    fn extracts_multi_15_value() {
        let value = extract_value(&String::from("abc1efg3hij5klm"));
        assert_eq!(value, 15);
    }

    #[test]
    fn extracts_double_value() {
        let value = extract_value(&String::from("abc1ef3g"));
        assert_eq!(value, 13);
    }

    #[test]
    fn extracts_single_value() {
        let value = extract_value(&String::from("abc1efg"));
        assert_eq!(value, 11);
    }

    #[test]
    fn extracts_no_value() {
        let value = extract_value(&String::from("abcefg"));
        assert_eq!(value, 0);
    }
}
