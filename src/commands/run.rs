use crate::{diagnostic::print_diagnostic, project::Project};
use anyhow::Result;

pub fn run_command() -> Result<()> {
    let mut project = Project::find_project()?;

    match project.build_main() {
        Ok(_) => {
            log::debug!("Built main file");
            Ok(())
        }
        Err(err) => {
            print_diagnostic(err, Some(project.content));
            Ok(())
        }
    }
}
