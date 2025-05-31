use std::error::Error;
use clap::Parser;
use console::style;
mod cli;
mod rules;
mod scanner;
mod utils;
use cli::CliArgs;
use rules::get_clean_rules;
use scanner::scan_path_for_matching_projects;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    //dbg!(&args);

    let is_dry_run = !args.apply;
    let rules = get_clean_rules();

    if is_dry_run {
        println!("\n{}", style("=== 預覽模式 ===").bold().red());
    } else {
        println!("\n{}", style("=== 清理模式 ===").bold().red());
        if !args.yes {
            println!("警告：這將實際刪除檔案！");
        }
    }

    for path_arg in &args.paths {

        if !path_arg.exists() { // 先檢查路徑是否存在
            eprintln!("警告: 路徑 {} 不存在。", style(path_arg.display()).red());
            continue;
        }
        
        if path_arg.is_dir() {
            scan_path_for_matching_projects(path_arg, &rules, is_dry_run, args.yes)?;
        } else {
             println!("提供的路徑 {} 不是一個目錄，將跳過。", style(path_arg.display()).yellow());
        }
    }
    Ok(())
}


