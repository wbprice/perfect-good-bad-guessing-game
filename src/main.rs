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

    // to implement logic for figuring out how many digits are in the guess number

    let guess : i64 = input().msg("What is your guess?").get();
    let secret_number_min = 100;
    let secret_number_max = 999;
    let secret_number = rand::thread_rng()
        .gen_range(secret_number_min, secret_number_max);

    dbg!(guess);
    dbg!(secret_number);    

    
}

