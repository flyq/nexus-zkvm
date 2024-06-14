use super::*;

use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_ec::short_weierstrass::{Projective, SWCurveConfig};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use nexus_api::prover::types::CommitmentScheme;
use nexus_api::prover::types::StepCircuit;
use nexus_api::nvm::memory::Path;

use nexus_api::prover::types::{IVCProof, SeqPP};

#[derive(Default, Clone)]
pub struct NovaProver {
    pp: Option<SeqPP>,
    proof: Option<IVCProof>,
}

// todo: replace with inherent associated types (https://github.com/rust-lang/rust/issues/8995)
trait NovaTypes {
    type G1: SWCurveConfig;
    type G2: SWCurveConfig;
    type C1: CommitmentScheme<Projective<Self::G1>>;
    type C2: CommitmentScheme<Projective<Self::G2>>;
    type RO: CryptographicSponge + Sync + Send;
}

impl NovaTypes for NovaProver {
    type G1 = nexus_api::prover::nova::types::G1; // ark_bn254::g1::Config
    type G2 = nexus_api::prover::nova::types::G2; // ark_grumpkin::GrumpkinConfig
    type C1 = nexus_api::prover::nova::types::C1; // PedersenCommitment<ark_bn254::G1Projective>
    type C2 = nexus_api::prover::nova::types::C2; // PedersenCommitment<ark_grumpkin::Projective>
    type RO = nexus_api::prover::nova::types::RO; // PoseidonSponge<ark_bn254::Fr>
}

impl Prover for NovaProver {
    type Arch = ProverArch::Local;
    type MemoryProof = Path;

    fn generate(&self, k: usize) -> Result<(), NexusError> {
        self.pp = nexus_api::prover::pp::gen_vm_pp(k, &())?;
        Ok(())
    }

    fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError> {
        self.proof = nexus_api::prover::nova::prove_seq(&self.pp, trace);
        Ok(())
    }

    fn verify(&self) -> Result<(), NexusError> {
        self.proof.verify(&self.pp, self.proof.step_num())?;
        Ok(())
    }
}
