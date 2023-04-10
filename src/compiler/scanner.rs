//! Scanner

use std::vec::Vec;

use super::super::vm::VM;

use super::CompilerError;

/// Forth is about words and numbers.
///
/// Words come in two flavors:
///     - Defined words, which are words kept in the VM 'dictionary'.
///     - Undefined words, which are words missing from the VM dictionary.
///
/// Undefined words that match the number pattern are considered to be numbers.
/// Otherwise, they're considered to be undefined (a symbol neither defined in
/// the dictionary nor a number).
#[derive(PartialEq, Debug)]
pub enum Symbol {
    WORD,
    NUMBER,
    UNDEFINED,
}

/// Forth text (e.g. from the user or a file) is first scanned for Tokens.
///
/// During scanning, the text is determined to be either a defined word, a
/// number, or an undefined word. The text and its symbolism are associated via
/// this struct.
#[derive(PartialEq, Debug)]
pub struct Token {
    pub token: String,
    pub symbol: Symbol,
}

/// Given the dictionary, are the given bytes a word or number?
fn token_from_bytes(word_or_number: &mut Vec<u8>, vm: &VM) -> Token {
    let name: String = String::from_utf8(word_or_number.to_vec()).unwrap();
    word_or_number.clear();
    for char in name.as_bytes().iter() {
        // If the number contains not-numbers, then it's probably a word
        if !char.is_ascii_digit() {
            let token: String = String::from(name);
            match vm.dictionary.contains_key(token.as_str()) {
                true => {
                    return Token {
                        token: token,
                        symbol: Symbol::WORD,
                    };
                },
                false => {
                    return Token {
                        token: token,
                        symbol: Symbol::UNDEFINED,
                    };
                },
            }
        }
    }
    return Token {
        token: String::from(name),
        symbol: Symbol::NUMBER,
    }
}

/// Scan the 'parse area' for tokens (see Token defined above). Only ASCII
/// characters are permitted in the parse area. Whitespace is required to be
/// present at the end of the parse area, but is otherwise ignored. Scanning
/// stops at the first word, whether defined or undefined.
pub fn scan(string: &str, vm: &mut VM) -> Result<(), CompilerError> {

    // For simplicity, assume the parse area is all ASCII characters
    if !string.is_ascii() {
        return Result::Err(
            CompilerError {
                msg: String::from("parse space contains non-ascii characters"),
            }
        )
    }

    let mut word_or_number: Vec<u8> = Vec::new();

    // Search for numbers and words, stopping on the first defined "word".
    for char in string.as_bytes().iter() {

        // Ignore characters that can't be displayed (whitespace, control chars, etc)
        if !char.is_ascii_graphic() {
            if word_or_number.is_empty() {
                continue;
            }
            let token: Token = token_from_bytes(&mut word_or_number, vm);
            match token.symbol {
                Symbol::NUMBER => {
                    vm.token_stack.push(token);
                    continue;
                },
                Symbol::WORD | Symbol::UNDEFINED => {
                    vm.token_stack.push(token);
                    return Result::Ok(());
                },
            }
        }

        word_or_number.push(*char);
    }

    // The parse area must always end with a new line (or any non-graphic character).
    if !word_or_number.is_empty() {
        return Result::Err(
            CompilerError {
                msg: String::from("parse space doesn't end with whitespace"),
            }
        )
    }

    return Result::Ok(());

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test:  Assert numbers are recognized
    #[test]
    fn scan_test_numbers() {

        // test setup
        let mut vm: VM = VM::default();

        // test cases
        assert!(scan("1 2 3\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
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
            ]
        );

    }

    /// Test:  Assert whitespace is ignored during parse area scanning
    #[test]
    fn scan_test_whitespace() {

        // test setup
        let mut vm: VM = VM::default();

        // test cases
        assert!(scan("1 2 3\n", &mut vm).is_ok());
        assert_eq!(vm.token_stack.len(), 3);
        vm.token_stack.clear();

        assert!(scan("1  2  3\n", &mut vm).is_ok());
        assert_eq!(vm.token_stack.len(), 3);
        vm.token_stack.clear();

        assert!(scan("1  2  3 \n", &mut vm).is_ok());
        assert_eq!(vm.token_stack.len(), 3);
        vm.token_stack.clear();

        assert!(scan(" 1  2  3 \n", &mut vm).is_ok());
        assert_eq!(vm.token_stack.len(), 3);
        vm.token_stack.clear();

        assert!(scan("  1  2  3 \n", &mut vm).is_ok());
        assert_eq!(vm.token_stack.len(), 3);
        vm.token_stack.clear();

        assert!(scan("  1 \x07 2  3 \n", &mut vm).is_ok());
        assert_eq!(vm.token_stack.len(), 3);
        vm.token_stack.clear();

    }

    /// Test:  Assert detection of undefined words
    #[test]
    fn scan_test_undefined_words() {

        // test setup
        let mut vm: VM = VM::default();

        // test cases
        assert!(scan("undefined_word\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
                Token {
                    token: String::from("undefined_word"),
                    symbol: Symbol::UNDEFINED,
                },
            ]
        );
        vm.token_stack.clear();

        assert!(scan(" undefined_word\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
                Token {
                    token: String::from("undefined_word"),
                    symbol: Symbol::UNDEFINED,
                },
            ]
        );
        vm.token_stack.clear();

        assert!(scan(" undefined_word\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
                Token {
                    token: String::from("undefined_word"),
                    symbol: Symbol::UNDEFINED,
                },
            ]
        );
        vm.token_stack.clear();

        assert!(scan(" 1 undefined_word\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
                Token {
                    token: String::from("1"),
                    symbol: Symbol::NUMBER,
                },
                Token {
                    token: String::from("undefined_word"),
                    symbol: Symbol::UNDEFINED,
                },
            ]
        );
        vm.token_stack.clear();

    }

    /// Test:  Assert detected of defined words
    #[test]
    fn scan_test_defined_words() {

        // test setup
        let mut vm: VM = VM::default();

        // "example" word that is initially undefined
        assert!(scan("example\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
                Token {
                    token: String::from("example"),
                    symbol: Symbol::UNDEFINED,
                }
            ],
        );
        vm.token_stack.clear();

        // define the word "example"
        vm.dictionary.insert("example", vec![]);

        // scan reports that "example" is a defined word
        assert!(scan("example\n", &mut vm).is_ok());
        assert_eq!(
            vm.token_stack,
            vec![
                Token {
                    token: String::from("example"),
                    symbol: Symbol::WORD,
                }
            ],
        );
        vm.token_stack.clear();

    }

}
