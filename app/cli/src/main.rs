use clap::{Parser, Subcommand};
mod menu;
// mod ts_export;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 菜单导出
    MenuExport,
    /// 菜单导入
    MenuImport,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::MenuExport) => {
            menu::export().await.unwrap();
        }
        Some(Commands::MenuImport) => {
            menu::import().await.unwrap();
        }
        None => {}
    };
}
