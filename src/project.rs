use crate::{
    ast::parser::Parser,
    error::PulseError::{
        InvalidProjectStructure, MultipleEntryPoints, ProjectNotFound, SemanticError,
    },
    fs::find_nearest_file,
    lexer::{token::Token, Lexer},
    resolver::Resolver,
};
use anyhow::Result;
use log::debug;
use std::{cmp::PartialEq, env, fmt::format, fs, path::PathBuf};

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
    pub root: PathBuf,
    pub tokens: Vec<Token>,
    pub content: String,
}

impl Project {
    pub fn from_path(root: PathBuf, project_type: ProjectType) -> Project {
        Project {
            project_type,
            root,
            tokens: vec![],
            content: String::new(),
        }
    }
}

impl Project {
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
}

impl Project {
    pub fn main_file(&self) -> PathBuf {
        self.root.join("src").join(self.project_type.file_name())
    }

    pub fn build_main(&mut self) -> Result<()> {
        let main_file = self.main_file();
        let main_content = fs::read_to_string(&main_file)?;
        self.content = main_content.clone();

        log::debug!("Building main file: {:?}", main_file);

        let mut lexer = Lexer::from_source(main_content);
        let tokens = lexer.lex()?;

        self.tokens = tokens;

        let mut parser = Parser::new(self.tokens.clone());
        let ast = parser.parse()?;

        let mut resolver = Resolver::new();

        resolver.resolve_ast(&ast)?;

        Ok(())
    }
}
