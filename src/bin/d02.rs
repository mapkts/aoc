use std::collections::HashMap;
use std::io::{self, Read, Write};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let (mut twos, mut threes) = (0, 0);

    for line in input.lines() {
        let mut counts = HashMap::new();
        for c in line.chars() {
            counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        if counts.values().any(|&c| c == 2) {
            twos += 1
        }
        if counts.values().any(|&c| c == 3) {
            threes += 1
        }
    }

    writeln!(io::stdout(), "p1: {}", twos * threes)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ids: Vec<&str> = input.lines().collect();
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            if let Some(common) = find_common_letters(&ids[i], &ids[j]) {
                writeln!(io::stdout(), "p2: {}", common)?;
                return Ok(());
            }
        }
    }

    // Should not reach here if input data is valid.
    Err(From::from("invalid input data"))
}

fn find_common_letters(s1: &str, s2: &str) -> Option<String> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut found_one_differ = false;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if found_one_differ {
                return None;
            }
            found_one_differ = true;
        }
    }

    Some(
        s1.chars()
            .zip(s2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}
