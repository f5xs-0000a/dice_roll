use core::time::Duration;
use std::{
    fmt::Write as _,
    io::Write,
};

use clap::Parser;
use itertools::Itertools as _;
use rand::random;

#[derive(Parser)]
struct Args {
    sizes: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    if args.sizes.len() != 0 {
        eprintln!(
            "`sizes` should have a number. Use --help for more information."
        );
    };

    roll_dice(&*args.sizes);
}

fn roll_dice(sizes: &[usize]) {
    use std::time::Instant;

    let mut rolls = Vec::with_capacity(dbg!(sizes).len());
    let one_second = Duration::from_secs(1);
    let mut stdout = std::io::stdout().lock();
    let mut buffer = String::new();

    let now = Instant::now();

    while now.elapsed() < one_second {
        rolls.clear();
        buffer.clear();

        // roll dice
        for die in sizes.iter() {
            // you can do a better randomizer here
            rolls.push(random::<usize>() % die + 1);
        }

        let text = write_dice_faces(&*rolls, sizes);

        // write to stdout and flush
        write!(&mut stdout, "\x1B[2J\x1B[1;1H{}", text).unwrap();
        stdout.flush().unwrap();
    }
}

fn write_dice_faces(
    rolls: &[usize],
    die_size: &[usize],
) -> String {
    let mut buffer = String::new();

    buffer += "<";
    for (roll, die_size) in rolls.iter().zip_eq(die_size.iter()) {
        let padding = determine_digits(*die_size);
        write!(&mut buffer, "{:0>padding$} ", roll).unwrap();
    }

    if !rolls.is_empty() {
        buffer.pop();
    }

    buffer += ">";

    text_to_ascii_art::convert(buffer).unwrap()
}

fn determine_digits(mut x: usize) -> usize {
    let mut digits = 1;

    while 10 <= x {
        digits += 1;
        x /= 10;
    }

    digits
}
