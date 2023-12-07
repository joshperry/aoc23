use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let file = read_to_string("two/input")
        .expect("could not read input file");

    let gamere = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let trialre = Regex::new(r"[0-9]+ (red|green|blue)").unwrap();

    let possiblesum: isize = file.lines()
        // Parse out the game and trials
        .map(|l| gamere.captures(l).unwrap() )
        .map(|c| (
            // Get the game number
            c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            // All trials in the game
            trialre.find_iter(c.get(2).unwrap().as_str())
                // Parse the number and color of each
                .map(|t|
                    match t.as_str().split(' ').collect::<Vec<_>>()[..] {
                        [count, color] => (count.parse::<isize>().unwrap(), color),
                        _ => (0, "none"),
                    }
                )
                // Filter out "possible" trials
                .filter(|t| match t {
                    (n, "red") => *n > 12,
                    (n, "green") => *n > 13,
                    (n, "blue") => *n > 14,
                    _ => false,
                })
                // Count of impossible trials
                .count()
        ))
        // keep games with no impossible trials
        .filter(|g| g.1 == 0)
        // game number
        .map(|g| g.0 )
        // sum them
        .sum();

    println!("The possibilities are endless, well kinda: {}", possiblesum);

    let power: isize = file.lines()
        // Parse out the game and trials
        .map(|l|
            trialre
                .find_iter(l)
                // Parse the number and color of each
                .map(|t|
                    match t.as_str().split(' ').collect::<Vec<_>>()[..] {
                        [count, color] => (count.parse::<isize>().unwrap(), color),
                        _ => (0, "none"),
                    }
                )
                // Fold into a tuple holding the largest count of each color
                .fold((0, 0, 0), |acc, t|
                    match t {
                        (n, "red") => if n > acc.0 { (n, acc.1, acc.2) } else { acc },
                        (n, "green") => if n > acc.1 { (acc.0, n, acc.2) } else { acc },
                        (n, "blue") => if n > acc.2 { (acc.0, acc.1, n) } else { acc },
                        _ => acc,
                    }
                )
        )
        // Product of largest counts of each color per game
        .map(|acc| acc.0 * acc.1 * acc.2)
        // Sum of products
        .sum();

    println!("This isn't even my final form: {}", power);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_lines() {
        let file = read_to_string("input")
            .expect("could not read input file");
        assert_eq!(file.lines().count(), 100);
    }
}
