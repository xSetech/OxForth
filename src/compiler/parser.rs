//! Parser

use std::vec::Vec;

use super::CompilerError;

use super::scanner::Role;
use super::scanner::Token;

#[derive(PartialEq, Debug)]
pub enum Operation {
    NOP,
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Operation>, CompilerError> {
    let operations: Vec<Operation> = tokens.iter().map(
        |token: &Token| {
            match token.role {
                Role::NOP => Operation::NOP,
            }
        }
    ).collect();
    return Result::Ok(operations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_parse_test() {

        let tokens: [Token; 3] = [
            Token {
                name: "a",
                role: Role::NOP,
            },
            Token {
                name: "b",
                role: Role::NOP,
            },
            Token {
                name: "c",
                role: Role::NOP,
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
