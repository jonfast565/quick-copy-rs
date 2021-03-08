use std::{fs, io};

pub fn enumerate_files(path: &str) -> io::Result<Vec<String>> {
    let mut entries =  fs::read_dir(path)?
    .map(| res | res.map( |e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    
    println!("{} entries...", entries.len());
    for e in entries.clone() {
        println!("{}", e.to_str().unwrap());
    }
    let result = entries.iter()
        .map(|x| x.clone().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>();

    Ok(result)
}
