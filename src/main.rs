use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "d", long = "digit")]
    digit: i8
}

fn main() {
    let args = Cli::from_args();
}
