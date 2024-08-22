use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, lang_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        input: PathBuf,
        message: String,
        output: PathBuf,
    },
    Decode {
        input: PathBuf,
    },
    Remove {
        input: PathBuf,
        ouput: PathBuf,
    },
    List {
        input: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            input,
            message,
            output,
        } => match encode_message(&input, &message, &output) {
            Ok(_) => println!("Message encoded succesfully"),
            Err(e) => eprintln!("Error encoding message: {}", e),
        },
        Commands::Decode { input } => {
            // Decode e=message from PNG file
        }
        Commands::Remove { input, ouput } => {
            // Remove the message from the PNG file
        }
        Commands::List { input } => {
            // List the chunks in the PNG file
        }
    }
}

use png::{BitDepth, ColorType, Encoder, HasParameters};

fn encode_message(
    input_path: &PathBuf,
    message: &str,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut decoder = png::Decoder::new(File::open(input_path)?);
}
