use structopt::StructOpt;
use read_input::prelude::*;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "d", long = "digit", default_value="3")]
    digit: i8,
    #[structopt(long = "debug")]
    debug: bool,
    #[structopt(short = "a", long="auto")]
    auto: bool
}

struct GuessRatings {
    guess: i64,
    perfect: i8,
    good: i8,
    bad: i8
}

fn main() {
    let args = Cli::from_args();
    if args.digit <= 0 {
        eprintln!("The value passed to the digit argument must be greater than zero.");
        std::process::exit(1);
    }

    let secret_number_min : i64 = i64::pow(10, args.digit as u32 - 1);
    let secret_number_max : i64 = i64::pow(10, args.digit as u32) - 1;
    let secret_number = rand::thread_rng()
        .gen_range(secret_number_min, secret_number_max);
    let mut guesses : Vec<GuessRatings> = Vec::new();

    if args.debug {
        dbg!(&args);
        dbg!(secret_number_min);
        dbg!(secret_number_max);
        dbg!(secret_number);
    }

    loop {
        let guess = make_guess(secret_number_min, secret_number_max, &guesses, &args);
        let guess_rating = rate_guess(guess, secret_number);

        if args.debug {
            dbg!(guess);
        }

        println!("{} Perfect", guess_rating.perfect);
        println!("{} Good", guess_rating.good);
        println!("{} Bad", guess_rating.bad);

        if guess_rating.perfect == args.digit as i8 {
            if args.auto {
                println!("The CPU won!");
            } else {
                println!("You win!");
            }
            break;
        } else {
            if args.auto {
                println!("The CPU didn't win!");
            } else {
                println!("You didn't win, try again!");
            }
            guesses.push(guess_rating);
        }
    }
}

fn make_guess(secret_number_min: i64, secret_number_max: i64, guesses: &Vec<GuessRatings>, args: &Cli) -> i64 {
    if args.auto {
        return cpu_guess(secret_number_min, secret_number_max, guesses, args);
    }
    return person_guess(secret_number_min, secret_number_max, args);
}

fn person_guess(secret_number_min: i64, secret_number_max: i64, args: &Cli) -> i64 {
    input()
        .inside_err(secret_number_min..=secret_number_max, format!("Your guess must have {} digits.  Try again!", args.digit))
        .msg("What is your guess? ").get()
}

fn cpu_guess(secret_number_min: i64, secret_number_max: i64, guesses: &Vec<GuessRatings>, args: &Cli) -> i64 {
    let guess = rand::thread_rng()
        .gen_range(secret_number_min, secret_number_max);
    println!("The CPU guesses . . . {}", guess);
    guess
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
        guess,
        perfect: perfect_count,
        good: good_count,
        bad: bad_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_perfect() {
        let ratings = rate_guess(123, 123);
        assert_eq!(ratings.perfect, 3);
        assert_eq!(ratings.good, 0);
        assert_eq!(ratings.bad, 0);
    }

    #[test]
    fn test_one_perfect_two_good() {
        let ratings = rate_guess(102, 120);
        assert_eq!(ratings.perfect, 1);
        assert_eq!(ratings.good, 2);
        assert_eq!(ratings.bad, 0);
    }

    #[test]
    fn test_three_good() {
        let ratings = rate_guess(132, 321);
        assert_eq!(ratings.perfect, 0);
        assert_eq!(ratings.good, 3);
        assert_eq!(ratings.bad, 0);
    }

    #[test]
    fn test_three_bad() {
        let ratings = rate_guess(132, 999);
        assert_eq!(ratings.perfect, 0);
        assert_eq!(ratings.good, 0);
        assert_eq!(ratings.bad, 3);
    }

    #[test]
    fn test_one_perfect_two_bad() {
        let ratings = rate_guess(132, 199);
        assert_eq!(ratings.perfect, 1);
        assert_eq!(ratings.good, 0);
        assert_eq!(ratings.bad, 2);
    }
}