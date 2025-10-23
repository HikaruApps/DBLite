use std::io;
fn main() {

    let mut help = String::new();
    println!("Hi, I'll help you with databases. To see all the commands, enter - .help");

    io::stdin()
    .read_line(&mut help)
    .expect("Failed to read line");
       
       let help = help.trim();
       
    if help == ".help" {
        println!("1. .add -> Create DB file");
        println!("2. .write -> Write a DB file");
    } else {
        println!("Unable to read line. For a list of commands, use -> .help");
    }
}