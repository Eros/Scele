
use std::io::{stdin, stdout, Write};
use std::env;
use std::path::Path;
use std::fs;

fn main() {

    let mut input = String::new();
    println!("Change directory? (Y/N)");
    stdin().read_line(&mut input).expect("Did not enter a string");

    let input = input.trim();

    if input == "Y"{
        println!("Enter new directory: ");
        let mut new_dir = String::new();
        let root = Path::new(&new_dir);
        assert!(env::set_current_dir(&root).is_ok());
        println!("Changed directory to: {}", root.display());
        scan_dir();
    } else if input == "N" {
        scan_dir(); 
    } else {
        println!("Invalid option");
        return;
    }
}

fn scan_dir(){
    let mut input = String::new();
    println!("Enter the file type to search for (.py, .docx, .java etc)");
    stdin().read_line(&mut input).expect("Did not enter a string");

    let input = input.trim();
    let paths = fs::read_dir("./").unwrap();
 
    for path in paths {

    }
}
