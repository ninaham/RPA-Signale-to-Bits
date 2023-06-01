use std::{
    fs::{self},
    io::Error,
};

use clap::{arg, Command};

fn main() {
    let cli = load_program_data();
    let args = cli.get_matches();

    match read_input(
        args.get_one::<String>("file")
            .expect("missing Input file. Try --help"),
    ) {
        Err(e) => println!("{}", e),
        Ok(s) => match args
            .get_one::<String>("veclen")
            .unwrap_or(&"32".to_string())
            .as_str()
        {
            "16" => s.iter().for_each(|seq| println!("{seq:016b}")),
            "32" => s.iter().for_each(|seq| println!("{seq:032b}")),
            "48" => s.iter().for_each(|seq| println!("{seq:048b}")),
            "64" => s.iter().for_each(|seq| println!("{seq:064b}")),
            _ => println!(
                "{}",
                Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Vec length should be 8, 16, 32, 48 or 64"
                )
            ),
        },
    };
}

fn load_program_data() -> Command {
    Command::new("RPA: Sigs to Bits")
        .version("0.1.0")
        .about("Konvertiert eine Reihe selbst definierter Signale zu einer Bitfolge. Input Format siehe README.")
        .args([
            arg!(veclen: -l --veclen <length> "Gibt die Länge des Outputvektors an. Mögliche Werte: 12, 32, 48, 64"),
            arg!(file: <file> "Inputdatei")
        ])
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
    if input.lines().count() < 1 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "no signals provided",
        ));
    }
    let sigs = parse_signals(input.lines().next().unwrap());

    input
        .lines()
        .skip(2)
        .filter(|l| l.split(' ').next().unwrap() != "c")
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
