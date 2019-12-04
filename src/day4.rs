use std::io::{stdin, Read};

pub fn run() {
    let input = parse_input();
    let mut valid_passwords = 0;
    for i in input.min..input.max {
        let (d1, d2, d3, d4, d5, d6) = (
            i / 100_000,
            (i / 10_000) % 10,
            (i / 1_000) % 10,
            (i / 100) % 10,
            (i / 10) % 10,
            i % 10,
        );

        let are_digits_consecutive = (d1 == d2 && d3 != d2)
            || (d2 == d3 && d4 != d3 && d1 != d3)
            || (d3 == d4 && d5 != d4 && d2 != d4)
            || (d4 == d5 && d6 != d5 && d3 != d5)
            || (d5 == d6 && d4 != d5);
        if !are_digits_consecutive {
            continue;
        }

        let are_digits_monotonic = d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6;
        if !are_digits_monotonic {
            continue;
        }

        valid_passwords = valid_passwords + 1;
    }

    println!("{}", valid_passwords);
}

fn parse_input() -> Input {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");
    let input_string = std::str::from_utf8(&s).expect("invalid utf8").trim();

    if let [min_str, max_str] = input_string.split("-").collect::<Vec<&str>>()[0..2] {
        let min = min_str.parse::<u64>().expect("cannot parse minimum");
        let max = max_str.parse::<u64>().expect("cannot parse maximum");

        Input { min, max }
    } else {
        panic!("cannot parse input")
    }
}

#[derive(Debug)]
struct Input {
    min: u64,
    max: u64,
}
