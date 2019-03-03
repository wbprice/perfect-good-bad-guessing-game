use structopt::StructOpt;
use read_input::prelude::*;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "d", long = "digit", default_value="3")]
    /// Sets the number of digits used for the secret number
    digit: i8,
    #[structopt(long = "debug")]
    /// Turns on debug logging
    debug: bool,
    #[structopt(short = "a", long="auto")]
    /// Asks the CPU to play itself
    auto: bool
}

struct GuessRatings {
    number: i64,
    perfect: i8,
    good: i8,
    bad: i8
}

#[derive(Default, Debug, Clone)]
struct NumberMemory {
    perfect: Vec<(i8, i8)>,
    good: Vec<(i8, i8)>,
    bad: Vec<i8>
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
                println!("The CPU won in {} guesses!", guesses.len());
            } else {
                println!("You won in {} guesses!", guesses.len());
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

fn make_guess(secret_number_min: i64, secret_number_max: i64, guesses: &[GuessRatings], args: &Cli) -> i64 {
    if args.auto {
        return cpu_guess(secret_number_min, secret_number_max, guesses);
    }
    person_guess(secret_number_min, secret_number_max, args)
}

fn person_guess(secret_number_min: i64, secret_number_max: i64, args: &Cli) -> i64 {
    input()
        .inside_err(secret_number_min..=secret_number_max, format!("Your guess must have {} digits.  Try again!", args.digit))
        .msg("What is your guess? ").get()
}

fn cpu_guess(secret_number_min: i64, secret_number_max: i64, guesses: &[GuessRatings]) -> i64 {
    cpu_naive_guess(secret_number_min, secret_number_max, guesses)
}

fn cpu_analyze_score(guess: GuessRatings, memory: &mut NumberMemory) {
    // If all three numbers were bad, none of them should be used in future guesses.
    if guess.bad == 3 {
        memory.bad.append(&mut split_number(guess.number))
    }
}

fn cpu_naive_guess(secret_number_min: i64, secret_number_max: i64, guesses: &[GuessRatings]) -> i64 {
    loop {
        let guess = rand::thread_rng()
            .gen_range(secret_number_min, secret_number_max);
        if guesses.iter().find(|x| x.number == guess).is_none() {
            println!("The CPU guesses . . . {}", guess);
            return guess;
        }
    }
}

fn cpu_clever_guess(guess: &[GuessRatings], memory: &mut NumberMemory) {
    // If the CPU knows any bad numbers, don't use them for future guesses.

    // If the CPU knows any good numbers, use them in future guesses.

    // If the CPU knows any perfect numbers, leave them where they are.
}

fn split_number(number: i64) -> Vec<i8> {
    number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i8)
        .collect()
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
        number: guess,
        perfect: perfect_count,
        good: good_count,
        bad: bad_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_guess_three_perfect() {
        let ratings = rate_guess(123, 123);
        assert_eq!(ratings.perfect, 3);
        assert_eq!(ratings.good, 0);
        assert_eq!(ratings.bad, 0);
    }

    #[test]
    fn test_rate_guess_one_perfect_two_good() {
        let ratings = rate_guess(102, 120);
        assert_eq!(ratings.perfect, 1);
        assert_eq!(ratings.good, 2);
        assert_eq!(ratings.bad, 0);
    }

    #[test]
    fn test_rate_guess_three_good() {
        let ratings = rate_guess(132, 321);
        assert_eq!(ratings.perfect, 0);
        assert_eq!(ratings.good, 3);
        assert_eq!(ratings.bad, 0);
    }

    #[test]
    fn test_rate_guess_three_bad() {
        let ratings = rate_guess(132, 999);
        assert_eq!(ratings.perfect, 0);
        assert_eq!(ratings.good, 0);
        assert_eq!(ratings.bad, 3);
    }

    #[test]
    fn test_rate_guess_one_perfect_two_bad() {
        let ratings = rate_guess(132, 199);
        assert_eq!(ratings.perfect, 1);
        assert_eq!(ratings.good, 0);
        assert_eq!(ratings.bad, 2);
    }

    #[test]
    fn test_cpu_analyze_one_perfect_two_bad() {
        let mut number_memory = NumberMemory {
            ..Default::default()
        };
        cpu_analyze_score(rate_guess(132, 999), &mut number_memory);
        dbg!(number_memory);
    }
}