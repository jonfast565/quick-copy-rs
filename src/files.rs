use std::{fs, io};

pub fn enumerate_files(path: &str) -> io::Result<()> {
    let mut entries =  fs::read_dir(path)?
    .map(| res | res.map( |e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    println!("{} entries...", entries.len());
    for e in entries {
        println!("{}", e.to_str().unwrap());
    }

    Ok(())
}
