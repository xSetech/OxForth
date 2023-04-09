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

//! OxFor - binary entry point

use std::io::{stdin, stdout, Write};

use oxfor::compiler::scanner::scan;
use oxfor::compiler::parser::parse;
use oxfor::vm::interpreter::execute;
use oxfor::vm::VM;

/// Entry point
fn main() {

    println!("OxFor 0.1.0 - https://github.com/xSetech/OxFor");
    println!("Ctrl-C to exit");
    println!("");

    let mut vm = VM::default();

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
        let scan_result = scan(&input, &vm);
        if let Err(err) = scan_result {
            print!("error: {}\n\n", err.msg);
            stdout().flush().unwrap();
            continue
        }
        let tokens = scan_result.unwrap();
        print!("ok: {} tokens\n", tokens.len());
        for token in tokens.iter() {
            print!("\t{:?}\n", token);
        }

        // Parse the scanned tokens into operations
        print!("> parse ");
        stdout().flush().unwrap();
        let parse_result = parse(tokens.as_slice());
        if let Err(err) = parse_result {
            print!("error: {}\n\n", err.msg);
            stdout().flush().unwrap();
            continue
        }
        let operations = parse_result.unwrap();
        print!("ok: {} operations\n", operations.len());
        for operation in operations.iter() {
            println!("\t{:?}", operation);
        }

        // Apply the operations against the VM
        print!("> execute ");
        let apply_result = execute(operations.as_slice(), &mut vm);
        if let Err(err) = apply_result {
            print!("error: {}\n\n", err.msg);
            stdout().flush().unwrap();
            continue
        }
        println!("ok\n");
        stdout().flush().unwrap();
    }
}
