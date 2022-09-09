use std::io;

pub fn start() {

    //Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    println!("Please enter a command 'Add Name to Department':");
 let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");




}

fn parse_command(command: &str) -> Option<(&str, &str)>{
    let keywords = command.split_whitespace();
    if keywords.next() != Some("Add"){
        return None;
    }
    let name = keywords.next();
    if keywords.next(){

    }
}
