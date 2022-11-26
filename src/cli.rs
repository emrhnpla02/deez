use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[clap(flatten)]
  verbose: clap_verbosity_flag::Verbosity,

  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  Create(App),
}

#[derive(Args, Debug, Clone)]
pub struct App {
  pub app_type: AppType,
  pub app_name: String,
  #[arg(short = 'c', long)]
  pub config: Option<std::path::PathBuf>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum AppType {
  React,
  Next,
  Astro,
}
