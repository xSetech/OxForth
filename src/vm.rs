//! Virtual Machine

use std::collections::HashMap;

use super::compiler::scanner::Token;
use super::compiler::parser::Operation;

pub mod interpreter;

/// Data on the data stack is a collection of bytes representing these types.
/// Casting to different types is done as needed depending on the executing word
/// and whether it's even possible.
#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    STRING,
    NUMBER,
}

/// Data that can be found on the "data stack"
#[derive(Clone, Debug, PartialEq)]
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
    pub fn define_core_words(&mut self) {

        macro_rules! define_single_op_word {
            ($word:expr, $operation:expr) => {{
                self.dictionary.insert(
                    $word,
                    vec![
                        $operation,
                    ],
                );
            }};
        }

        define_single_op_word!("NOP", Operation::NOP);
        define_single_op_word!("ABS", Operation::ABS);
        define_single_op_word!("+", Operation::ADD);
        define_single_op_word!("BYE", Operation::BYE);
        define_single_op_word!("=", Operation::CMP_EQ);
        define_single_op_word!("<", Operation::CMP_LT);
        define_single_op_word!(">", Operation::CMP_GT);
        define_single_op_word!("<>", Operation::CMP_NE);
        define_single_op_word!("/", Operation::DIV);
        define_single_op_word!("DROP", Operation::DROP);
        define_single_op_word!("DUP", Operation::DUP);
        define_single_op_word!("MAX", Operation::MAX);
        define_single_op_word!("MIN", Operation::MIN);
        define_single_op_word!("MOD", Operation::MOD);
        define_single_op_word!("*", Operation::MUL);
        define_single_op_word!("NEGATE", Operation::NEGATE);
        define_single_op_word!("-", Operation::SUB);
        define_single_op_word!("0=", Operation::ZERO_EQ);
        define_single_op_word!("0<", Operation::ZERO_LT);
        define_single_op_word!("0>", Operation::ZERO_GT);
        define_single_op_word!("0<>", Operation::ZERO_NE);

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::compiler::scanner::scan;
    use super::super::compiler::parser::parse;

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

    #[test]
    fn vm_test_defining_words() {
        let mut vm: VM = VM::default();
        vm.dictionary.insert(
            "NOP_INC",
            vec![
                Operation::NOP_INC,
            ],
        );
        assert_eq!(vm._ops_applied, 0);
        assert!(scan("NOP_INC\n", &mut vm).is_ok());
        assert!(parse(&mut vm).is_ok());
        assert!(interpreter::execute(&mut vm).is_ok());
        assert_eq!(vm._ops_applied, 1);
    }

}
