use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

enum Folder {
    File(String),
    Folder(String, Vec<Folder>),
}

fn set_tf_logging(log_level: &str, log_path: &str) {
    env::set_var("TF_LOG", log_level);
    env::set_var("TF_LOG_PATH", log_path);
}

fn remove_tf_logging() {
    env::remove_var("TF_LOG");
    env::remove_var("TF_LOG_PATH");
}

fn set_pk_logging(log_level: &str, log_path: &str) {
    env::set_var("PACKER_LOG", log_level);
    env::set_var("PACKER_LOG_PATH", log_path);
}

fn remove_pk_logging() {
    env::remove_var("PACKER_LOG");
    env::remove_var("PACKER_LOG_PATH");
}

fn convert_strings_to_files(file_names: &[&str]) -> Vec<Folder> {
    file_names
        .into_iter()
        .map(|name| Folder::File(name.to_string()))
        .collect()
}

fn generate_envs_folder() -> Folder {
    let env_files = convert_strings_to_files(&["dev.tfvars", "qa.tfvars", "prod.tfvars"]);
    Folder::Folder(String::from("envs"), env_files)
}

fn generate_service_folder(name: &str) -> Folder {
    let envs_folder = generate_envs_folder();
    let mut files = convert_strings_to_files(&["main.tf", "variables.tf", "output.tf"]);
    files.push(envs_folder);
    Folder::Folder(name.to_string(), files)
}

fn generate_infrastructure_folder(app_name: &str) -> Folder {
    let service_folder = generate_service_folder(app_name);
    Folder::Folder("infrastructure".to_string(), vec![service_folder])
}

fn generate_module_folder(name: &str) -> Folder {
    let files = convert_strings_to_files(&["main.tf", "variables.tf", "output.tf"]);
    Folder::Folder(name.to_string(), files)
}

fn generate_packer_project(name: &str) -> Folder {
    let assets = Folder::Folder(
        "assets".to_string(),
        convert_strings_to_files(&[".gitkeep"]),
    );
    let files = Folder::Folder("files".to_string(), convert_strings_to_files(&[".gitkeep"]));
    let scripts = Folder::Folder(
        "scripts".to_string(),
        convert_strings_to_files(&[".gitkeep"]),
    );
    let user_data = Folder::Folder(
        "user_data".to_string(),
        convert_strings_to_files(&[".gitkeep"]),
    );
    let packer_json = Folder::File(format!("{}.json", name));

    Folder::Folder(
        name.to_string(),
        vec![assets, files, scripts, user_data, packer_json],
    )
}

fn generate_paths(filesystem: Folder) -> Vec<PathBuf> {
    let file_paths = match filesystem {
        Folder::File(x) => vec![PathBuf::from(x)],
        Folder::Folder(folder, folders) => folders
            .into_iter()
            .flat_map(|path| generate_paths(path))
            .collect::<Vec<PathBuf>>()
            .into_iter()
            .map(|path| PathBuf::from(&folder).join(path))
            .collect(),
    };
    file_paths
}

fn create_path(path: PathBuf) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?
    }
    File::create(path)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let matches = App::new("pax")
        .about("A tool that aids working with DevOps tooling, specifically Terraform and Packer.")
        .version("1.0")
        .author("BlueGhost Labs - Jamie Phillips")
        .subcommand(SubCommand::with_name("tf")
            .about("Terraform related commands")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("new") 
                .about("Creates new directories for Terraform.")
                .subcommand(SubCommand::with_name("infra")
                .about("Creates the entire infrastructure directory.")
                .arg(Arg::with_name("name")
                    .required(false)
                    .help("The name for the application. If not provided defaults to app.")))
                .subcommand(SubCommand::with_name("app")
                .about("Creates a new application in the current directory.")
                .arg(Arg::with_name("name")
                    .required(true)
                    .help("The name for the application.")))
                .subcommand(SubCommand::with_name("module")
                .about("Creates a new module in the current directory.")
                .arg(Arg::with_name("name")
                    .required(true)
                    .help("The name of the module."))))
            .subcommand(SubCommand::with_name("logging")
                .about("Enables logging for Terraform.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("enable")
                    .about("Enables logging.")                
                    .arg(Arg::with_name("file")
                        .required(false)
                        .default_value("terraform_log.txt")
                        .help("Defines a custom log file name. Default will be terraform_log.txt")))
                .subcommand(SubCommand::with_name("disable")
                    .about("Disables logging."))))
        .subcommand(SubCommand::with_name("pk")
            .about("Packer related commands")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("new") 
                .about("Creates new project for Packer.")
                .arg(Arg::with_name("name")
                    .required(true)
                    .help("The name for the Packer project.")))
            .subcommand(SubCommand::with_name("logging")
                .about("Control logging for Packer.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("enable")
                    .about("Enables logging.")                
                    .arg(Arg::with_name("file")
                        .required(false)
                        .default_value("packer_log.txt")
                        .help("Defines a custom log file name. Default will be packer_log.txt")))
                .subcommand(SubCommand::with_name("disable")
                    .about("Disables logging."))))
        .get_matches();

    match matches.subcommand() {
        ("tf", Some(tf_matches)) => match tf_matches.subcommand() {
            ("new", Some(new_matches)) => match new_matches.subcommand() {
                ("infra", Some(infra_matches)) => {
                    let filesystem =
                        generate_infrastructure_folder(infra_matches.value_of("name").unwrap());
                    let paths = generate_paths(filesystem);
                    for path in paths {
                        create_path(path)?;
                    }
                }
                ("app", Some(app_matches)) => {
                    let filesystem = generate_service_folder(app_matches.value_of("name").unwrap());
                    let paths = generate_paths(filesystem);
                    for path in paths {
                        create_path(path)?;
                    }
                }
                ("module", Some(module_matches)) => {
                    let filesystem =
                        generate_module_folder(module_matches.value_of("name").unwrap());
                    let paths = generate_paths(filesystem);
                    for path in paths {
                        create_path(path)?;
                    }
                }
                _ => unreachable!(),
            },
            ("logging", Some(logging_matches)) => match logging_matches.subcommand() {
                ("enable", Some(enable_matches)) => {
                    set_tf_logging("DEBUG", enable_matches.value_of("file").unwrap());
                }
                ("disable", Some(_)) => {
                    remove_tf_logging();
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        ("pk", Some(pk_matches)) => match pk_matches.subcommand() {
            ("new", Some(new_matches)) => {
                let filesystem = generate_packer_project(new_matches.value_of("name").unwrap());
                let paths = generate_paths(filesystem);
                for path in paths {
                    create_path(path)?;
                }
            }
            ("logging", Some(logging_matches)) => match logging_matches.subcommand() {
                ("enable", Some(enable_matches)) => {
                    set_pk_logging("1", enable_matches.value_of("file").unwrap());
                }
                ("disable", Some(_)) => {
                    remove_pk_logging();
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    Ok(())
}
