use std::fs::create_dir_all;
use std::fs::File;
use crate::constance::*;
use crate::utils::*;

/// Function for all startup events.
pub fn startup() -> std::io::Result<String> {
    let path_to_app_dir = build_full_app_path()?;
    create_dir_all(&path_to_app_dir)?;
    create_new_app_config(&path_to_app_dir)?;
    Ok("All configuration files created successfully.".to_string())
}

/// Creates new config file with structure.
fn create_new_app_config(path: &str) -> std::io::Result<()> {
    let config_file_path = format!("{}/{}", path, APP_CONFIG);

    if std::path::Path::new(&config_file_path).exists() {
        return Ok(())
    }

    File::create(&config_file_path)?;
    let default_config_structure =
r#"
{
    "jira": {
        "companies": []
    }
}
"#;
    std::fs::write(config_file_path, default_config_structure)?;
    Ok(())
}