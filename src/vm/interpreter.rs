//! Interpretation

use std::process;

use super::{Data, VM};
use super::VirtualMachineError;

use super::super::compiler::parser::Operation;

pub fn execute(vm: &mut VM) -> Result<(), VirtualMachineError> {
    while let Some(operation) = vm.operation_stack.pop() {
        match operation {

            // Non-operational / internal test ops
            Operation::NOP => {
                continue;
            },
            Operation::NOP_INC => (),

            // Core words that happen to be VM operations, in alphabetical order.
            Operation::BYE => {
                println!("It's time to say goodbye~");
                process::exit(0);
            },
            Operation::DROP => {
                let x: Option<Data> = vm.data_stack.pop();
                if x.is_none() {
                    return Result::Err(
                        VirtualMachineError {
                            msg: String::from("stack underflow"),
                        }
                    );
                }
            },
            Operation::DUP => {
                let x: Option<&Data> = vm.data_stack.last();
                if x.is_none() {
                    return Result::Err(
                        VirtualMachineError {
                            msg: String::from("stack underflow"),
                        }
                    );
                }
                let x: &Data = x.unwrap();
                let x2: Data = x.clone();
                vm.data_stack.push(x2);
            }

        }
        vm._ops_applied += 1;
    }
    return Result::Ok(());
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    use super::super::DataType;

    #[test]
    fn base_interpret_test() {
        let mut vm: VM = VM::default();
        vm.operation_stack.push(Operation::NOP);
        vm.operation_stack.push(Operation::NOP);
        assert_eq!(
            execute(&mut vm).unwrap(),
            (),
        );
    }

    #[test]
    fn operation_test__drop() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack w/ drop
        assert!(vm.data_stack.is_empty());
        vm.operation_stack.push(Operation::DROP);
        assert!(execute(&mut vm).is_err());

        // case:  drop removes a single stack item from the top
        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
            Data {
                value: String::from("item2"),  // <- top of stack
                data_type: DataType::STRING,
            },
        ];
        vm.operation_stack = vec![Operation::DROP];
        assert!(execute(&mut vm).is_ok());
        assert_eq!(
            vm.data_stack,
            vec![
                Data {
                    value: String::from("item1"),
                    data_type: DataType::STRING,
                },
            ]
        );

    }

    #[test]
    fn operation_test__dup() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack w/ drop
        assert!(vm.data_stack.is_empty());
        vm.operation_stack.push(Operation::DUP);
        assert!(execute(&mut vm).is_err());

        // case:  drop removes a single stack item from the top
        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
            Data {
                value: String::from("item2"),  // <- top of stack
                data_type: DataType::STRING,
            },
        ];
        vm.operation_stack = vec![Operation::DUP];
        assert!(execute(&mut vm).is_ok());
        assert_eq!(
            vm.data_stack,
            vec![
                Data {
                    value: String::from("item1"),
                    data_type: DataType::STRING,
                },
                Data {
                    value: String::from("item2"),
                    data_type: DataType::STRING,
                },
                Data {
                    value: String::from("item2"),  // <- top of stack
                    data_type: DataType::STRING,
                },
            ]
        );

    }

}
