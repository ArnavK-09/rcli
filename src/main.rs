use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "hello" => println!("Hello, world!"),
            "goodbye" => println!("Goodbye, world!"),
            _ => println!("Invalid command"),
        }
    } else {
        println!("Please provide a command (hello or goodbye)");
    }
}