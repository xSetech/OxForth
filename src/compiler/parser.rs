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

    /// ( n1 | u1 n2 | u2 -- n3 | u3 ) Add n2 | u2 to n1 | u1, giving the sum n3 | u3.
    /// https://forth-standard.org/standard/core/Plus
    ADD,

    /// "Return control to the host operating system"
    /// https://forth-standard.org/standard/tools/BYE
    BYE,

    /// ( x1 x2 -- flag ) flag is true if and only if x1 is bit-for-bit the same as x2.
    /// https://forth-standard.org/standard/core/Equal
    CMP_EQ,

    /// ( n1 n2 -- flag ) flag is true if and only if n1 is greater than n2.
    /// https://forth-standard.org/standard/core/more
    CMP_GT,

    /// ( n1 n2 -- flag ) flag is true if and only if n1 is less than n2.
    /// https://forth-standard.org/standard/core/less
    CMP_LT,

    /// ( x1 x2 -- flag ) flag is true if and only if x1 is not bit-for-bit the same as x2.
    /// https://forth-standard.org/standard/core/ne
    CMP_NE,

    /// ( n1 n2 -- n3 ) Divide n1 by n2, giving the single-cell quotient n3.
    /// An ambiguous condition exists if n2 is zero.
    /// https://forth-standard.org/standard/core/Div
    DIV,

    /// ( x -- ) Remove x from the stack.
    /// https://forth-standard.org/standard/core/DROP
    DROP,

    /// ( x -- x x ) Duplicate x.
    /// https://forth-standard.org/standard/core/DUP
    DUP,

    /// ( n1 -- n2 ) Negate n1, giving its arithmetic inverse n2.
    /// https://forth-standard.org/standard/core/NEGATE
    NEGATE,

    /// ( n1 n2 -- n3 ) n3 is the greater of n1 and n2.
    /// https://forth-standard.org/standard/core/MAX
    MAX,

    /// ( n1 n2 -- n3 ) n3 is the lesser of n1 and n2.
    /// https://forth-standard.org/standard/core/MIN
    MIN,

    /// ( n1 n2 -- n3 ) Divide n1 by n2, giving the single-cell "remainder" n3.
    /// An ambiguous condition exists if n2 is zero.
    /// https://forth-standard.org/standard/core/MOD
    MOD,

    /// ( n1 | u1 n2 | u2 -- n3 | u3 ) Multiply n1 | u1 by n2 | u2 giving the product n3 | u3.
    /// https://forth-standard.org/standard/core/Times
    MUL,

    /// ( n1 | u1 n2 | u2 -- n3 | u3 ) Subtract n2 | u2 from n1 | u1, giving the difference n3 | u3.
    /// https://forth-standard.org/standard/core/Minus
    SUB,

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

    while let Some(token) = vm.tokens.pop_front() {
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
                vm.operations.extend(word_ops);
            },

            Symbol::UNDEFINED => {
                vm.tokens.clear();
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

    use std::collections::VecDeque;

    use super::super::scanner::Token;

    #[test]
    fn parser_test_numbers() {
        let mut vm: VM = VM::default();
        vm.tokens = VecDeque::from([
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
        ]);
        assert!(parse(&mut vm).is_ok());
        assert_eq!(
            vm.data_stack,
            vec![
                Data {
                    value: String::from("1"),
                    data_type: DataType::NUMBER,
                },
                Data {
                    value: String::from("2"),
                    data_type: DataType::NUMBER,
                },
                Data {
                    value: String::from("3"),
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
        vm.tokens = VecDeque::from([
            Token {
                token: String::from("1"),
                symbol: Symbol::NUMBER,
            },
            Token {
                token: String::from("NOP_INC"),
                symbol: Symbol::WORD,
            },
        ]);
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
            vm.operations,
            vec![
                Operation::NOP_INC,
            ]
        );
    }

}
