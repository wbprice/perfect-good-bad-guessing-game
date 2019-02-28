use structopt::StructOpt;
use read_input::prelude::*;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "d", long = "digit")]
    digit: i8
}

fn main() {
    let args = Cli::from_args();
    dbg!(args.digit);

    let guess : i64 = input().msg("What is your guess?").get();
    let secret_number_min = 100;
    let secret_number_max = 999;
    let secret_number = rand::thread_rng()
        .gen_range(secret_number_min, secret_number_max);

    let secret_number_string = secret_number.to_string();
    let guess_string = guess.to_string();

    let mut perfect_count = 0;
    let mut good_count = 0;
    let mut bad_count = 0;

    dbg!(guess);
    dbg!(secret_number);

    for (guess_index, guess_integer) in guess_string.chars().enumerate() {
        for (secret_number_index, secret_number_integer) in secret_number_string.chars().enumerate() {
            if guess_integer == secret_number_integer {
                if guess_index == secret_number_index {
                    perfect_count = perfect_count + 1;
                } else {
                    good_count = good_count + 1;
                }
            } else {
                bad_count = bad_count + 1;
            }
        }
    }

    dbg!(perfect_count);
    dbg!(good_count);
    dbg!(bad_count);
}

