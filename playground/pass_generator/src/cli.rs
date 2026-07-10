use clap::Parser;

#[derive(Parser)]
#[command(author = "Leonardo Boeno", version = "1.0", about = "Gerador de senhas em Rust")]
pub struct Args {
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,
    #[arg(long, default_value_t = false)]
    pub symbols: bool,
    #[arg(long, default_value_t = false)]
    pub numbers: bool,
    #[arg(long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(long, default_value_t = false)]
    pub lowercase: bool,
}