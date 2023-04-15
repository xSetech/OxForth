//! Interpretation

use std::cmp;
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

/// Pop two numbers off the stack, cast to i64, and return them.
fn two_ints_from_stack<'vm>(vm: &'vm mut VM) -> Result<(i64, i64), VirtualMachineError> {
    let x1: Option<Data> = vm.data_stack.pop();
    let x2: Option<Data> = vm.data_stack.pop();
    if x1.is_none() || x2.is_none() {
        return Result::Err(
            VirtualMachineError {
                msg: String::from("stack underflow"),
            }
        );
    }
    let x1: Data = x1.unwrap();
    let x2: Data = x2.unwrap();
    if x1.data_type == DataType::STRING || x2.data_type == DataType::STRING {
        return Result::Err(
            VirtualMachineError {
                msg: String::from("refusing cast: string->number"),
            }
        );
    }
    let x1: i64 = x1.value.parse::<i64>().unwrap();
    let x2: i64 = x2.value.parse::<i64>().unwrap();
    return Result::Ok((x2, x1)); // x2 is lower on stack, so it goes first
}

pub fn execute(vm: &mut VM) -> Result<(), VirtualMachineError> {
    while let Some(operation) = vm.operations.pop_front() {
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
            Operation::ADD => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let n3: i64 = n1 + n2;
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::BYE => {
                println!("It's time to say goodbye~");
                process::exit(0);
            },
            Operation::CMP_EQ => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let flag: bool = n1 == n2;
                vm.data_stack.push(
                    Data {
                        value: (flag as i64).to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::CMP_GT => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let flag: bool = n1 > n2;
                vm.data_stack.push(
                    Data {
                        value: (flag as i64).to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::CMP_LT => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let flag: bool = n1 < n2;
                vm.data_stack.push(
                    Data {
                        value: (flag as i64).to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::CMP_NE => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let flag: bool = n1 != n2;
                vm.data_stack.push(
                    Data {
                        value: (flag as i64).to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::DIV => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                if n2 == 0 {
                    return Result::Err(
                        VirtualMachineError {
                            msg: String::from("divisor cannot be zero"),
                        }
                    );
                }
                let n3: i64 = n1 / n2; // no floating-point yet
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
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
            },
            Operation::MAX => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let n3: i64 = cmp::max(n1, n2);
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::MIN => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let n3: i64 = cmp::min(n1, n2);
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::MOD => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                if n2 == 0 {
                    return Result::Err(
                        VirtualMachineError {
                            msg: String::from("divisor cannot be zero"),
                        }
                    );
                }
                let n3: i64 = n1.rem_euclid(n2);
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::MUL => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let n3: i64 = n1 * n2;
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
            Operation::NEGATE => {
                let (n, d): (i64, &mut Data) = int_and_data_from_stack(vm)?;
                d.value = (n * -1).to_string();
            },
            Operation::SUB => {
                let (n1, n2): (i64, i64) = two_ints_from_stack(vm)?;
                let n3: i64 = n1 - n2;
                vm.data_stack.push(
                    Data {
                        value: n3.to_string(),
                        data_type: DataType::NUMBER,
                    }
                );
            },
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
            },

        }
        vm._ops_applied += 1;
    }
    return Result::Ok(());
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    use std::collections::VecDeque;

    use super::super::DataType;

    /// Helper macro to confirm operations that require data on the stack produce an
    /// error if the stack doesn't contain enough data.
    macro_rules! empty_stack_test_case {
        ($vm:expr, $operation:expr) => {{
            assert!($vm.data_stack.is_empty());
            $vm.operations.push_back($operation);
            assert!(execute(&mut $vm).is_err());
        }};
    }

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
            $vm.operations = VecDeque::from([$operation]);
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

    /// Helper macro to test operations that pop two values and push a single value.
    macro_rules! two_in_one_out_op_test_case {
        ($vm:expr, $value1:expr, $value2:expr, $operation:expr, $expected:expr) => {{
            $vm.data_stack = vec![
                Data {
                    value: String::from("bottom of stack - should be ignored"),
                    data_type: DataType::STRING,
                },
                Data {
                    value: String::from($value1.to_string()),
                    data_type: DataType::NUMBER,
                },
                Data {
                    value: String::from($value2.to_string()), // top of stack
                    data_type: DataType::NUMBER,
                },
            ];
            $vm.operations = VecDeque::from([$operation]);
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
        vm.operations.push_back(Operation::NOP);
        vm.operations.push_back(Operation::NOP);
        assert_eq!(
            execute(&mut vm).unwrap(),
            (),
        );
    }

    #[test]
    fn operation_test__single_value_ops() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack
        empty_stack_test_case!(vm, Operation::ABS);
        empty_stack_test_case!(vm, Operation::NEGATE);

        // case:  refuses to cast a string -> int
        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
        ];
        vm.operations = VecDeque::from([Operation::ABS]);
        assert!(execute(&mut vm).is_err());

        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
        ];
        vm.operations = VecDeque::from([Operation::NEGATE]);
        assert!(execute(&mut vm).is_err());

        single_value_op_test_case!(vm, 42, Operation::ABS, 42);
        single_value_op_test_case!(vm, -42, Operation::ABS, 42);

        single_value_op_test_case!(vm,  0, Operation::NEGATE,  0);
        single_value_op_test_case!(vm,  1, Operation::NEGATE, -1);
        single_value_op_test_case!(vm, -1, Operation::NEGATE,  1);

    }

    #[test]
    fn operation_test__arithmetic() {
        let mut vm: VM = VM::default();

        empty_stack_test_case!(vm, Operation::ADD);
        empty_stack_test_case!(vm, Operation::SUB);
        empty_stack_test_case!(vm, Operation::MUL);
        empty_stack_test_case!(vm, Operation::DIV);
        empty_stack_test_case!(vm, Operation::MOD);

        two_in_one_out_op_test_case!(vm,  0,  0, Operation::ADD,  0);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::ADD,  1);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::ADD,  1);
        two_in_one_out_op_test_case!(vm,  1,  1, Operation::ADD,  2);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::ADD, -1);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::ADD, -1);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::ADD, -2);

        two_in_one_out_op_test_case!(vm,  0,  0, Operation::SUB,  0);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::SUB, -1);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::SUB,  1);
        two_in_one_out_op_test_case!(vm,  1,  1, Operation::SUB,  0);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::SUB,  1);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::SUB, -1);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::SUB,  0);

        two_in_one_out_op_test_case!(vm,  0,  0, Operation::MUL,  0);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::MUL,  0);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::MUL,  0);
        two_in_one_out_op_test_case!(vm,  1,  1, Operation::MUL,  1);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::MUL,  0);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::MUL,  0);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::MUL,  1);
        two_in_one_out_op_test_case!(vm,  1, -1, Operation::MUL, -1);
        two_in_one_out_op_test_case!(vm, -1,  1, Operation::MUL, -1);

        two_in_one_out_op_test_case!(vm,  0,  1, Operation::DIV,  0);
        two_in_one_out_op_test_case!(vm,  1,  1, Operation::DIV,  1);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::DIV,  0);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::DIV,  1);
        two_in_one_out_op_test_case!(vm,  1, -1, Operation::DIV, -1);
        two_in_one_out_op_test_case!(vm, -1,  1, Operation::DIV, -1);

        two_in_one_out_op_test_case!(vm,  0,  1, Operation::MOD,  0);
        two_in_one_out_op_test_case!(vm,  1,  1, Operation::MOD,  0);
        two_in_one_out_op_test_case!(vm,  2,  1, Operation::MOD,  0);
        two_in_one_out_op_test_case!(vm,  1,  2, Operation::MOD,  1);
        two_in_one_out_op_test_case!(vm, -1, -2, Operation::MOD,  1);
        two_in_one_out_op_test_case!(vm,  1, -2, Operation::MOD,  1);
        two_in_one_out_op_test_case!(vm, -1,  2, Operation::MOD,  1);

    }

    #[test]
    fn operation_test__comparisons() {
        let mut vm: VM = VM::default();

        empty_stack_test_case!(vm, Operation::CMP_EQ);
        empty_stack_test_case!(vm, Operation::CMP_GT);
        empty_stack_test_case!(vm, Operation::CMP_LT);
        empty_stack_test_case!(vm, Operation::CMP_NE);
        empty_stack_test_case!(vm, Operation::MAX);
        empty_stack_test_case!(vm, Operation::MIN);

        two_in_one_out_op_test_case!(vm,  1,  1, Operation::CMP_EQ,  1);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::CMP_EQ,  0);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::CMP_EQ,  0);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::CMP_EQ,  1);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::CMP_EQ,  0);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::CMP_EQ,  0);

        two_in_one_out_op_test_case!(vm,  1,  1, Operation::CMP_GT,  0);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::CMP_GT,  1);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::CMP_GT,  0);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::CMP_GT,  0);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::CMP_GT,  1);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::CMP_GT,  0);

        two_in_one_out_op_test_case!(vm,  1,  1, Operation::CMP_LT,  0);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::CMP_LT,  0);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::CMP_LT,  1);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::CMP_LT,  0);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::CMP_LT,  0);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::CMP_LT,  1);

        two_in_one_out_op_test_case!(vm,  1,  1, Operation::CMP_NE,  0);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::CMP_NE,  1);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::CMP_NE,  1);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::CMP_NE,  0);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::CMP_NE,  1);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::CMP_NE,  1);

        two_in_one_out_op_test_case!(vm,  0,  0, Operation::MAX,  0);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::MAX,  1);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::MAX,  1);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::MAX, -1);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::MAX,  0);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::MAX,  0);

        two_in_one_out_op_test_case!(vm,  1,  1, Operation::MIN,  1);
        two_in_one_out_op_test_case!(vm,  1,  0, Operation::MIN,  0);
        two_in_one_out_op_test_case!(vm,  0,  1, Operation::MIN,  0);
        two_in_one_out_op_test_case!(vm, -1, -1, Operation::MIN, -1);
        two_in_one_out_op_test_case!(vm,  0, -1, Operation::MIN, -1);
        two_in_one_out_op_test_case!(vm, -1,  0, Operation::MIN, -1);

    }

    #[test]
    fn operation_test__drop() {
        let mut vm: VM = VM::default();

        // case:  stack underflow error on empty stack
        empty_stack_test_case!(vm, Operation::DROP);

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
        vm.operations = VecDeque::from([Operation::DROP]);
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
        empty_stack_test_case!(vm, Operation::DUP);

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
        vm.operations = VecDeque::from([Operation::DUP]);
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
        empty_stack_test_case!(vm, Operation::ZERO_EQ);
        empty_stack_test_case!(vm, Operation::ZERO_GT);
        empty_stack_test_case!(vm, Operation::ZERO_LT);
        empty_stack_test_case!(vm, Operation::ZERO_NE);

        // case:  refuses to cast a string -> int
        vm.data_stack = vec![
            Data {
                value: String::from("item1"),
                data_type: DataType::STRING,
            },
        ];

        vm.operations = VecDeque::from([Operation::ZERO_EQ]);
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        vm.operations = VecDeque::from([Operation::ZERO_GT]);
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        vm.operations = VecDeque::from([Operation::ZERO_LT]);
        assert!(execute(&mut vm).is_err());
        assert_eq!(vm.data_stack.len(), 1);

        vm.operations = VecDeque::from([Operation::ZERO_NE]);
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
