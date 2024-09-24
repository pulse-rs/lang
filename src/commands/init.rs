use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use crate::error::PulseError;

pub fn init_command(name: Option<String>) -> Result<()> {
    let project_name = name.unwrap_or_else(|| "pulse_project".to_string());
    let path = PathBuf::from(&project_name);

    if path.exists() {
        return Err(PulseError::ProjectAlreadyExists.into());
    }

    fs::create_dir_all(path.join("src"))
        .with_context(|| format!("Failed to create project directories for {}", project_name))?;

    let files: Vec<(&str, &str)> = vec![
        (
            "src/main.pulse",
            "fn main() {\n    println(\"Hello, World!\");\n}\n",
        ),
        (
            "pulse.toml",
            "[project]\nname = \"pulse_project\"\nversion = \"0.1.0\"\n",
        ),
        (".gitignore", "build/\n"),
    ];

    for (file, content) in files {
        let file_path = path.join(file);
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write file {}", file))?;
        log::info!("Created file: {:?}", file_path);
    }

    log::info!("Created project at {:?}", path);

    Ok(())
}
