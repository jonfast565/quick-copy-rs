use std::path::Path;
use std::{fs, io};

pub fn enumerate_files(path: &str) -> io::Result<Vec<String>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    println!("{} entries...", entries.len());

    for e in entries.clone() {
        println!("{}", e.to_str().unwrap());
    }

    let result = entries
        .iter()
        .map(|x| x.clone().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>();

    Ok(result)
}

pub fn get_all_files(dir: &String) -> io::Result<Vec<String>> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            get_all_files_windows(dir)
        } else {
            get_all_files_others(dir)
        }
    }
}

#[cfg(not(windows))]
pub fn get_all_files_others(dir: &String) -> io::Result<Vec<String>> {
    let mut result = Vec::<String>::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && dir != path {
            let cur_path = path.into_os_string().into_string().unwrap();
            result.push(cur_path.clone());
            let mut results = get_all_files_others(Path::new(&cur_path))?;
            result.append(&mut results);
        } else {
            result.push(path.into_os_string().into_string().unwrap());
        }
    }
    Ok(result)
}

#[cfg(windows)]
pub fn get_all_files_windows(dir: &String) -> io::Result<Vec<String>> {
    Ok(Vec::new())
}
