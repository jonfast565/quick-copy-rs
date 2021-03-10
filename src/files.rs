use std::{fs, io};
use std::path::Path;

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

pub fn visit_all(dir: &Path) -> io::Result<Vec<String>> {
    let mut result = Vec::<String>::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && dir != path {
            let cur_path = path.into_os_string().into_string().unwrap();
            result.push(cur_path.clone());
            let mut results = visit_all(Path::new(&cur_path))?;
            result.append(&mut results);
        } else {
            result.push(path.into_os_string().into_string().unwrap());
        }
    }

    for e in result.clone() {
        println!("{}", e);
    }

    Ok(result)
}
