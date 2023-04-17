//! OxForth CLI - Read, Execute, Print Loop

use std::io::{stdin, stdout, Write};

use oxforth::compiler::scanner::scan;
use oxforth::compiler::parser::parse;
use oxforth::vm::interpreter::execute;
use oxforth::vm::VM;

pub fn repl() {
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
        print!("> scan ");
        stdout().flush().unwrap();
        let scan_result = scan(&input, &mut vm);
        if let Err(err) = scan_result {
            print!("error: {}\n\n", err.msg);
            stdout().flush().unwrap();
            continue
        }
        print!("ok: {} tokens\n", vm.tokens.len());
        for token in vm.tokens.iter() {
            print!("\t{:?}\n", token);
        }

        // Parse the scanned tokens into operations
        print!("> parse ");
        stdout().flush().unwrap();
        let parse_result = parse(&mut vm);
        if let Err(ref err) = parse_result {
            println!("error: {}", err.msg);
            stdout().flush().unwrap();
        } else {
            println!("ok: {} stack items, {} operations", vm.data_stack.len(), vm.operations.len());
        }
        println!("\tdata stack:");
        for data in vm.data_stack.iter() {
            println!("\t\t{:?}", data);
        }
        println!("\toperation stack:");
        for operation in vm.operations.iter() {
            println!("\t\t{:?}", operation);
        }
        if parse_result.is_err() {
            println!("");
            continue;
        }

        // Apply the operations against the VM
        println!("> output:");
        let apply_result = execute(&mut vm);
        stdout().flush().unwrap();
        print!("> execute ");
        if let Err(err) = apply_result {
            println!("error: {}", err.msg);
            stdout().flush().unwrap();
        } else {
            println!("ok");
        }
        println!("\tdata stack:");
        for data in vm.data_stack.iter() {
            println!("\t\t{:?}", data);
        }
        println!("\toperation stack:");
        for operation in vm.operations.iter() {
            println!("\t\t{:?}", operation);
        }
        println!("");
        stdout().flush().unwrap();
    }
}
