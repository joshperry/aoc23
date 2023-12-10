use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let numre = Regex::new(r"[0-9]+").unwrap();

    let file = read_to_string("three/input")
        .expect("could not read input file");

    // Line endings mean nothing
    let blob: String = file
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    // rust can't index into strings since they're utf8 (wat?!?); the input is only ascii, so
    // create a vew of it as bytes (non-allocating)
    let bblob = blob.as_bytes();

    let numsums: usize = numre
        // Find all contiguous number spans
        .find_iter(&blob)
        // Get the start, end, and contents of the span
        .map(|m| (m.start(), m.end(), m.as_str().parse::<usize>().unwrap()))
        .map(|m| (
            [
                // previous line's span sized -1 & +1 (each "line" is 140 chars)
                if m.0 >= 141 { bblob.get(m.0 - 141..m.1 - 139) } else { None },
                // one char before
                if m.0 > 0 { bblob.get(m.0 - 1..m.0) } else { None },
                // one char after
                if m.1 < bblob.len() { bblob.get(m.1..m.1 + 1) } else { None },
                // following line's span sized -1 & +1
                if m.1 <= bblob.len() - 141 { bblob.get(m.0 + 139..m.1 + 141) } else { None },
            ]
                .into_iter()
                // Unwrap `get`, with default to period (so it gets filtered)
                .map(|c| c.unwrap_or(&[46]))
                .flatten()
                // only non-number, non-period, characters
                .filter(|c| **c != 46 && (**c < 48 || **c > 57) )
                // we only care if there are any, not what they are
                .count(),
            // Found number
            m.2,
        ))
        // only "part numbers" adjacent to a symbol
        .filter(|m| m.0 > 0 )
        // get sum of part numbers
        .map(|m| m.1)
        .sum();

    println!("Pinnacle of German engineering: {:?}", numsums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_lines() {
        let file = read_to_string("input")
            .expect("could not read input file");
        assert_eq!(file.lines().count(), 140);
    }

    #[test]
    fn removes_whitespace() {
        let file = read_to_string("input")
            .expect("could not read input file");
        let chars = file
            .chars()
            .filter(|c| !c.is_whitespace());
        assert_eq!(chars.count(), 19600);
    }
}
