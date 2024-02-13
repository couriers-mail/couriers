mod command;

use anyhow::Result;
use clap::Parser;

use self::command::serve::ServeOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Serve(opt) => opt.exec().await?,
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(
    name = "couriers",
    about = "📨 SMTP Server with Web UI for Email solutions development",
    author = "Esteban Borai",
    max_term_width = 80,
    next_line_help = true
)]
pub enum Command {
    /// Runs SMTP server with Web UI
    Serve(ServeOpt),
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
