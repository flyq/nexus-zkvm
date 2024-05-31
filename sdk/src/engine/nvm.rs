struct NVMEngine {
    type Memory = MerkleTrie;

    vm: Option<NexusVM<Self::Memory>>,

    prover: Option<impl Prover>,

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

}

impl Default for NVMEngine {}

impl NVMEngine {

    fn new() -> Self {

    }

}
