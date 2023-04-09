//! Scanner

use std::vec::Vec;

use super::CompilerError;

#[derive(PartialEq, Debug)]
pub enum Role {
    NOP,
}

#[derive(PartialEq, Debug)]
pub struct Token<'token> {
    pub name: &'token str,
    pub role: Role,
}

pub fn scan(string: &str) -> Result<Vec<Token>, CompilerError> {
    let scanned_string: Vec<Token> = string.split(' ').map(
        |part: &str| {
            Token {
                name: part,
                role: Role::NOP,
            }
        }
    ).collect();
    return Result::Ok(scanned_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_scan_test() {

        assert_eq!(
            scan("a b c").unwrap(),
            vec![
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
            ]
        );

    }

}
