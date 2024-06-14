use super::error::NexusError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use nexus_api::{
    config::vm::ProverImpl,
    nvm::{interactive::load_elf, memory::Memory, NexusVM, NexusVMError},
};

use super::prover::Prover;

pub use nexus_api::config::vm::VmConfig as NexusVMConfig;

pub trait Engine: Default + Clone {
    /// The memory model to be used by the VM.
    type Memory: Memory;

    // setup

    /// Load the ELF file at `path` into the VM.
    fn load(&mut self, path: &PathBuf) -> Result<Self, NexusError>;

    /// Load the indicated test machine into the VM.
    fn load_test_machine(&mut self, machine: &String) -> Result<Self, NexusError>;

    // configuration

    /// Load VM configuration from a `NexusVMConfig` object.
    fn from_config(&mut self, config: NexusVMConfig) -> Result<Self, NexusError>;

    /// Print debugging trace of the VM execution.
    fn enable_debug_execution_mode(&mut self) -> Result<Self, NexusError>;

    /// Print debugging trace of the VM execution.
    fn enable_prover_mode(&mut self) -> Result<Self, NexusError>;

    /// Set choice of prover, using its defaults.
    fn set_defaulted_prover(&mut self, prover: &ProverImpl) -> Result<Self, NexusError>;

    /// Set choice of prover.
    fn set_prover(&mut self, prover: &impl Prover) -> Result<Self, NexusError>;

    /// Set `k` parameter that captures how many VM cycles should be proven per step.
    fn set_k(&mut self, k: usize) -> Result<Self, NexusError>;

    // input

    /// Serialize the input of type `T` for reading as private input in the VM.
    fn set_input<T: Serialize>(&self, input: T) -> Result<Self, NexusError>;

    /// Serialize the input byte slice for reading as private input in the VM.
    fn set_input_bytes(&self, input: &[u8]) -> Result<Self, NexusError>;

    /// Serialize the input of type `T` for reading as public input in the VM.
    fn set_public_input<T: Serialize>(&self, input: T) -> Result<Self, NexusError>;

    /// Serialize the input byte slice for reading as public input in the VM.
    fn set_public_input_bytes(&self, input: &[u8]) -> Result<Self, NexusError>;

    // running

    /// Generate public parameters for proving the VM execution.
    fn generate(&self) -> Result<Self, NexusError>;

    /// Execute the VM.
    fn execute(&self) -> Result<Self, NexusError>;

    /// Execute the VM and generate a proof of its execution.
    fn prove(&self) -> Result<Self, NexusError>;

    /// Verify a proven VM execution.
    fn verify(&self) -> Result<Self, NexusError>;

    /// Rewind the VM, without clearing the loaded program or inputs.
    fn rewind(&self) -> Result<Self, NexusError>;

    /// Reset the VM, including clearing the loaded program and inputs.
    fn reset(&self) -> Result<Self, NexusError>;

    // status

    /// Indicates whether configured to generate a proof when executing.
    fn proving(&self) -> bool;

    /// Indicates whether configured to print debug output for program execution.
    fn debugging_execution(&self) -> bool;

    /// Indicates whether program execution has started.
    fn started(&self) -> bool;

    /// Indicates whether program execution has halted.
    fn halted(&self) -> bool;

    /// Indicates whether program execution has been proven.
    fn proven(&self) -> bool;

    /// Indicates whether proof of program execution has been verified.
    fn verified(&self) -> bool;
}
