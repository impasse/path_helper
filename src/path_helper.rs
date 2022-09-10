use std::fs;
use std::io::Result;
use std::vec::*;

fn read_single_file<T: Into<String>>(path: T) -> Vec<String> {
    let body = fs::read_to_string(path.into());
    match body {
        Err(_) => {
            Vec::new()
        }
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

pub fn read_paths() -> Result<String> {
    let sub_files = fs::read_dir("/etc/paths.d")?
    .flat_map(|res| {
        res.map(|p| p.path())
    })
    .flat_map(|p| p.into_os_string().into_string())
    .collect::<Vec<_>>();

    let (mut p, mut s): (Vec<_>, Vec<_>) = sub_files
    .iter()
    .partition(|p| p.starts_with("0"));

    p.sort();

    s.sort();

    let mut path_vec: Vec<String> = Vec::new();

    p
    .into_iter()
    .for_each(|i| {
        path_vec.extend(read_single_file(i))
    });

    path_vec.extend(read_single_file("/etc/paths"));

    s
    .into_iter()
    .for_each(|i| {
        path_vec.extend(read_single_file(i))
    });

    Ok(format!("PATH=\"{}\"; export PATH;", path_vec.join(":")))
}
