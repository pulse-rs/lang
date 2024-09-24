use std::cmp::PartialEq;
use std::{env, fs};
use std::fmt::format;
use std::path::PathBuf;
use anyhow::Result;
use crate::error::PulseError::{ProjectNotFound, InvalidProjectStructure, MultipleEntryPoints};
use crate::fs::find_nearest_file;

#[derive(Debug, PartialEq)]
pub enum ProjectType {
    Binary,
    Library,
}

impl ProjectType {
    pub fn file_name(&self) -> &str {
        match self {
            ProjectType::Binary => "main.pulse",
            ProjectType::Library => "lib.pulse",
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub project_type: ProjectType,
    pub content: String,
    pub root: PathBuf,
}

impl Project {
    pub fn from_path(root: PathBuf, project_type: ProjectType) -> Project {
        let content = fs::read_to_string(root.join(format!("src/{}", project_type.file_name()))).unwrap_or_default();

        Project {
            project_type,
            content,
            root,
        }
    }
}

pub fn find_project() -> Result<Project> {
    let cwd = env::current_dir()?;
    let path = find_nearest_file(cwd, "pulse.toml");

    if let Some(path) = path {
        log::debug!("Project found at {:?}", path);

        if let Some(root) = path.parent() {
            let src_folder = root.join("src");
            let main_exists = src_folder.join("main.pulse").exists();
            let lib_exists = src_folder.join("lib.pulse").exists();

            let project_type = match (main_exists, lib_exists) {
                (true, false) => ProjectType::Binary,
                (false, true) => ProjectType::Library,
                (true, true) => return Err(MultipleEntryPoints.into()),
                _ => return Err(InvalidProjectStructure.into()),
            };

            return Ok(Project::from_path(root.to_path_buf(), project_type));
        }
    }

    log::debug!("No project found");
    Err(ProjectNotFound.into())
}
