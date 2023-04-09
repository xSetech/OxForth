//! Compiler

pub mod scanner;
pub mod parser;

#[derive(Debug)]
pub struct CompilerError {
    /// `msg` describes what went wrong
    pub msg: String
}
