#[derive(Debug, Clone)]
pub struct ProjectCleanRule {
    pub project_type: String,
    pub identifier_file: String,
    pub directories_to_clean: Vec<String>,
    pub description: String,
}

pub fn get_clean_rules() -> Vec<ProjectCleanRule> {
    vec![
        ProjectCleanRule {
            project_type: "Rust".to_string(),
            identifier_file: "Cargo.toml".to_string(),
            directories_to_clean: vec!["target".to_string()],
            description: "Rust 建置產物".to_string(),
        },
        ProjectCleanRule {
            project_type: "Node.js".to_string(),
            identifier_file: "package.json".to_string(),
            directories_to_clean: vec!["node_modules".to_string()],
            description: "Node.js 依賴項目".to_string(),
        },
    ]
}