use std::collections::HashSet;
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
    let mut freq = 0;
    for line in input.lines() {
        let change = line.parse::<i32>()?;
        freq += change;
    }
    writeln!(io::stdout(), "p1: {}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();

    // We are starting from a frequency of zero, aka. we have seen zero.
    seen.insert(0);

    for line in input.lines().cycle() {
        let change = line.parse::<i32>()?;
        freq += change;
        if seen.contains(&freq) {
            writeln!(io::stdout(), "p2: {}", freq)?;
            return Ok(());
        }
        seen.insert(freq);
    }

    Ok(())
}
