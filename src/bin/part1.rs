use regex::{Error, Regex};

fn main() -> Result<(), Error> {
    let puzzle_input = include_str!("../../input/instructions.txt");
    let re = Regex::new(r"mul\((.\d*),(.\d*)\)")?;

    let operands: Vec<_> = re
        .captures_iter(puzzle_input)
        .map(|captures| {
            let (_, [lhs, rhs]) = captures.extract();
            let (lhs, rhs) = (lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap());
            (lhs, rhs)
        })
        .collect();

    let mut sum = 0;
    for item in operands {
        println!("mul({}, {});", item.0, item.1);
        sum += item.0 * item.1;
    }
    println!("{}", sum);

    Ok(())
}
