use super::*;

struct NVMEngine {
    type Memory = impl Memory;

    vm: Option<NexusVM<Self::Memory>>,
    config: Option<VmConfig>,

    prover: Option<impl Prover>,

    k: usize,

    proving: bool,
    debug_execution: bool,

    started: bool,
    halted: bool,
    proven: bool,
    verified: bool,
}

impl Engine for NVMEngine {

    fn set_input<T: Serialize + Deserialize>(
        &self,
        input: T
    ) -> Result<Self, Error> {
        if vm.is_none() {
            // do stuff
        }

        // check if input already set

        let mut serialized = postcard::to_stdvec(input)?;

        vm.set_input(serialized.to_slice());
        self
    }

    fn set_input_bytes<T: Serialize + Deserialize>(
        &self,
        input: &[u8],
    ) -> Result<Self, Error> {
        if vm.is_none() {
            // do stuff
        }

        // check if input already set

        vm.set_input(input);
        self
    }

    fn set_public_input<T: Serialize + Deserialize>(
        &self,
        input: T
    ) -> Result<Self, Error> {
        unimplemented!()
    }

    fn set_public_input_bytes<T: Serialize + Deserialize>(
        &self,
        input: &[u8],
    ) -> Result<Self, Error> {
        unimplemented!()
    }

    fn generate(&self) -> Result<Self, Error> {
        self.prover.generate(self.k)?;
        self
    };

    fn execute(&self) -> Result<Self, Error> {
        if self.proving {
            return self.prove();
        }

        self.started = true;
        nvm::interactive::eval(&mut self.vm, self.debug_execution)?;
        self.halted = true;

        self
    };

    fn prove(&self) -> Result<Self, Error> {
        self.prover.prove()?;

        self.proven = true;
        self
    };

    fn verify(&self) -> Result<Self, Error> {
        self.prover.verify()?;

        self.verified = true;
        self
    };

    fn rewind(&self) -> Result<Self, Error> {

    };

    fn reset(&self) -> Result<Self, Error> {

    };

}

impl Default for NVMEngine {}

impl NVMEngine {

    fn new() -> Self {

    }

}
