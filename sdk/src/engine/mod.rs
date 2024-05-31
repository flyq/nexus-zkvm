use std::path::PathBuf;

use nexus_api::{
    nvm::{interactive::{load_elf},
          NexusVMError,
          NexusVM,
    },
};


pub trait Engine: Default + Clone {
    type Memory;

    // setup

    fn load(
        &mut self,
        path: &PathBuf
    ) -> Result<Self, Error> {
        self.vm = load_elf::<Self::Memory>(path)?;
        self
    }

    // configuration

    fn enable_prover_mode(
        &mut self,
    ) -> Result<Self, Error> {
        self.proving = true;
        self
    }

    // input

    fn set_input<T: Serialize + Deserialize>(input: T) -> Result<Self, Error>;

    fn set_input_bytes(input: &[u8]) -> Result<Self, Error>;

    fn set_public_input<T: Serialize + Deserialize>(input: T) -> Result<Self, Error>;

    fn set_public_input_bytes(input: &[u8]) -> Result<Self, Error>;

    // running

    fn generate() -> Result<Self, Error>;

    fn execute() -> Result<Self, Error>;

    fn prove() -> Result<Self, Error>;

    fn rewind() -> Result<Self, Error>;

    fn reset() -> Result<Self, Error>;

    // status

    fn started(&self) -> bool {
        self.started
    }

    fn halted(&self) -> bool {
        self.halted
    }

    fn proven(&self) -> bool {
        self.proven
    }

    fn verified(&self) -> bool {
        self.verified
    }

}

