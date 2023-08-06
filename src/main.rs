use core::time::Duration;
use rand::random;
use clap::Parser;
use std::io::Write;

#[derive(Parser)]
struct Args {
    sizes: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    if args.sizes.len() != 0 {
        eprintln!("`sizes` should have a number. Use --help for more information.");
    };

    roll_dice(&*args.sizes);
}

fn roll_dice(sizes: &[usize]) {
    use std::time::Instant;

    let mut rolls = Vec::with_capacity(sizes.len());
    let one_second = Duration::from_secs(1);
    let mut stdout = std::io::stdout().lock();

    let now = Instant::now();

    while now.elapsed() < one_second {
        rolls.clear();

        for die in sizes.iter() {
            rolls.push(random::<usize>() % die + 1); // you can do a better randomizer here
        }

        write_dice_faces(&*rolls, &mut stdout).unwrap();
    }
}

fn write_dice_faces(rolls: &[usize], w: &mut impl Write) -> std::io::Result<()> {
    for roll in rolls.iter() {
        write!(w, "{}  ", roll)?;
    }

    w.flush()
}
