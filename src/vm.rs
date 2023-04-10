//! Virtual Machine

use std::collections::HashMap;

use super::compiler::scanner::Token;
use super::compiler::parser::Operation;

pub mod interpreter;

/// Data on the data stack is a collection of bytes representing these types.
/// Casting to different types is done as needed depending on the executing word
/// and whether it's even possible.
pub enum DataType {
    STRING,
    NUMBER,
}

/// Data that can be found on the "data stack"
pub struct Data {
    pub value: String,
    pub data_type: DataType,
}

#[derive(Default)]
pub struct VM<'vm> {

    /// Number of operations applied to the VM instance
    pub _ops_applied: u64,

    /// Words are mapped to an ordered collection of VM operations
    pub dictionary: HashMap<&'vm str, Vec<Operation>>,

    /// Stack:  Tokens found by scanning the parse area
    pub token_stack: Vec<Token>,

    /// Stack:  Operations to perform on the VM and its stacks
    pub operation_stack: Vec<Operation>,

    /// Stack:  The stack used by operations
    pub data_stack: Vec<Data>,

}

#[derive(Debug)]
pub struct VirtualMachineError {
    /// `msg` describes what went wrong
    pub msg: String
}

#[allow(non_snake_case)]
impl<'vm> VM<'vm> {

    /// Define words at runtime:  Implementation-defined
    pub fn define_words__implementation(&mut self) {

        self.dictionary.insert(
            "NOP",
            vec![
                Operation::NOP,
            ],
        );

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_test_dictionary() {
        let mut vm: VM = VM::default();
        vm.dictionary.insert(
            "word",
            vec![
                Operation::NOP,
            ],
        );
    }

}
