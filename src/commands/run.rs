use anyhow::Result;
use crate::project::{Project};

pub fn run_command() -> Result<()> {
    let mut project = Project::find_project()?;

    project.build_main()?;
    println!("{:#?}", project);

    Ok(())
}