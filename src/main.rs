mod cli;
mod commands;
mod configs;
mod utils;

use clap::Parser;
use cli::Cli;
use cli::Command;

use commands::create_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match &cli.command {
    Command::Create(app) => create_app(app.clone()).await?,
  };

  Ok(())
}
