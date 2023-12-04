use std::fs::read_to_string;
use std::collections::BTreeMap;
use phf::phf_map;

static SPELLS: phf::Map<&'static str, &'static str> = phf_map! {
    "one" => "1",
    "two" => "2",
    "three" => "3",
    "four" => "4",
    "five" => "5",
    "six" => "6",
    "seven" => "7",
    "eight" => "8",
    "nine" => "9",
};

fn calc_posvalue(matches: BTreeMap<usize, &str>) -> i32 {
    let first = matches.first_key_value().and_then(|x| Some(x.1));
    let last = matches.last_key_value().and_then(|x| Some(x.1));

    let value = match (first, last) {
        (Some(first), Some(last)) => format!("{first}{last}"),
        _ => 0.to_string(),
    };

    value.parse::<i32>().unwrap()
}

fn main() {
    let file = read_to_string("one/input").unwrap();
    let lines = file.lines();

    let sum:i32 = lines
        .clone() // just the iterator
        .map(|line|
            line
                // Collect all digit chars into a vector
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>()
        )
        // Get the calibration value for the line
        .map(|digits|
            match digits[..] {
                [] => 0.to_string(),
                [first] => format!("{first}{first}"),
                [first, .., last] => format!("{first}{last}"),
            }
            .parse::<i32>()
            .unwrap() // Safe as non-digits filtered
        )
        // Sum all the values
        .sum();
    println!("calibration time, come on: {sum}");

    let sum:i32 = lines
        .clone()
        .map(|line|
            line
                .match_indices(|c: char|  c.is_digit(10))
                .chain(
                    SPELLS
                        .keys() 
                        // For every spelled number
                        .map(|spell|
                            line
                                // Find all (position, spell) instances in the line
                                .match_indices(spell) 
                                // map to (position, digit)
                                .map(|m| (m.0, *SPELLS.get(m.1).unwrap())) 
                        )
                        .flatten() // flatten the list of match lists
                )
                .collect()
        )
        .map(calc_posvalue)
        .sum();
    println!("calibration time, again: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_digit_value(line: &str) -> i32 {
        let digits = line
            .match_indices(|c: char|  c.is_digit(10))
            .collect();
        calc_posvalue(digits)
    }

    fn extract_spells(line: &str) -> BTreeMap<usize, &str> {
        SPELLS
            .keys()
            .map(|spell|
                line
                    .match_indices(spell)
                    .map(|m| (m.0, *SPELLS.get(m.1).unwrap()))
            )
            .flatten()
            .collect()
    }

    fn extract_spell_value(line: &str) -> i32 {
        let digits = extract_spells(line);
        calc_posvalue(digits)
    }

    fn extract_mixed_value(line: &str) -> i32 {
        let digits = line
            .match_indices(|c: char|  c.is_digit(10))
            .chain(extract_spells(line))
            .collect();
        calc_posvalue(digits)
    }

    #[test]
    fn reads_lines() {
        let file = read_to_string("input").unwrap();
        let lines: Vec<&str> = file.lines().collect();
        assert_eq!(lines.len(), 1000);
    }

    #[test]
    fn calculates_digit_value() {
        let file = read_to_string("input").unwrap();
        let lines = file.lines();
        let value:i32 = lines
            .map(extract_digit_value)
            .sum();
        assert_eq!(value, 54304);
    }

    #[test]
    fn calculates_mixed_value() {
        let file = read_to_string("input").unwrap();
        let lines = file.lines();
        let value:i32 = lines
            .map(extract_mixed_value)
            .sum();
        assert_eq!(value, 54418);
    }

    #[test]
    fn extracts_multi_13_value() {
        let value = extract_digit_value("abc1efg2hij3klm");
        assert_eq!(value, 13);
    }

    #[test]
    fn extracts_multi_15_value() {
        let value = extract_digit_value("abc1efg3hij5klm");
        assert_eq!(value, 15);
    }

    #[test]
    fn extracts_double_value() {
        let value = extract_digit_value("abc1ef3g");
        assert_eq!(value, 13);
    }

    #[test]
    fn extracts_single_value() {
        let value = extract_digit_value("abc1efg");
        assert_eq!(value, 11);
    }

    #[test]
    fn extracts_no_value() {
        let value = extract_digit_value("abcefg");
        assert_eq!(value, 0);
    }

    #[test]
    fn extracts_single_spell_value() {
        let value = extract_spell_value("abonecde");
        assert_eq!(value, 11);
    }

    #[test]
    fn extracts_multi_spell_value() {
        let value = extract_spell_value("abonecdetwofgh");
        assert_eq!(value, 12);
    }

    #[test]
    fn extracts_overlap_spell_value() {
        let value = extract_spell_value("aboneightfgh");
        assert_eq!(value, 18);
    }

    #[test]
    fn extracts_unordered_overlap_spell_value() {
        let value = extract_spell_value("abtwoneightfgh");
        assert_eq!(value, 28);
    }

    #[test]
    fn extracts_mixed_value() {
        let value = extract_mixed_value("ab1cdetwofgh");
        assert_eq!(value, 12);
    }
}
