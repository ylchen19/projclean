use std::path::{Path, PathBuf};
use std::error::Error;
use walkdir::WalkDir;
use dialoguer::{Confirm, theme::ColorfulTheme};
use console::style;
use crate::rules::ProjectCleanRule;
use crate::utils::{calculate_dir_size, format_size};

pub fn handle_identified_project(
    project_root: &Path,
    identifier_file_name: &str,
    rule: &ProjectCleanRule,
    is_dry_run: bool,
    skip_confirmation: bool
) -> Result<bool, Box<dyn Error>> {
    println!(
        "  在 {} 發現 {} 專案 (識別檔案: {})",
        style(project_root.display()).blue(),
        style(&rule.project_type).green(),
        style(identifier_file_name).italic()
    );

    let mut found_target_for_this_project_in_rule = false;

    for dir_to_clean_name in &rule.directories_to_clean {
        let target_dir_to_check = project_root.join(dir_to_clean_name);
        if target_dir_to_check.is_dir() {
            found_target_for_this_project_in_rule = true;
            process_target_directory(
                &target_dir_to_check,
                &rule.project_type,
                dir_to_clean_name,
                is_dry_run,
                skip_confirmation,
            )?;
        }
    }

    if !found_target_for_this_project_in_rule {
        println!(
            "    -> 在 {} 專案 {} 中未找到可清理的目標目錄 ({:?})。",
            style(&rule.project_type).green(),
            style(project_root.display()).blue(),
            rule.directories_to_clean
        );
    }
    Ok(found_target_for_this_project_in_rule)

}

pub fn scan_path_for_matching_projects(
    scan_path: &PathBuf,
    rules: &[ProjectCleanRule],
    is_dry_run: bool,
    skip_confirmation: bool,
) -> Result<(), Box<dyn Error>>{
    println!("\n掃描路徑 {} 以尋找可清理的專案目標...", style(scan_path.display()).cyan());
    let mut overall_found_any_target = false;

    for entry in WalkDir::new(scan_path)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok()) {
            let current_path = entry.path();

            if !current_path.is_file() {
                continue;
            }

            let file_name_str = match current_path.file_name().and_then(|name| name.to_str()) {
                Some(name_str) => name_str,
                None => continue, // 如果沒有檔名或檔名不是有效的 UTF-8，則跳過
            };

            let project_root = match current_path.parent() {
                Some(root) => root,
                None => continue, // 如果沒有父目錄 (不太可能發生於檔案路徑)，則跳過
            };

            for rule in rules {
                if file_name_str == rule.identifier_file {
                    // 呼叫輔助函式來處理已識別的專案
                    if handle_identified_project(
                        project_root,
                        file_name_str,
                        rule,
                        is_dry_run,
                        skip_confirmation,
                    )? {
                        overall_found_any_target = true;
                    }
                    break; // 找到匹配此檔案的規則後，無需再用其他規則檢查同一個檔案
                }
            }
        }
        if !overall_found_any_target {
            println!("在路徑 {} 下未找到任何符合清理規則的專案目標。", style(scan_path.display()).cyan());
        }
        Ok(())
}



pub fn process_target_directory(
    target_dir_path: &PathBuf,
    project_type: &str,
    target_artifact_name: &str,
    is_dry_run: bool,
    skip_confirmation: bool,
) -> Result<(), Box<dyn Error>> {
    let dir_size = calculate_dir_size(target_dir_path).unwrap_or(0); // 簡單計算大小
    let size_str = format_size(dir_size);

    let target_display_path = style(target_dir_path.display()).cyan();
    let friendly_message_prefix = format!("{} 的 {} 目錄", style(project_type).green(), style(target_artifact_name).yellow());

    if is_dry_run {
        println!(
            "  {} 發現 {}: {} (估計大小: {})",
            style("[預覽]").yellow().bold(),
            friendly_message_prefix,
            target_display_path,
            style(&size_str).blue()
        );
    } else {
        println!(
            "  發現 {}: {} (估計大小: {})",
            friendly_message_prefix,
            target_display_path,
            style(&size_str).blue()
        );

        let confirmed = if skip_confirmation {
            true
        } else {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("    ❓ 是否確定刪除 {} ({}) ?", friendly_message_prefix, target_display_path))
                .default(false)
                .show_default(true)
                .wait_for_newline(true)
                .interact_opt()?
                .unwrap_or(false)
        };

        if confirmed {
            println!(
                "    {} 正在刪除 {} ({}) ...",
                style("[清理]").red().bold(),
                friendly_message_prefix,
                target_display_path
            );
            std::fs::remove_dir_all(target_dir_path)?;
            println!(
                "    {} {} ({}) (模擬)已成功刪除。", // 先用模擬訊息
                style("[清理]").red().bold(),
                friendly_message_prefix,
                target_display_path
            );
        } else {
            println!(
                "    {} 跳過刪除 {} ({})",
                style("[操作取消]").yellow(),
                friendly_message_prefix,
                target_display_path
            );
        }
    }
    Ok(())
}
