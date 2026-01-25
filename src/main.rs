use std::env;
use clap::{Parser, ArgGroup};
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use gnt_tools::core_char;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// On définit un groupe "input" qui est obligatoire (required = true)
// et qui interdit d'utiliser les deux arguments en même temps (conflicts_with est géré par le groupe)
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["text", "file"])
))]
struct Args {
    /// Raw text directly passed to the program.
    #[arg(group = "input")]
    text: Option<String>,

    /// Path to a file containing a text to process.
    #[arg(short = 'f', long, group = "input")]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Variable qui contiendra le résultat final (texte ou contenu du fichier)
    let text: String;

    if let Some(text_input) = args.text {
        text = text_input;
    }
    else if let Some(file_path) = args.file {
        text = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read the file {:?}", file_path))?;
    } else {
        // Grâce au ArgGroup "required", ce cas est impossible à atteindre,
        // clap affichera l'aide avant d'arriver ici.
        unreachable!();
    }

    println!("{}",
        
        text.chars()
            .filter_map(|c| core_char(&c))
            .collect::<String>()
    );

    Ok(())
}

