use clap::{Parser, Subcommand};
mod menu;
mod ts_export;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    MenuInit,
    TsExport,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::MenuInit) => {
            menu::init().await.unwrap();
        }
        Some(Commands::TsExport) => {
            ts_export::init();
        }
        None => {}
    };
}
