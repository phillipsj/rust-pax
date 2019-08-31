use std::fs;
use std::fs::File;
use std::path::PathBuf;

enum Folder {
    File(String),
    Folder(String, Vec<Folder>),
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
    let mut files = convert_strings_to_files(&["main.tf", "main.tf", "output.tf"]);
    files.push(envs_folder);
    Folder::Folder(name.to_string(), files)
}

fn generate_infrastructure_folder(app_name: &str) -> Folder {
    let service_folder = generate_service_folder(app_name);
    Folder::Folder("infrastructure".to_string(), vec![service_folder])
}

fn generate_paths(filesystem: Folder) -> Vec<PathBuf> {
    let file_paths = match filesystem {
        Folder::File(x) => vec![PathBuf::from(x)],
        // After the first map I have a list of paths that I now need to append the folder to the front
        Folder::Folder(folder, folders) => folders
            .into_iter()
            .map(|name| generate_paths(name))
            .into_iter()
            .map(|paths| {
                paths
                    .into_iter()
                    .map(|path| PathBuf::from(&folder).join(path))
                    .collect()
            })
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
    let filesystem = generate_infrastructure_folder("myapp");
    let paths = generate_paths(filesystem);
    for path in paths {
        if let Some(stuff) = path.to_str() {
            println!("{}",stuff);
        }

        create_path(path)?;
    }

    Ok(())
}
