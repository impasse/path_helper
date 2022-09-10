use crate::constant::*;
use crate::mode::Mode;
use crate::rich_vec::*;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::vec::*;

fn read_single_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let body = fs::read_to_string(path);
    match body {
        Err(_) => Vec::new(),
        Ok(body) => body
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>(),
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
    let mut prelude: Vec<PathBuf> = Vec::new();
    let mut successor: Vec<PathBuf> = Vec::new();

    if let Ok(dir) = fs::read_dir(DIR_PATH) {
        dir.flat_map(|e| e.ok()).for_each(|e| {
            let path = e.path();
            if e.file_name()
                .to_str()
                .map(|s| s.starts_with("0"))
                .unwrap_or(false)
            {
                prelude.push_if_not_exists(path);
            } else {
                successor.push_if_not_exists(path);
            }
        })
    }

    prelude.sort();

    successor.sort();

    let mut file_list: Vec<PathBuf> = Vec::new();

    file_list.extend_if_not_exists(prelude);
    file_list.push(PathBuf::from(DEFAULT_PATH));
    file_list.extend_if_not_exists(successor);

    let mut paths: Vec<String> = file_list
        .iter()
        .flat_map(|f| read_single_file(f))
        .collect();

    paths.extend_if_not_exists(read_env());

    let expanded: Vec<PathBuf> = paths
        .iter()
        .flat_map(|p| shellexpand::full(p))
        .map(|p| PathBuf::from(p.as_ref()))
        .filter(|pb| pb.is_dir())
        .collect();

    let string_list: Vec<String> = expanded
        .dedup_ordered()
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    match mode {
        Mode::CSH => Ok(format!("setenv PATH \"{}\";", string_list.join(":"))),
        _ => Ok(format!("PATH=\"{}\"; export PATH;", string_list.join(":"))),
    }
}
