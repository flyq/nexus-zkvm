use nexus_api::config::vm::{NovaImpl, ProverImpl};
use nexus_api::nvm::{interactive::Trace, memory::MemoryProof};

use super::error::NexusError;

mod nova_seq;
use nova_seq::NovaProver;

#[derive(Default)]
enum ProverArch {
    #[default]
    Local,
}

pub trait Prover: Default + Clone {
    /// The proving architecture used to prove the trace.
    type Arch: ProverArch;

    /// The memory proof form used in the trace.
    type MemoryProof: MemoryProof;

    /// Generate public parameters for proving.
    fn generate(&self, k: usize) -> Result<(), NexusError>;

    /// Prove a VM trace.
    fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError>;

    /// Verify a proven VM execution.
    fn verify(&self) -> Result<(), NexusError>;
}

fn build_prover(prover: &ProverImpl, arch: &ProverArch) -> Option<impl Prover> {
    let built = match (prover, arch) {
        (ProverImpl::Nova(NovaImpl::Sequential), ProverArch::Local) => Some(NovaProver::default()),
        _ => None,
    };

    built
}
