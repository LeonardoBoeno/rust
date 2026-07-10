use crate::cli::Args;
use crate::password::generate_password;
use clap::Parser;

pub fn run_with_args(args: &Args) -> Result<String, String> {
    let mut charset = Vec::new();

    if args.lowercase {
        charset.extend(b"abcdefghijklmnopqrstuvwxyz");
    }
    if args.uppercase {
        charset.extend(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if args.numbers {
        charset.extend(b"0123456789");
    }
    if args.symbols {
        charset.extend(b"!@#$%^&*()_-+=<>?");
    }

    if charset.is_empty() {
        return Err("No character type was set.".into());
    }

    Ok(generate_password(args.length, &charset))
}

pub fn run() {
    let args = Args::parse();

    match run_with_args(&args) {
        Ok(password) => println!("Senha gerada: {}", password),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}