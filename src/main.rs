use std::{fs::File, io::prelude::*, process::exit};

mod compiler;
mod compiler_line;
mod html;
mod python;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.len() > 3 { // 0 or 3+ arguments -> usage info
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }

    // compile before opening the output file so we have it open for a minimum time
    let mut output = python::execute_md(&args[1]);
    output = compiler::compile_str(output);

    if args.len() == 2 { // 1 argument -> output to stdout
        println!("{}", output);
    } else { // 2 arguments -> output to file
        let dest = &args[2];
        let file = File::create(dest);
        if file.is_err() {
            eprintln!("Could not write to file {}", dest);
            exit(1);
        }
        file.unwrap().write_all(output.as_bytes()).unwrap();
    }
    exit(0);
}

