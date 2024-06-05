use nexus_api::config::vm::{NovaImpl, ProverImpl};
use nexus_api::nvm::{interactive::Trace, memory::MemoryProof};

use super::error::NexusError;

mod nova_seq;
use nova_seq::NovaProver;

pub trait Prover: Default + Clone {
    /// The memory proof form used in the trace.
    type MemoryProof: MemoryProof;

    /// Generate public parameters for proving.
    fn generate(&self, k: usize) -> Result<(), NexusError>;

    /// Prove a VM trace.
    fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError>;

    /// Verify a proven VM execution.
    fn verify(&self) -> Result<(), NexusError>;
}

fn build_prover(prover: &ProverImpl) -> Option<impl Prover> {
    let built = match prover {
        ProverImpl::Nova(NovaImpl::Sequential) => Some(NovaProver::default()),
        _ => None,
    };

    built
}
