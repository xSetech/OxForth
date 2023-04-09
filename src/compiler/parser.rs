//! Parser

use std::vec::Vec;

use super::CompilerError;

use super::scanner::Symbol;
use super::scanner::Token;

#[derive(PartialEq, Debug)]
pub enum Operation {
    NOP,
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Operation>, CompilerError> {
    let mut operations: Vec<Operation> = Vec::new();
    for token in tokens.iter() {
        match token.symbol {
            Symbol::NUMBER => {
                operations.push(Operation::NOP);
            },
            Symbol::WORD => {
                operations.push(Operation::NOP);
            },
            Symbol::UNDEFINED => {
                return Result::Err(
                    CompilerError {
                        msg: String::from(format!("undefined word: {:?}", token)),
                    }
                );
            },
        };
    }
    return Result::Ok(operations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_parse_test() {

        let tokens: [Token; 3] = [
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

        assert_eq!(
            parse(&tokens).unwrap(),
            vec![
                Operation::NOP,
                Operation::NOP,
                Operation::NOP,
            ]
        );

    }

}
