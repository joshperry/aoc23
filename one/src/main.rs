use aoclib::read_lines;
use std::collections::HashMap;

fn extract_digits(line: &String) -> Vec<(usize, &str)> {
    line
        .match_indices(|c: char| c.is_digit(10))
        .collect()
}

fn extract_spells(line: &String) -> Vec<(usize, &str)> {
    let spells = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    spells
        .iter()
        .map(|spell|
            line
                .match_indices(spell.0)
                .map(|m| (m.0, *spells.get(m.1).unwrap()))
        )
        .flatten()
        .collect()
}

fn extract_posvalue(chars: Vec<(usize, &str)>) -> i32 {
    let mut schars = chars.clone();
    schars
        .sort_by(|a, b| a.0.cmp(&b.0));
    let digits = schars
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

fn extract_mixed(line: &String) -> Vec<(usize, &str)> {
    let mut digits = extract_spells(&line);
    digits.extend(extract_digits(&line));
    digits
}

fn extract_mixed_values(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(extract_mixed)
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

    let values = extract_mixed_values(&lines);
    let sum:i32 = values
        .into_iter()
        .sum();
    println!("calibration time, again: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_digit_value(line: &String) -> i32 {
        let digits = extract_digits(&line);
        extract_posvalue(digits)
    }

    fn extract_spell_value(line: &String) -> i32 {
        let digits = extract_spells(&line);
        extract_posvalue(digits)
    }

    fn extract_mixed_value(line: &String) -> i32 {
        let mut digits = extract_spells(&line);
        digits.extend(extract_digits(&line));
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

    #[test]
    fn extracts_single_spell_value() {
        let value = extract_spell_value(&String::from("abonecde"));
        assert_eq!(value, 11);
    }

    #[test]
    fn extracts_multi_spell_value() {
        let value = extract_spell_value(&String::from("abonecdetwofgh"));
        assert_eq!(value, 12);
    }

    #[test]
    fn extracts_overlap_spell_value() {
        let value = extract_spell_value(&String::from("aboneightfgh"));
        assert_eq!(value, 18);
    }

    #[test]
    fn extracts_unordered_overlap_spell_value() {
        let value = extract_spell_value(&String::from("abtwoneightfgh"));
        assert_eq!(value, 28);
    }

    #[test]
    fn extracts_mixed_value() {
        let value = extract_mixed_value(&String::from("ab1cdetwofgh"));
        assert_eq!(value, 12);
    }
}
