use std::process::ExitCode;
use std::{fs, io};

use velo::models::{Cosmos, Rune, Vessel};
use velo::sail::{Config, Termination, sail};

use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    file_path: String,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    trace: bool,

    #[arg(long, requires = "trace")]
    ignore_void: bool,
}

fn main() -> ExitCode {
    let cli = Args::parse();

    let config = Config::new(cli.debug || cli.trace, cli.trace, cli.ignore_void);

    match load_velo_code(&cli.file_path) {
        Err(msg) => {
            eprintln!("Failed to load velo file. {:}", msg);
            ExitCode::FAILURE
        }
        Ok(code) => {
            // let code_lines = harmonize_runes(raw_code);

            let code_lines: Vec<String> = code.lines().map(|line| line.to_string()).collect();

            let cosmos = materialize_runes(code_lines);

            let start_rune = cosmos.get(0, 0);

            let vessel = Vessel::new(0, 0, start_rune);

            match sail(cosmos, vessel, config) {
                Termination::Stopped => ExitCode::SUCCESS,
                Termination::NoSignal(last_signal_x, last_signal_y) => {
                    eprintln!(
                        "The vessel traveled out of the cosmos. Last signal coordinate: {{ x: {:}, y: {:} }}",
                        last_signal_x, last_signal_y
                    );
                    ExitCode::FAILURE
                }
                Termination::NoInitialVelocityOrDirection => {
                    eprintln!("Here was no Thrust rune at the top left corner of the cosmos.");
                    ExitCode::FAILURE
                }
            }
        }
    }
}

fn load_velo_code(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

fn materialize_runes(lines: Vec<String>) -> Cosmos {
    let runes: Vec<Vec<Rune>> = lines
        .iter()
        .map(|line| {
            if let Some((before_hash, _)) = line.split_once('#') {
                before_hash.to_string()
            } else {
                line.clone()
            }
            .chars()
            .map(|c| char_to_rune(c))
            .collect()
        })
        .collect();

    let height = runes.len();
    let width = runes.iter().map(|line| line.len()).max().unwrap_or(0);

    Cosmos::new(runes, width, height)
}

fn char_to_rune(c: char) -> Rune {
    match c {
        '^' => Rune::ThrustUp,
        'v' => Rune::ThrustDown,
        '<' => Rune::ThrustLeft,
        '>' => Rune::ThrustRight,
        'P' => Rune::Parking,
        '+' => Rune::EntropyIncrease,
        '-' => Rune::EntropyDecrease,
        '[' => Rune::SteerLeft,
        ']' => Rune::SteerRight,
        ',' => Rune::Input,
        '.' => Rune::Output,
        'D' => Rune::Debug,
        _ => Rune::Void,
    }
}
