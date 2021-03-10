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

fn visit_dirs(dir: &Path) -> io::Result<Vec<String>> {
    let result = Vec::<String>::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result.push(path.into_os_string().to_string());
            let results = visit_dirs(&path)?;
            result.append(&mut results);
        } else {
            result.push(path.to_string());
        }
    }
    Ok(())
}
