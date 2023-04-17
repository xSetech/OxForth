//! OxForth CLI - Read, Execute, Print Loop

use std::io::{stdin, stdout, Write};

use oxforth::compiler::scanner::scan;
use oxforth::compiler::parser::parse;
use oxforth::vm::interpreter::execute;
use oxforth::vm::{DataType, VM};

use super::arguments::Options;

pub fn repl(options: &Options) {
    println!("Ctrl-C to exit");
    println!("");

    let mut vm = VM::default();
    vm.define_core_words();

    stdout().flush().unwrap();

    loop {
        print!("< ");
        stdout().flush().unwrap();

        // Read text from the user
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            continue;
        }

        // Scan the input for tokens
        if options.verbose {
            print!("> scan ");
            stdout().flush().unwrap();
        }
        let scan_result = scan(&input, &mut vm);
        if let Err(err) = scan_result {
            print!("error: {}\n\n", err.msg);
            stdout().flush().unwrap();
            continue
        }
        if options.verbose {
            print!("ok: {} tokens\n", vm.tokens.len());
            for token in vm.tokens.iter() {
                print!("\t{:?}\n", token);
            }
        }

        // Parse the scanned tokens into operations
        if options.verbose {
            print!("> parse ");
            stdout().flush().unwrap();
        }
        let parse_result = parse(&mut vm);
        if let Err(ref err) = parse_result {
            println!("error: {}", err.msg);
            stdout().flush().unwrap();
        } else if options.verbose {
            println!("ok: {} stack items, {} operations", vm.data_stack.len(), vm.operations.len());
        }
        if options.verbose {
            println!("\tdata stack:");
            for data in vm.data_stack.iter().rev() {
                println!("\t\t{:?}", data);
            }
            println!("\toperations:");
            for operation in vm.operations.iter() {
                println!("\t\t{:?}", operation);
            }
        }
        if parse_result.is_err() {
            if options.verbose {
                println!("");
            }
            continue;
        }

        // Apply the operations against the VM
        if options.verbose {
            println!("> output:");
        }
        let apply_result = execute(&mut vm);
        stdout().flush().unwrap();
        if options.verbose {
            print!("> execute ");
        }
        if let Err(ref err) = apply_result {
            println!("error: {}", err.msg);
            stdout().flush().unwrap();
        } else if options.verbose {
            println!("ok");
        }
        if options.verbose {
            println!("\tdata stack:");
            for data in vm.data_stack.iter().rev() {
                println!("\t\t{:?}", data);
            }
            println!("\toperations:");
            for operation in vm.operations.iter() {
                println!("\t\t{:?}", operation);
            }
            println!("");
        } else {
            if let Some(cell) = vm.data_stack.last() {
                match cell.data_type {
                    DataType::NUMBER => {
                        println!("{}", cell.value.parse::<i64>().unwrap())
                    },
                    DataType::STRING => {
                        println!("{}", cell.value);
                    }
                }
            } else {
                if apply_result.is_ok() {
                    println!("ok");
                }
            }
        }
        stdout().flush().unwrap();
    }
}
