//! Parser

use super::CompilerError;

use super::scanner::Symbol;

use super::super::vm::VM;

/// Operations change VM state (e.g. dictionary, stacks, etc).
#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum Operation {

    /// Non-operation
    NOP,

    /// Non-operation, but VM operations applied count increments
    NOP_INC,

}

/// Translate tokens into VM operations
pub fn parse(vm: &mut VM) -> Result<(), CompilerError> {

    while let Some(token) = vm.token_stack.pop() {
        match token.symbol {

            Symbol::NUMBER => {
                vm.operation_stack.push(
                    Operation::NOP,
                );
            },

            Symbol::WORD => {
                vm.operation_stack.push(
                    Operation::NOP,
                );
            },

            Symbol::UNDEFINED => {
                vm.token_stack.clear();
                return Result::Err(
                    CompilerError {
                        msg: String::from(format!("undefined word: {:?}", token)),
                    }
                );
            },

        };
    }

    return Result::Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::scanner::Token;

    #[test]
    fn base_parse_test() {

        let mut vm: VM = VM::default();

        vm.token_stack = vec![
            Token {
                token: String::from("1"),
                symbol: Symbol::NUMBER,
            },
            Token {
                token: String::from("2"),
                symbol: Symbol::NUMBER,
            },
            Token {
                token: String::from("3"),
                symbol: Symbol::NUMBER,
            },
        ];

        assert!(parse(&mut vm).is_ok());
        assert_eq!(
            vm.operation_stack,
            vec![
                Operation::NOP,
                Operation::NOP,
                Operation::NOP,
            ]
        );

    }

}
