//! Virtual Machine

use std::collections::HashMap;

use super::compiler::parser::Operation;

pub mod interpreter;

#[derive(Default)]
pub struct VM<'vm> {

    /// Number of operations applied to the VM instance
    pub _ops_applied: u64,

    /// Words are mapped to an ordered collection of VM operations
    pub dictionary: HashMap<&'vm str, Vec<Operation>>

}

#[derive(Debug)]
pub struct VirtualMachineError {
    /// `msg` describes what went wrong
    pub msg: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_test_dictionary() {
        let mut vm: VM = VM::default();
        vm.dictionary.insert("word", vec![Operation::NOP]);
    }

}
