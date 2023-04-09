//! Virtual Machine

pub mod interpreter;

pub struct VM {

    /// Number of operations applied to the VM instance
    pub _ops_applied: u64,

}

#[derive(Debug)]
pub struct VirtualMachineError {
    /// `msg` describes what went wrong
    pub msg: String
}
