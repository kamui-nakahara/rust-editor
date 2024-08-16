use crate::Data;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run(data: &mut Data) -> bool {
    let commands: Vec<&str> = data.command.split(" ").collect();
    match commands[0] {
        "q" => return true,
        "set" => {
            match commands[1] {
                "number" => {
                    data.settings.number = true;
                }
                "nonumber" => {
                    data.settings.number = false;
                }
                _ => {}
            }
            data.mode = 'n';
            return false;
        }
        "e" => {
            data.load(commands[1].to_string()).unwrap();
            data.mode = 'n';
            return false;
        }
        "w" => {
            let path: String;
            if data.path.is_empty() {
                if commands.len() == 1 {
                    return false;
                }
                path = commands[1].to_string();
            } else {
                path = data.path.clone();
            }
            let mut file = File::create(path.clone()).unwrap();
            let mut content = String::new();
            for i in 0..data.content.len() - 1 {
                let line = data.content[i].clone();
                content += &line;
                content += "\n";
            }
            content += &data.content[data.content.len() - 1].clone();
            file.write(content.as_bytes()).expect("cannot write");
            data.path = path;
            data.mode = 'n';
            return false;
        }
        "wq" => {
            let mut path = String::new();
            if path == String::new() {
                path = data.path.clone();
            } else {
                path = commands[1].to_string();
            }
            let mut file = File::create(path).unwrap();
            let mut content = String::new();
            for i in 0..data.content.len() - 1 {
                let line = data.content[i].clone();
                content += &line;
                content += "\n";
            }
            content += &data.content[data.content.len() - 1].clone();
            file.write(content.as_bytes()).expect("cannot write");
            return true;
        }
        "Ex" => {
            if commands.len() == 1 {
                if data.path.is_empty() {
                    let file = std::env::current_dir().unwrap();
                    let path = file.display().to_string();
                    data.load(path).unwrap();
                } else {
                    let file = PathBuf::from(data.path.clone());
                    let parent_path = file.parent().unwrap().display().to_string();
                    data.load(parent_path).unwrap();
                }
            } else {
                let path = commands[1];
                data.load(path.to_string()).unwrap();
            }
            return false;
        }
        _ => {
            data.mode = 'n';
            return false;
        }
    }
}
