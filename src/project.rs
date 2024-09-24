use std::cmp::PartialEq;
use std::env;
use anyhow::Result;
use crate::error::PulseError::{ProjectNotFound, InvalidProjectStructure, MultipleEntryPoints};
use crate::fs::find_nearest_file;

#[derive(Debug, PartialEq)]
pub enum ProjectType {
    Binary,
    Library,
}

#[derive(Debug)]
pub struct Project {
    pub project_type: ProjectType,
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

            return Ok(Project { project_type });
        }
    }

    log::debug!("No project found");
    Err(ProjectNotFound.into())
}
