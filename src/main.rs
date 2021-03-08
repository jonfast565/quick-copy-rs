mod utilities;
mod paths;
mod files;
mod change_detector;
mod configuration;

fn main() {
    println!("Hello, world!");

    let result = files::enumerate_files("C:\\Users\\jnfst\\Desktop\\Files");
    match result {
        Ok(_) => println!("{}", "()"),
        Err(e) => panic!(e)
    }
}
