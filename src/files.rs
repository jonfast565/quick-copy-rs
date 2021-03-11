#[cfg(windows)] extern crate winapi;

use std::path::Path;
use std::{fs, io};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use std::io::Error;
use std::str;

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
    let mut result = Vec::<String>::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && Path::new(&dir) != path {
            let cur_path = path.into_os_string().into_string().unwrap();
            result.push(cur_path.clone());
            let mut results = get_all_files(&cur_path)?;
            result.append(&mut results);
        } else {
            result.push(path.into_os_string().into_string().unwrap());
        }
    }
    Ok(result)
}

