use std::{io, path::PathBuf};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, Password};

fn main() -> io::Result<()> {
    let logged_in = false;

    let args = Args::parse();
    let interactive =
        args.name.is_none() || args.framework().is_none() || (args.new && args.api_key.is_none());

    let theme = ColorfulTheme::default();

    // 1. Log in (if not logged in yet)
    if interactive && !logged_in {
        println!("First, let's log in to your Shuttle account.");
        let api_key = Password::with_theme(&theme)
            .with_prompt("API key")
            .interact()?;
        login(api_key)?;
        println!();
    }

    // 2. Ask for project name
    let project_name = match &args.name {
        Some(name) => name.clone(),
        None => {
            println!("How do you want to name your project? It will be hosted at ${{project_name}}.shuttleapp.rs.");
            let name = loop {
                let name: String = Input::with_theme(&theme)
                    .with_prompt("Project name")
                    .interact()?;
                if is_available(&name) {
                    break name;
                }
                println!("Unfortunately, that name is already taken. Please try a different name.");
            };
            println!();
            name
        }
    };

    // 3. Confirm directory with same name
    let directory = if interactive {
        println!("Where should we create this project?");
        let directory: String = Input::with_theme(&theme)
            .with_prompt("Directory")
            .with_initial_text(&project_name)
            .interact()?;
        println!();
        directory
    } else {
        project_name.clone()
    };

    // 3. Ask for the framework
    let framework = match args.framework() {
        Some(framework) => framework,
        None => {
            println!("Shuttle works with a range of web frameworks. Which one do you want to use?");
            let frameworks = ["axum", "rocket", "tide"];
            let index = FuzzySelect::with_theme(&theme)
                .items(&frameworks)
                .default(0)
                .interact()?;
            println!();
            frameworks[index]
        }
    };

    // 4. Initialize locally
    init(&project_name, &directory, framework)?;

    // 5. Confirm that the user wants to create the project environment on Shuttle
    let should_create_environment = if !interactive {
        args.new
    } else if args.new {
        true
    } else {
        Confirm::with_theme(&theme)
            .with_prompt("Do you want to create the project environment on Shuttle?")
            .default(true)
            .interact()?
    };
    if should_create_environment {
        set_environment(&project_name);
    }

    Ok(())
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    pub name: Option<String>,
    #[clap(long, conflicts_with_all = &["rocket", "tide"])]
    pub axum: bool,
    #[clap(long, conflicts_with_all = &["axum", "tide"])]
    pub rocket: bool,
    #[clap(long, conflicts_with_all = &["axum", "rocket"])]
    pub tide: bool,
    #[clap(long)]
    pub new: bool,
    #[clap(long)]
    pub api_key: Option<String>,
    #[clap(default_value = ".")]
    pub path: PathBuf,
}

impl Args {
    fn framework(&self) -> Option<&str> {
        if self.axum {
            Some("axum")
        } else if self.rocket {
            Some("rocket")
        } else if self.tide {
            Some("tide")
        } else {
            None
        }
    }
}

fn login(api_key: String) -> io::Result<()> {
    eprintln!("--> Logging in with {}", api_key);
    Ok(())
}

fn is_available(project_name: &str) -> bool {
    eprintln!("--> Checking if {} is available", project_name);
    project_name != "p"
}

fn set_environment(project_name: &str) {
    eprintln!("--> Setting environment for project {}", project_name);
}

fn init(project_name: &str, directory: &str, framework: &str) -> io::Result<()> {
    eprintln!(
        "--> Cargo init with project name {} in dir {}",
        project_name, directory
    );
    eprintln!("--> Generating code for {}", framework);
    Ok(())
}
