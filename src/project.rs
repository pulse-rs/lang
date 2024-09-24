use std::env;

use anyhow::Result;

use crate::fs::find_nearest_file;

#[derive(Debug)]
pub struct Project {

}

pub fn find_project() -> Result<Project> {
    let cwd = env::current_dir()?;
    let path = find_nearest_file(cwd, file_name)

    Ok(Project {})
}