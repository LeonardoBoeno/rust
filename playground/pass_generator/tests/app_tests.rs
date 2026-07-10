use pass_generator::{app::run_with_args, cli::Args};
use clap::Parser;

#[test]
fn test_app_generates_password() {
    let args = Args::parse_from([
        "prog",
        "--length", "10",
        "--lowercase",
    ]);

    let result = run_with_args(&args).unwrap();
    assert_eq!(result.len(), 10);
}

#[test]
fn test_app_errors_when_charset_empty() {
    let args = Args::parse_from([
        "prog",
        "--length", "10",
    ]);

    let result = run_with_args(&args);
    assert!(result.is_err());
}
