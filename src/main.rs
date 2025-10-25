use std::io::{self, Write};
use std::fs::{self, File};
use std::num::ParseIntError;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
fn main() {

    println!("Hi, I'll help you with databases. To see all the commands, enter - .help");

    loop {
    let mut command = String::new();
    if io::stdin()
    .read_line(&mut command)
    .is_err() {
        eprintln!("Failed to read line");
        continue;
    }
       
  match command.trim() {
      ".help" => {
                println!("1. .add   -> Create folder for DB (enter folder path)");
                println!("2. .write -> Create a DB file (enter file path)");
                println!("3. .exit  -> Exit the program");
                println!("Created by Hikaru.exe");
                println!("GitHub -> https://github.com/HikaruApps");
      }
      ".add" => {
        if let Err(e) = add() {
            eprintln!("Error in .add: {}", e);
        }
      }
      ".write" => {
        if let Err(e) = write_db() {
            eprintln!("Error in .write: {}", e);
        }
      }
      ".exit" => {
        println!("The program ends");
        break;
      }
      other => {
        println!("Unknow command: '{}'. For a list of commands, use => .help", other);
      }
  }
}
}

fn add() -> io::Result<()> {

    println!("Specify the path to create the file");

    let mut input = String::new();
    io::stdin().read_line(&mut input); 
    let folder_path= input.trim();

    let path = Path::new(folder_path);

    if path.exists() {
        println!("This folder already exists: {}", folder_path);
    } else {
        fs::create_dir_all(path)?;
        println!("The folder named <{}> has been successfully created.", folder_path);
    }

    Ok(())
}

fn write_db() -> io::Result<()> {
    println!("Specify the file path to create the DB file (e.g. E:\\path\\db.txt):");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let file_path = input.trim();

    let path = PathBuf::from(file_path);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(&path)?;
    writeln!(file, "#DBLite\n# created by Hikaru.exe")?;

    println!("File created: {}", path.display());
    Ok(())
}