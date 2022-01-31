use std::{
    io::prelude::*,
    fs::File,
    process::exit,
};

mod compiler; use compiler::*;
mod html;
mod compiler_line;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 { // 1 argument -> output to stdout
        println!("{}", compile_str(read(&args[1])));
        exit(0);
    }

    if args.len() == 3 { // 2 arguments -> output to file
        let output = compile_str(read(&args[1])); 
        // compile before opening file so we have it open for a minimum time

        let dest = &args[2];
        let file = File::create(dest);
        if file.is_err() {
            eprintln!("Could not write to file {}", dest);
            exit(1);
        }
        file.unwrap().write_all(output.as_bytes()).unwrap();
        exit(0);
    } 

    // 0 or 3+ arguments -> usage info
    println!("Usage: {} <input> [output]", args[0]);
    exit(1);
}

fn read(path: &str) -> String {
    let file = File::open(path);
    if file.is_err() {
        eprintln!("Could not read file {}", path);
        exit(1);
    }
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();
    contents
}
