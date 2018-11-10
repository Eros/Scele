
extern crate walkdir;

use std::io::{stdin, stdout, Write};
use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {

    let mut input = String::new();
    println!("Change directory? (Y/N)");
    stdin().read_line(&mut input).expect("Invalid string");

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
    let file_type = String::new();
    println!("Enter file type: ");
    stdin().read_line(&mut file_type).expect("Invalid string");

    for entry in Walkdir::new(file_type).into_iter().filter_entry(|e| !is_hidden(e))){
        let entry = entry.unwrap();
        if entry.ends_with(file_type){
            println!("Found: {}", entry.path().display())
        }
    }
}


fn is_hiden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}
