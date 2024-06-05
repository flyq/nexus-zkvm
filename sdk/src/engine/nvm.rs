use super::*;
use super::super::error::NexusError;

pub mod customize {
    use super::*;

    #[derive(Clone)]
    pub struct NVMEngine<Mem>
    where
        Mem: Memory
    {
        vm: Option<NexusVM<Self::Memory>>,
        prover: Option<impl Prover>,

        k: usize,

        _proving: bool,
        _debug_execution: bool,
        _started: bool,
        _halted: bool,
        _proven: bool,
        _verified: bool,
        _private_input: bool,
    }

    impl<Mem> Engine for NVMEngine<Mem>
    where
        Mem: Memory
    {
        type Memory = Mem;

        fn load(
            &mut self,
            path: &PathBuf
        ) -> Result<Self, NexusError> {
            self.vm = load_elf::<Self::Memory>(path)?;
            self
        }

        fn from_config(
            &mut self,
            config: NexusVMConfig
        ) -> Result<Self, NexusError> {
            self.set_k(config.opts.k);

            if Some(name) = config.opts.machine {
                self.load_test_machine(name);
            }

            if Some(path) = config.opts.file {
                self.load(path);
            }

            self.set_defaulted_prover(config.prover);
            self
        };

        fn enable_prover_mode(
            &mut self,
        ) -> Result<Self, NexusError> {
            self.proving = true;
            self
        }

        fn enable_debug_execution_mode(
            &mut self,
        ) -> Result<Self, NexusError> {
            self.debug_execution = true;
            self
        }

        fn set_defaulted_prover(
            &mut self,
            prover: &ProverImpl
        ) -> Result<Self, NexusError> {
            self.prover = build_prover.get(prover);

            if self.prover.is_none() {
                // do stuff
            }

            self.check_compat()?;
            self
        };

        fn set_prover(
            &mut self,
            prover: &impl Prover
        ) -> Result<Self, NexusError> {
            self.prover = Some(prover);

            self.check_compat()?;
            self
        };

        fn set_k(&mut self, k: usize) -> Result<Self, NexusError> {
            self.k = k;
            self;
        }

        fn set_input<T: Serialize>(
            &self,
            input: T
        ) -> Result<Self, NexusError> {
            if vm.is_none() {
                // do stuff
            }

            // check if input already set

            let mut serialized = postcard::to_stdvec(input)?;

            vm.set_input(serialized.to_slice());

            self._private_input = true;
            self
        }

        fn set_input_bytes(
            &self,
            input: &[u8],
        ) -> Result<Self, NexusError> {
            if vm.is_none() {
                // do stuff
            }

            // check if input already set

            vm.set_input(input);

            self._private_input = true;
            self
        }

        fn set_public_input<T: Serialize>(
            &self,
            input: T
        ) -> Result<Self, NexusError> {
            unimplemented!()
        }

        fn set_public_input_bytes(
            &self,
            input: &[u8],
        ) -> Result<Self, NexusError> {
            unimplemented!()
        }

        fn generate(&self) -> Result<Self, NexusError> {
            self.prover.generate(self.k)?;
            self
        };

        fn execute(&self) -> Result<Self, NexusError> {
            if self.proving {
                return self.prove();
            }

            self.started = true;
            nvm::interactive::eval(&mut self.vm, self.debug_execution)?;
            self.halted = true;

            self
        };

        fn prove(&self) -> Result<Self, NexusError> {
            self.prover.prove()?;

            self.proven = true;
            self
        };

        fn verify(&self) -> Result<Self, NexusError> {
            self.prover.verify()?;

            self.verified = true;
            self
        };

        fn rewind(&self) -> Result<Self, NexusError> {

        };

        fn reset(&self) -> Result<Self, NexusError> {

        };

        fn proving(&self) -> bool {
            self._proving
        }

        fn debugging_execution(&self) -> bool {
            self._debug_execution
        }

        fn started(&self) -> bool {
            self._started
        }

        fn halted(&self) -> bool {
            self._halted
        }

        fn proven(&self) -> bool {
            self._proven
        }

        fn verified(&self) -> bool {
            self._verified
        }

    }

    impl<Mem> Default for NVMEngine<Mem>
    where
        Mem: Memory
    {
        fn default() -> Self {
            NVMEngine {
                vm: None,
                config: None,
                prover: Some(NovaProver::default()),
                k: 1,
                _proving: false,
                _debug_execution: false,
                _started: false,
                _halted: false,
                _proven: false,
                _verified: false,
                _private_input: false,
            }
        }
    }

    impl<Mem> NVMEngine<Mem>
    where
        Mem: Memory
    {
        fn new() -> Self {
            NVMEngine {


            }
        }
    }
}

#[derive(Clone)]
pub struct NVMEngine {
    vm: Option<NexusVM<Self::Memory>>,
    prover: Option<impl Prover>,

    k: usize,

    _proving: bool,
    _debug_execution: bool,
    _started: bool,
    _halted: bool,
    _proven: bool,
    _verified: bool,
    _private_input: bool,
}

impl Engine for NVMEngine {
    type Memory = MerkleTrie;

    fn load(
        &mut self,
        path: &PathBuf
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::load(&mut self, path)
    }

    fn from_config(
        &mut self,
        config: NexusVMConfig
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::load(&mut self, config)
    }

    fn enable_prover_mode(
        &mut self,
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::enable_prover_mode(&mut self)
    }

    fn enable_debug_execution_mode(
        &mut self,
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::enable_debug_execution_mode(&mut self)
    }

    fn set_defaulted_prover(
        &mut self,
        prover: &ProverImpl
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_defaulted_prover(&mut self, prover)
    }

    fn set_prover(
        &mut self,
        prover: &impl Prover
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_prover(&mut self, prover)
    }

    fn set_k(
        &mut self,
        k: usize
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_k(&mut self, k)
    }

    fn set_input<T: Serialize>(
        &self,
        input: T
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_input::<T>(&self, input)
    }

    fn set_input_bytes(
        &self,
        input: &[u8],
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_input_bytes(&self, input)
    }

    fn set_public_input<T: Serialize>(
        &self,
        input: T
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_public_input::<T>(&self, input)
    }

    fn set_public_input_bytes<T: Serialize>(
        &self,
        input: &[u8],
    ) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::set_public_input_bytes(&self, input)
    }

    fn generate(&self) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::generate(&self)
    };

    fn execute(&self) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::execute(&self)
    };

    fn prove(&self) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::prove(&self)
    };

    fn verify(&self) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::verify(&self)
    };

    fn rewind(&self) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::rewind(&self)
    };

    fn reset(&self) -> Result<Self, NexusError> {
        customize::NVMEngine::<Self::Memory>::reset(&self)
    };

    fn proving(&self) -> bool {
        customize::NVMEngine::<Self::Memory>::proving(&self)
    }

    fn debugging_execution(&self) -> bool {
        customize::NVMEngine::<Self::Memory>::debugging_execution(&self)
    }

    fn started(&self) -> bool {
        customize::NVMEngine::<Self::Memory>::started(&self)
    }

    fn halted(&self) -> bool {
        customize::NVMEngine::<Self::Memory>::halted(&self)
    }

    fn proven(&self) -> bool {
        customize::NVMEngine::<Self::Memory>::proven(&self)
    }

    fn verified(&self) -> bool {
        customize::NVMEngine::<Self::Memory>::verified(&self)
    }
}

impl Default for NVMEngine {
    fn default() -> Self {
        customize::NVMEngine::<Self::Memory>::default()
    }
}

impl NVMEngine {
    fn new() -> Self {
        customize::NVMEngine::<Self::Memory>::new()
    }
}
