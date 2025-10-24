use std::io;
fn main() {

    println!("Hi, I'll help you with databases. To see all the commands, enter - .help");

    loop {
    let mut help = String::new();

    io::stdin()
    .read_line(&mut help)
    .expect("Failed to read line");
       
    if help.trim() == ".help" {
        println!("1. .add -> Create DB file");
        println!("2. .write -> Write a DB file");
        println!("3. .exit -> Exit the program")
        
    } else {
        println!("Unable to read line. For a list of commands, use -> .help");
    }
    if help.trim() == ".exit" {
        println!("The program ends");
        break;
    }
}
}