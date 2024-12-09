use regex::{Error, Regex};

fn main() -> Result<(), Error> {
    let puzzle_input = include_str!("../../input/example2.txt");

    let num_re = r".\d*";
    let mul_re = format!(r"mul\({num_re},{num_re}\)");

    let ok_re = r"do\(\)";
    let no_re = r"don't\(\)";

    let re = Regex::new(&format!(r"{mul_re}|{ok_re}|{no_re}"))?;

    let operands: Vec<_> = re
        .captures_iter(puzzle_input)
        .map(|captures| {
            println!("{:?}", captures);
        })
        .collect();

    Ok(())
}
