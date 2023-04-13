/*  Copyright © 2023 Seth Junot
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, version 3 of the License.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! OxForth - binary entry point

use std::io::{stdin, stdout, Write};

use oxforth::compiler::scanner::scan;
use oxforth::compiler::parser::parse;
use oxforth::vm::interpreter::execute;
use oxforth::vm::VM;

/// Entry point
fn main() {

    println!("OxForth 0.1.0 - https://github.com/xSetech/OxForth");
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
        print!("ok: {} tokens\n", vm.token_stack.len());
        for token in vm.token_stack.iter() {
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
            println!("ok: {} stack items, {} operations", vm.data_stack.len(), vm.operation_stack.len());
        }
        println!("\tdata stack:");
        for data in vm.data_stack.iter() {
            println!("\t\t{:?}", data);
        }
        println!("\toperation stack:");
        for operation in vm.operation_stack.iter() {
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
        for operation in vm.operation_stack.iter() {
            println!("\t\t{:?}", operation);
        }
        println!("");
        stdout().flush().unwrap();

    }
}
