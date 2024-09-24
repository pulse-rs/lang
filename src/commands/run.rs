use anyhow::Result;
use crate::project::find_project;

pub fn run_command() -> Result<()> {
    let project = find_project()?;

    println!("Running project: {:#?}", project);
    Ok(())
}