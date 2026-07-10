use pass_generator::cli::Args;
use clap::Parser;

#[test]
fn test_default_values() {
    let args = Args::parse_from(["prog"]);
    assert_eq!(args.length, 16);
    assert!(!args.symbols);
    assert!(!args.numbers);
    assert!(!args.uppercase);
    assert!(!args.lowercase);
}

#[test]
fn test_custom_length() {
    let args = Args::parse_from(["prog", "--length", "40"]);
    assert_eq!(args.length, 40);
}

#[test]
fn test_enable_symbols() {
    let args = Args::parse_from(["prog", "--symbols"]);
    assert!(args.symbols);
}
