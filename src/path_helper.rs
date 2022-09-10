use crate::constant::*;
use crate::mode::Mode;
use crate::rich_vec::*;
use std::env;
use std::fs;
use std::fs::read_dir;
use std::io::Result;
use std::vec::*;

fn read_single_file<T: Into<String>>(path: T) -> Vec<String> {
    let body = fs::read_to_string(path.into());
    match body {
        Err(_) => Vec::new(),
        Ok(body) => {
            let mut vec: Vec<String> = Vec::new();
            for line in body.lines() {
                let trimed = line.trim();
                if !trimed.is_empty() {
                    vec.push(trimed.to_string())
                }
            }
            vec
        }
    }
}

pub fn read_env() -> Vec<String> {
    match env::var(ENV_KEY) {
        Err(_) => Vec::new(),
        Ok(value) => value
            .split(":")
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    }
}

pub fn read_paths(mode: &Mode) -> Result<String> {
    let (mut p, mut s): (Vec<String>, Vec<String>) = read_dir(DIR_PATH)?
        .flat_map(|p| p.ok())
        .flat_map(|p| p.file_name().to_os_string().into_string().ok())
        .partition(|file_name| file_name.starts_with("0"));

    p.sort();

    s.sort();

    let mut file_list: Vec<String> = Vec::new();

    let prefix = format!("{}/", DIR_PATH).to_string();

    file_list.extend_with_prefix(p, &prefix);
    file_list.push(DEFAULT_PATH.to_string());
    file_list.extend_with_prefix(s, &prefix);

    let mut path_vec: Vec<String> = Vec::new();

    file_list
        .into_iter()
        .for_each(|i| path_vec.extend_if_not_exists(read_single_file(i)));

    path_vec.extend_if_not_exists(read_env());

    match mode {
        Mode::CSH => Ok(format!("setenv PATH \"{}\";", path_vec.join(":"))),
        _ => Ok(format!("PATH=\"{}\"; export PATH;", path_vec.join(":"))),
    }
}
