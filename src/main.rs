use std::fs::File;
use std::path::{Path, PathBuf};

enum Folder {
    File(String),
    Folder(String, Vec<Folder>)
}

fn strings_to_files(file_names: &[&str]) -> Vec<Folder> {
    file_names.into_iter().map(|name| Folder::File(name.to_string())).collect()
}

fn create_envs_folder() -> Folder {
    let env_files = strings_to_files(&["dev.tfvars", "qa.tfvars", "prod.tfvars"]);
    Folder::Folder(String::from("envs"), env_files)
}

fn create_service_folder(name: &str) -> Folder {
    let envs_folder = create_envs_folder();
    let files = strings_to_files(&["main.tf", "main.tf", "output.tf"]);
    Folder::Folder(name.to_string(), files)
}

fn create_infrastructure_folder(app_name: &str) -> Folder {
    let service_folder = create_service_folder(app_name);
    Folder::Folder("infrastructure".to_string(), vec![service_folder])
}

fn create_paths(filesystem: Folder) -> PathBuf {
    let stuff = match filesystem {
        Folder::File(x) => PathBuf::from(x),
        // After the first map I have a list of paths that I now need to append the folder to the front
        Folder::Folder(folder, folders) => folders.into_iter().map(|name| create_paths(name)).map(|path| PathBuf::from(&folder).join(path)).collect()
    };
    stuff
}

fn main() -> std::io::Result<()> {
    let filesystem = create_infrastructure_folder("myapp");
    let paths = create_paths(filesystem);
    let create = File::create("test/filename.txt")?;
    println!("Hello, world!");

    Ok(())
}
