//! Interpretation

use std::process;

use super::VM;
use super::VirtualMachineError;

use super::super::compiler::parser::Operation;

pub fn execute(vm: &mut VM) -> Result<(), VirtualMachineError> {
    while let Some(operation) = vm.operation_stack.pop() {
        match operation {
            Operation::NOP => {
                continue;
            },
            Operation::NOP_INC => (),
            Operation::BYE => {
                println!("It's time to say goodbye~");
                process::exit(0);
            },
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
        vm.operation_stack.push(Operation::NOP);
        vm.operation_stack.push(Operation::NOP);

        assert_eq!(
            execute(&mut vm).unwrap(),
            (),
        );

    }

}
