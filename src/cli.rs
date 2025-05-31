use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "一個清理專案暫存檔的CLI工具")]
pub struct CliArgs {
    #[arg(short = 'y', long, default_value_t = false)]
    pub yes: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub apply: bool,

    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,
}