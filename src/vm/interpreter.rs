//! Interpretation

use std::process;

use super::{Data, DataType, VM};
use super::VirtualMachineError;

use super::super::compiler::parser::Operation;

/// Common cast from STRING DataType to an i64, with error-checking
fn int_and_data_from_stack<'vm>(vm: &'vm mut VM) -> Result<(i64, &'vm mut Data), VirtualMachineError> {
    let o: Option<&mut Data> = vm.data_stack.last_mut();
    if o.is_none() {
        return Result::Err(
            VirtualMachineError {
                msg: String::from("stack underflow"),
            }
        );
    }
    let d: &mut Data = o.unwrap();
    if d.data_type != DataType::NUMBER {
        return Result::Err(
            VirtualMachineError {
                msg: String::from("refusing cast: string->number"),
            }
        );
    }
    let n: i64 = d.value.parse::<i64>().unwrap();
    return Result::Ok((n, d));
}

pub fn execute(vm: &mut VM) -> Result<(), VirtualMachineError> {
    while let Some(operation) = vm.operation_stack.pop() {
        match operation {

            // Non-operational / internal test ops
            Operation::NOP => {
                continue;
            },
            Operation::NOP_INC => (),

            // Core words that happen to be VM operations, in alphabetical order.
            Operation::ABS => {
                let (n, d): (i64, &mut Data) = int_and_data_from_stack(vm)?;
                d.value = n.abs().to_string();
            },
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
            Operation::ZERO_EQ => {
                let (n, d): (i64, &mut Data) = int_and_data_from_stack(vm)?;
                let flag: bool = n == 0;
                d.value = (flag as i64).to_string();
            },
            Operation::ZERO_GT => {
                let (n, d): (i64, &mut Data) = int_and_data_from_stack(vm)?;
                let flag: bool = n > 0;
                d.value = (flag as i64).to_string();
            },
            Operation::ZERO_LT => {
                let (n, d): (i64, &mut Data) = int_and_data_from_stack(vm)?;
                let flag: bool = n < 0;
                d.value = (flag as i64).to_string();
            },
            Operation::ZERO_NE => {
                let (n, d): (i64, &mut Data) = int_and_data_from_stack(vm)?;
                let flag: bool = n != 0;
                d.value = (flag as i64).to_string();
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

    /// Helper macro to test operations that take and return a single value.
    macro_rules! single_value_op_test_case {
        ($vm:expr, $value:expr, $operation:expr, $expected:expr) => {{
            $vm.data_stack = vec![
                Data {
                    value: String::from("bottom of stack - should be ignored"),
                    data_type: DataType::STRING,
                },
                Data {
                    value: String::from($value.to_string()),
                    data_type: DataType::NUMBER,
                },
            ];
            $vm.operation_stack = vec![$operation];
            assert!(execute(&mut $vm).is_ok());
            assert_eq!(
                $vm.data_stack,
                vec![
                    Data {
                        value: String::from("bottom of stack - should be ignored"),
                        data_type: DataType::STRING,
                    },
                    Data {
                        value: String::from($expected.to_string()),
                        data_type: DataType::NUMBER,
                    }
                ]
            );
        }};
    }

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
    fn operation_test__abs() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack
        assert!(vm.data_stack.is_empty());
        vm.operation_stack.push(Operation::ABS);
        assert!(execute(&mut vm).is_err());

        // case:  refuses to cast a string -> int
        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
        ];
        vm.operation_stack = vec![Operation::ABS];
        assert!(execute(&mut vm).is_err());

        // case:  takes the absolute value of the stop stack item
        single_value_op_test_case(vm, 42, Operation::ABS, 42);
        single_value_op_test_case(vm, -42, Operation::ABS, 42);

    }

    #[test]
    fn operation_test__drop() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack
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

        // case:  stack underflow error on empty stack
        assert!(vm.data_stack.is_empty());
        vm.operation_stack.push(Operation::DUP);
        assert!(execute(&mut vm).is_err());

        // case:  duplicates the top stack entry
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

    #[test]
    fn operation_test__zero_comparisons() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack
        assert!(vm.data_stack.is_empty());
        vm.operation_stack = vec![Operation::ZERO_EQ];
        assert!(execute(&mut vm).is_err());

        assert!(vm.data_stack.is_empty());
        vm.operation_stack = vec![Operation::ZERO_GT];
        assert!(execute(&mut vm).is_err());

        assert!(vm.data_stack.is_empty());
        vm.operation_stack = vec![Operation::ZERO_LT];
        assert!(execute(&mut vm).is_err());

        assert!(vm.data_stack.is_empty());
        vm.operation_stack = vec![Operation::ZERO_NE];
        assert!(execute(&mut vm).is_err());

        // case:  refuses to cast a string -> int
        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
        ];

        vm.operation_stack = vec![Operation::ZERO_EQ];
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        vm.operation_stack = vec![Operation::ZERO_GT];
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        vm.operation_stack = vec![Operation::ZERO_LT];
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        vm.operation_stack = vec![Operation::ZERO_NE];
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        // case:  normal comparisons
        single_value_op_test_case!(vm, 1, Operation::ZERO_EQ, 0);
        single_value_op_test_case!(vm, 0, Operation::ZERO_EQ, 1);
        single_value_op_test_case!(vm, -1, Operation::ZERO_EQ, 0);

        single_value_op_test_case!(vm, 1, Operation::ZERO_GT, 1);
        single_value_op_test_case!(vm, 0, Operation::ZERO_GT, 0);
        single_value_op_test_case!(vm, -1, Operation::ZERO_GT, 0);

        single_value_op_test_case!(vm, 1, Operation::ZERO_LT, 0);
        single_value_op_test_case!(vm, 0, Operation::ZERO_LT, 0);
        single_value_op_test_case!(vm, -1, Operation::ZERO_LT, 1);

        single_value_op_test_case!(vm, 1, Operation::ZERO_NE, 1);
        single_value_op_test_case!(vm, 0, Operation::ZERO_NE, 0);
        single_value_op_test_case!(vm, -1, Operation::ZERO_NE, 1);

    }

}
