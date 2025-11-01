use clap::Parser;
use std::io::prelude::*;
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Prompt string to ask the user for input
    prompt: String,
    /// The default response, if no input is received from the user.
    #[arg(long, short, value_parser = ["yes", "y", "no", "n"])]
    default: Option<String>,
}
const ERROR_CODE: u8 = 0xff;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let input = std::io::stdin();
    let mut output = std::io::stdout();

    print!("{} ", cli.prompt);
    if let Err(err) = output.flush() {
        eprintln!("{err}");
        return ExitCode::from(ERROR_CODE);
    }

    loop {
        let mut buf = String::new();
        if let Err(err) = input.read_line(&mut buf) {
            eprintln!("{err}");
            return ExitCode::from(ERROR_CODE);
        }

        let mut line = buf.trim().to_lowercase();
        if let Some(default) = cli.default.as_ref()
            && line.is_empty()
        {
            line = default.clone();
        }

        if line == "y" || line == "yes" {
            break ExitCode::SUCCESS;
        } else if line == "n" || line == "no" {
            break ExitCode::FAILURE;
        } else {
            print!("Unrecognized response! {} ", cli.prompt);
            output.flush().unwrap();
        }
    }
}
