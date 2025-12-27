mod cli;

use cli::{Cli, Commands};
use clap::{Parser};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};
use directories_next::ProjectDirs;
use std::fs::{File,create_dir_all, read_to_string};
use serde_json;

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        return Ok(())
    };

    let cmds_path = check_and_create_file()?;

    let mut json = read_cmd_file(&cmds_path)?;

    println!("{:#?}", cmds_path);
        
    let mut is_dirty = false;

    match command {
        Commands::Save { name, cmd } => {
            let cmd = cmd.join(" ");
            save_command(&mut json, &name, &cmd);
            is_dirty = true;
            println!("Saved command {name}\n updated json: {:#?}", json);
        },
        Commands::Run { name } => {
            println!("run command called: {name}");
        },
        Commands::List => {
            list_commands(&json);
        },
        Commands::Show { name } => {
            match show_command(&json, &name) {
                Some(cmd_value) => println!("{name}: {cmd_value}"),
                None => eprintln!("Unable to find command {name}"),
            }
        }, 
        Commands::Remove { name } => {
            match remove_command(&mut json, &name) {
                Some(removed_cmd) => { 
                    is_dirty = true;
                    println!("Removed command: \n {name}: {removed_cmd}\n updated json: {:#?}", json) 
                },
                None => eprintln!("Cannot find command: {name}"),
            }
        }
    }

    if is_dirty {
        update_file(&cmds_path, &json);
    }

    Ok(())
}

fn update_file(path: &PathBuf, json: &FileJson) {

}

fn remove_command(json: &mut FileJson, cmd_name: &str) -> Option<String> {
    json.commands.remove(cmd_name)
}

fn save_command(json: &mut FileJson, cmd_name: &str, cmd: &str) {
    json.commands.insert(cmd_name.to_owned(), cmd.to_owned());
}

fn show_command<'a>(json: &'a FileJson, cmd_name: &str) -> Option<&'a str> {
    json.commands.get(cmd_name).map(|v| v.as_str())
}

fn list_commands(json: &FileJson) {
    let mut sorted_names: Vec<&String> = json.commands.keys().collect();

    sorted_names.sort();

    println!("Available commands:");
    for name in sorted_names {
        println!(" {name}");
    }
}

fn read_cmd_file(path: &PathBuf) -> Result<FileJson, String> {
    println!("{:#?}", path);
    let contents = read_to_string(path)
        .map_err(|e| format!("Failed to read commands file at {}: {}", path.display(), e))?;

    let json: FileJson = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse commands file at {}: {}", path.display(), e))?;

    Ok(json)
}

#[derive(Debug, Deserialize)]
struct FileJson {
    #[allow(dead_code)] // not read but used in serialization
    version: u8,
    commands: HashMap<String, String>,
}

fn check_and_create_file() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "QuickCmd", "qc") {
        let app_data_dir = proj_dirs.data_dir();

        let cmds_file_path = app_data_dir.join("cmds.json");

        if let Err(_) = create_dir_all(app_data_dir) {
            return Err(String::from("Unable to create commands file"));
        }

        if !cmds_file_path.exists() {
            if let Err(_) = File::create(&cmds_file_path) {
                return Err(String::from("Unable to create commands file"));
            }
        }
        return Ok(cmds_file_path);

    } else {
        Err(String::from("Unable to find path"))
    }
}