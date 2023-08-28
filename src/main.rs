use core::time::Duration;
use std::io::Write;
use std::fmt::Write as _;

use clap::Parser;
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

        let text = write_dice_faces(&*rolls);

        // write to stdout and flush
        write!(&mut stdout, "\x1B[2J\x1B[1;1H{}", text);
        stdout.flush();
    }
}

fn write_dice_faces(
    rolls: &[usize],
) -> String {
    let mut buffer = String::new();

    buffer += "<";
    for roll in rolls.iter() {
        write!(&mut buffer, "{} ", roll).unwrap();
    }

    if !rolls.is_empty() {
        buffer.pop();
    }

    buffer += ">";

    text_to_ascii_art::convert(buffer).unwrap()
}
