mod templates;

use clap::{arg, Command};
use templates::{TEMPLATE_CONFIG_CONTENT, TEMPLATE_MAIN_CONTENT};
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

pub struct RucoCli;

impl RucoCli {
    pub fn cli() -> Command<'static> {
        Command::new("ruco")
            .about("Ruco utility for command line")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("new")
                    .about("Create new project")
                    .arg(arg!(<NAME> "Name of the project"))
                    .arg_required_else_help(true),
            )
            .subcommand(Command::new("repl").about("Start the REPL"))
    }

    pub fn start_cli() {
        let matches = RucoCli::cli().get_matches();

        match matches.subcommand() {
            Some(("new", sub_matches)) => {
                RucoCli::handle_new_command(sub_matches);
            }
            Some(("repl", _)) => {
                RucoCli::handle_repl_command();
            }
            _ => {
                println!("No subcommand was used");
            }
        }
    }

    fn handle_new_command(matches: &clap::ArgMatches) {
        let name = matches.value_of("NAME").unwrap_or("ruco-project");
        println!("Creating new project {}", name);

        // create a folder with the name
        std::fs::create_dir(name).unwrap();

        let main_path = format!("{}/main.ruco", name);
        let main_content = TEMPLATE_MAIN_CONTENT.replace("{{NAME}}", name);
        std::fs::write(&main_path, main_content).unwrap();
        println!("Created {}", main_path);

        let config_path = format!("{}/Ruco.toml", name);
        let config_content = TEMPLATE_CONFIG_CONTENT.replace("{{NAME}}", name);
        std::fs::write(&config_path, config_content).unwrap();
        println!("Created {}", config_path);
    }

    fn handle_repl_command() {
        let pb = ProgressBar::new(100);
        println!("Starting repl...");
        for _ in 0..100 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(5));
        }
        pb.finish_with_message("done");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
