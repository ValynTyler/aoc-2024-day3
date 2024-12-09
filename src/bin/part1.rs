use regex::{Error, Regex};

fn main() -> Result<(), Error> {
    let puzzle_input = include_str!("../../input/example.txt");
    let re = Regex::new(r"mul\(.\d*,.\d*\)")?;

    for item in re.find_iter(puzzle_input) {
        println!("{}", item.as_str());
    }

    Ok(())
}
