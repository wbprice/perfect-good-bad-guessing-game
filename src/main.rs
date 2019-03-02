use structopt::StructOpt;
use read_input::prelude::*;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "d", long = "digit", default_value="3")]
    digit: i8,
    #[structopt(long = "debug", default_value="false")]
    debug: bool
}

struct GuessRatings {
    perfect: i8,
    good: i8,
    bad: i8
}

fn main() {
    let args = Cli::from_args();

    let secret_number_min : i64 = i64::pow(10, args.digit as u32 - 1);
    let secret_number_max : i64 = i64::pow(10, args.digit as u32) - 1;
    let secret_number = rand::thread_rng()
        .gen_range(secret_number_min, secret_number_max);

    if args.debug {
        dbg!(secret_number_min);
        dbg!(secret_number_max);
        dbg!(secret_number);
    }

    loop {
        let guess : i64 = input().msg("What is your guess? ").get();
        let guess_ratings = rate_guess(guess, secret_number);

        if args.debug {
            dbg!(guess);
        }

        println!("{} Perfect", guess_ratings.perfect);
        println!("{} Good", guess_ratings.good);
        println!("{} Bad", guess_ratings.bad);

        if guess_ratings.perfect == args.digit as i8 {
            println!("You win!");
            break;
        } else {
            println!("You didn't win, try again!");
        }
    }
}


fn rate_guess(guess: i64, secret: i64) -> GuessRatings {
    let secret_string = secret.to_string();
    let secret_string_vector : Vec<_> = secret_string.chars().collect();
    let guess_string = guess.to_string();

    let mut perfect_count = 0;
    let mut good_count = 0;
    let mut bad_count = 0;

    for (guess_index, guess_integer) in guess_string.chars().enumerate() {
        if guess_integer == secret_string_vector[guess_index] {
            perfect_count += 1;
        } else if secret_string_vector.contains(&guess_integer) {
            good_count += 1;
        } else {
            bad_count += 1;
        }
    }

    GuessRatings {
        perfect: perfect_count,
        good: good_count,
        bad: bad_count
    }
}