//! OxForth CLI - Argument handling

use std::env::{Args, args};
use std::iter::{ExactSizeIterator, Iterator};

#[derive(Debug, PartialEq)]
pub struct ArgParseError {
    /// `msg` describes what went wrong
    pub msg: String,
}

/// Behaviors of the CLI based on the arguments passed by the user.
#[derive(Debug, PartialEq)]
pub enum Behavior {

    /// Read, Execute, Print Loop
    REPL,

    /// Display standard CLI help
    HELP,

}

/// Options that affect the behavior of the CLI.
#[derive(Debug, PartialEq)]
pub struct Options {
    pub verbose: bool,
}

/// Helpful information about the CLI printed out via --help
const HELP: &str = "
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3 of the License. This program
comes with ABSOLUTELY NO WARRANTY.

Arguments and options:
    --help, -h          Print the text you're currently reading.
    --verbose, -v       REPL will print more details when executing.
";

/// Standard print out of helpful information about the CLI
pub fn help() {
    println!("{}", HELP);
}

/// Parse arguments from the environmental arguments iterator over Strings.
/// This is a separate method from parse() for testability.
fn parse_args_iter<StringIterator>(arguments: StringIterator) -> Result<(Behavior, Options), ArgParseError>
where
    StringIterator: Iterator<Item = String> + ExactSizeIterator,
{

    // The first "argument" is always ignored, as it's the cli name or path.
    let arguments = arguments.skip(1);

    // Default behavior: a low-verbosity REPL
    if arguments.len() == 0 {
        return Result::Ok(
            (
                Behavior::REPL,
                Options {
                    verbose: false,
                }
            ),
        );
    }

    let behavior: Behavior = Behavior::REPL;
    let mut options: Options = Options {
        verbose: false,
    };

    // Dump the iterator, since multiple passes over args may be needed.
    let arguments: Vec<String> = arguments.collect();

    // Any placement of --help is sufficient to get help
    if arguments.iter().any( |arg| { arg == "--help" || arg == "-h" }) {
        return Result::Ok((Behavior::HELP, options));
    }

    for arg in arguments.iter() {

        if arg == "--verbose" {
            options.verbose = true;
            continue
        }

        return Result::Err(
            ArgParseError {
                msg: String::from(format!("unknown argument: {}", arg)),
            }
        )

    }

    return Result::Ok((behavior, options));

}

/// Parse arguments given to the CLI into a decision about how the CLI should behave.
/// The first "argument" is always ignored, as it's the cli name or path.
pub fn parse() -> Result<(Behavior, Options), ArgParseError> {
    let arguments: Args = args();
    return parse_args_iter(arguments);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::vec::{Vec, IntoIter};

    struct TestArgs {
        inner: IntoIter<String>,
    }

    impl Iterator for TestArgs {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            return self.inner.next();
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            return (self.inner.len(), Some(self.inner.len()));
        }
    }

    impl ExactSizeIterator for TestArgs {}

    macro_rules! test_case_args_and_result {
        ($cmdline_args:expr, $expected_result:expr) => {{
            let args: Vec<&str> = Vec::from($cmdline_args);
            let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();
            let test_args: TestArgs = TestArgs {
                inner: args.into_iter(),
            };
            let parse_args_result = parse_args_iter(test_args);
            assert_eq!(
                parse_args_result,
                $expected_result
            );
        }};
    }

    #[test]
    pub fn test_parse_args_iter() {

        test_case_args_and_result!(
            ["oxforth", "--invalid-argument"],
            Err(ArgParseError { msg: String::from("unknown argument: --invalid-argument") })
        );

        test_case_args_and_result!(
            ["oxforth", "--invalid-argument", "--help"],
            Ok((Behavior::HELP, Options {
                verbose: false,
            }))
        );

        test_case_args_and_result!(
            ["oxforth", "--help", "--invalid-argument"],
            Ok((Behavior::HELP, Options {
                verbose: false,
            }))
        );

        test_case_args_and_result!(
            ["oxforth", "--invalid-argument", "-h"],
            Ok((Behavior::HELP, Options {
                verbose: false,
            }))
        );

        test_case_args_and_result!(
            ["oxforth"],
            Ok((Behavior::REPL, Options {
                verbose: false,
            }))
        );

        test_case_args_and_result!(
            ["oxforth", "--verbose"],
            Ok((Behavior::REPL, Options {
                verbose: true,
            }))
        );

    }

}
