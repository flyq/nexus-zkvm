use std::path::PathBuf;

use nexus_api::{
    nvm::{interactive::{load_elf},
          NexusVMError,
          NexusVM,
    },
};


pub trait Engine: Default + Clone {
    /// The memory model to be used by the VM.
    type Memory;

    // setup

    /// Load the ELF file at `path` into the VM.
    fn load(
        &mut self,
        path: &PathBuf
    ) -> Result<Self, Error> {
        self.vm = load_elf::<Self::Memory>(path)?;
        self
    }

    // configuration

    /// When executing the VM, generate a proof.
    fn enable_prover_mode(
        &mut self,
    ) -> Result<Self, Error> {
        self.proving = true;
        self
    }

    /// Print debugging trace of the VM execution. 
    fn enable_debug_execution_mode(
        &mut self,
    ) -> Result<Self, Error> {
        self.debug_execution = true;
        self
    }

    // input

    /// Serialize the input of type `T` for reading as private input in the VM.
    fn set_input<T: Serialize + Deserialize>(input: T) -> Result<Self, Error>;

    /// Serialize the input byte slice for reading as private input in the VM.
    fn set_input_bytes(input: &[u8]) -> Result<Self, Error>;

    /// Serialize the input of type `T` for reading as public input in the VM.
    fn set_public_input<T: Serialize + Deserialize>(input: T) -> Result<Self, Error>;

    /// Serialize the input byte slice for reading as public input in the VM.
    fn set_public_input_bytes(input: &[u8]) -> Result<Self, Error>;

    // running

    /// Generate public parameters for proving the VM execution.
    fn generate(&self) -> Result<Self, Error>;

    /// Execute the VM.
    fn execute(&self) -> Result<Self, Error>;

    /// Execute the VM and generate a proof of its execution.
    fn prove(&self) -> Result<Self, Error>;

    /// Verify a proven VM execution.
    fn verify(&self) -> Result<Self, Error>;

    /// Rewind the VM, without clearing the loaded program or inputs.
    fn rewind(&self) -> Result<Self, Error>;

    /// Reset the VM, including clearing the loaded program and inputs.
    fn reset(&self) -> Result<Self, Error>;

    // status
    
    /// Indicates whether configured to generate a proof when executing.
    fn proving(&self) -> bool {
        self.proving
    }

    /// Indicates whether configured to print debug output for program execution. 
    fn debugging_execution(&self) -> bool {
        self.debug_execution
    }

    /// Indicates whether program execution has started.
    fn started(&self) -> bool {
        self.started
    }

    /// Indicates whether program execution has halted.
    fn halted(&self) -> bool {
        self.halted
    }

    /// Indicates whether program execution has been proven.
    fn proven(&self) -> bool {
        self.proven
    }

    /// Indicates whether proof of program execution has been verified..
    fn verified(&self) -> bool {
        self.verified
    }

}

