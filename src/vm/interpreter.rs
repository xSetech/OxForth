//! Interpretation

use super::VM;
use super::VirtualMachineError;

use super::super::compiler::parser::Operation;

pub fn execute(operations: &[Operation], vm: &mut VM) -> Result<(), VirtualMachineError> {
    for operation in operations.iter() {
        match operation {
            Operation::NOP => (),
        }
        vm._ops_applied += 1;
    }
    return Result::Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_interpret_test() {

        let mut vm: VM = VM::default();

        let operations: [Operation; 2] = [
            Operation::NOP,
            Operation::NOP,
        ];

        assert_eq!(
            execute(&operations, &mut vm).unwrap(),
            (),
        );

    }

}
