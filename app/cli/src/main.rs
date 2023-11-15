use clap::Parser;
mod init;
mod menu;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Data Init
    Init(init::CliInitParams),
    /// Menu Export
    MenuExport,
    /// Menu Import
    MenuImport,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = Some(format!("{}=INFO", env!("CARGO_PKG_NAME")));
    utils::logger::init(env_filter);
    let cli = Cli::parse();
    let result = match &cli {
        Cli::Init(params) => init::exec(params).await,
        Cli::MenuExport => menu::export().await,
        Cli::MenuImport => menu::import().await,
    };
    if let Err(e) = result {
        tracing::error!("{:#?}", e);
    }
    Ok(())
}
