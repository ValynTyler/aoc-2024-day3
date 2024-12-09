use regex::{Error, Regex};

#[derive(Debug)]
enum ProcessedInput {
    Mul(i32, i32),
    Ok,
    No,
}

fn main() -> Result<(), Error> {
    let puzzle_input = include_str!("../../input/instructions.txt");

    let num = r".\d*";
    let mul = format!(r"mul\({num},{num}\)");

    let ok = r"do\(\)";
    let no = r"don't\(\)";

    let re = Regex::new(&format!(r"{mul}|{ok}|{no}"))?;

    let processed_input: Vec<_> = re
        .captures_iter(puzzle_input)
        .map(|captures| {
            let (sub, []) = captures.extract();
            match sub {
                "do()" => ProcessedInput::Ok,
                "don't()" => ProcessedInput::No,
                _s => {
                    let mul_re = Regex::new(&format!(r"mul\(({num}),({num})\)")).unwrap();
                    if let Some(cap) = mul_re.captures(sub) {
                        let (_, [lhs, rhs]) = cap.extract();
                        ProcessedInput::Mul(
                            lhs.trim().parse().unwrap(), 
                            rhs.trim().parse().unwrap(),
                        )
                    } else {
                        panic!()
                    }
                }
            }
        })
        .collect();

    let mut sum = 0;
    let mut flag = 1;
    for item in processed_input {
        match item {
            ProcessedInput::Ok => flag = 1,
            ProcessedInput::No => flag = 0,
            ProcessedInput::Mul(lhs, rhs) => {
                println!("{:?} {}", item, flag);
                sum += lhs * rhs * flag;
            }
        }
    }
    println!("{sum}");

    Ok(())
}
