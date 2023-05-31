use std::{
    env,
    fs::{self},
    io::Error,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!(
            "{}",
            Error::new(std::io::ErrorKind::InvalidInput, "no input file specified",)
        );
    }
    match read_input(args[1].as_str()) {
        Err(e) => println!("{}", e),
        Ok(s) => {
            s.iter().for_each(|seq| println!("{seq:032b}"));
        }
    };
}

fn read_input(file: &str) -> Result<Vec<usize>, Error> {
    match fs::read_to_string(file) {
        Err(x) => Err(x),
        Ok(input) => parse_input(&input),
    }
}

fn parse_signals(sigs: &str) -> Vec<&str> {
    sigs.split(' ').collect()
}

fn parse_input(input: &str) -> Result<Vec<usize>, Error> {
    let sigs = parse_signals(input.lines().next().unwrap());

    input
        .lines()
        .skip(2)
        .map(|l| parse_signal_sequence(l, &sigs))
        .map(|l| match l {
            Ok(s) => Ok(process_signal_sequence(s)),
            Err(e) => Err(e),
        })
        .collect()
}

fn parse_signal_sequence(input: &str, sigs: &[&str]) -> Result<Vec<usize>, Error> {
    input
        .split(' ')
        .map(|s| sigs.iter().position(|s1| *s1 == s))
        .map(|o| match o {
            Some(p) => Ok(p),
            None => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "unknown signal",
            )),
        })
        .collect()
}

fn process_signal_sequence(input: Vec<usize>) -> usize {
    let mut sequence: usize = 0;

    input.iter().for_each(|b| sequence |= 1 << b);

    sequence
}
