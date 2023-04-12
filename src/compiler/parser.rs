//! Parser

use super::CompilerError;

use super::scanner::Symbol;

use super::super::vm::{Data, DataType, VM};

/// Operations change VM state (e.g. dictionary, stacks, etc).
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {

    /// Non-operation
    NOP,

    /// Non-operation, but VM operations applied count increments
    NOP_INC,

    /// ( n -- u ) u is the absolute value of n.
    /// https://forth-standard.org/standard/core/ABS
    ABS,

    /// "Return control to the host operating system"
    /// https://forth-standard.org/standard/tools/BYE
    BYE,

    /// ( x -- ) Remove x from the stack.
    /// https://forth-standard.org/standard/core/DROP
    DROP,

    /// ( x -- x x ) Duplicate x.
    /// https://forth-standard.org/standard/core/DUP
    DUP,

    /// ( x -- flag ) flag is true if and only if x is equal to zero.
    /// https://forth-standard.org/standard/core/ZeroEqual
    ZERO_EQ,

    /// ( n -- flag ) flag is true if and only if n is less than zero.
    /// https://forth-standard.org/standard/core/Zeroless
    ZERO_LT,

    /// ( n -- flag ) flag is true if and only if n is greater than zero.
    /// https://forth-standard.org/standard/core/Zeromore
    ZERO_GT,

    /// ( x -- flag ) flag is true if and only if x is not equal to zero.
    /// https://forth-standard.org/standard/core/Zerone
    ZERO_NE,

}

/// Translate tokens into VM operations
pub fn parse(vm: &mut VM) -> Result<(), CompilerError> {

    while let Some(token) = vm.token_stack.pop() {
        match token.symbol {

            Symbol::NUMBER => {
                vm.data_stack.push(
                    Data {
                        value: String::from(token.token),
                        data_type: DataType::NUMBER,
                    }
                );
            },

            Symbol::WORD => {
                let word_ops: Vec<Operation> = vm.dictionary.get(token.token.as_str()).unwrap().to_vec();
                vm.operation_stack.extend(word_ops);
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
    fn parser_test_numbers() {
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
            vm.data_stack,
            vec![
                Data {
                    value: String::from("3"),
                    data_type: DataType::NUMBER,
                },
                Data {
                    value: String::from("2"),
                    data_type: DataType::NUMBER,
                },
                Data {
                    value: String::from("1"),
                    data_type: DataType::NUMBER,
                },

            ]
        );
    }

    #[test]
    fn parser_test_words() {
        let mut vm: VM = VM::default();
        vm.dictionary.insert(
            "NOP_INC",
            vec![
                Operation::NOP_INC,
            ],
        );
        vm.token_stack = vec![
            Token {
                token: String::from("1"),
                symbol: Symbol::NUMBER,
            },
            Token {
                token: String::from("NOP_INC"),
                symbol: Symbol::WORD,
            },
        ];
        assert!(parse(&mut vm).is_ok());
        assert_eq!(
            vm.data_stack,
            vec![
                Data {
                    value: String::from("1"),
                    data_type: DataType::NUMBER,
                },
            ]
        );
        assert_eq!(
            vm.operation_stack,
            vec![
                Operation::NOP_INC,
            ]
        );
    }

}
