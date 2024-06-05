use super::*;

use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_ec::short_weierstrass::{Projective, SWCurveConfig};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use nexus_api::prover::types::CommitmentScheme;
use nexus_api::prover::types::StepCircuit;

pub mod custom {
    use super::*;

    use nexus_api::prover::types::{
        seq::{IVCProof, PublicParams as SeqPP},
        SC,
    };

    #[derive(Default, Clone)]
    pub struct NovaProver<G1, G2, C1, C2, RO, Mem>
    where
        G1: SWCurveConfig,
        G2: SWCurveConfig<BaseField = G1::ScalarField, ScalarField = G1::BaseField>,
        C1: CommitmentScheme<Projective<G1>>,
        C2: CommitmentScheme<Projective<G2>>,
        RO: CryptographicSponge + Sync,
        RO::Config: CanonicalSerialize + CanonicalDeserialize + Sync,
        Mem: MemoryProof,
    {
        pp: SeqPP<G1, G2, C1, C2, RO, SC>,
        proof: IVCProof<G1, G2, C1, C2, RO, SC>,
    }

    impl<G1, G2, C1, C2, RO, Mem> Prover for NovaProver<G1, G2, C1, C2, RO, Mem>
    where
        G1: SWCurveConfig,
        G2: SWCurveConfig<BaseField = G1::ScalarField, ScalarField = G1::BaseField>,
        C1: CommitmentScheme<Projective<G1>>,
        C2: CommitmentScheme<Projective<G2>>,
        RO: CryptographicSponge + Sync,
        RO::Config: CanonicalSerialize + CanonicalDeserialize + Sync,
        Mem: MemoryProof,
    {
        type MemoryProof = Mem;

        fn generate(&self, k: usize) -> Result<(), NexusError> {
            self.pp = nexus_api::prover::pp::gen_vm_pp(k, &())?;
            Ok(())
        }

        fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError> {
            self.proof = nexus_api::prover::prove_seq(&self.pp, trace);
            Ok(())
        }

        fn verify(&self) -> Result<(), NexusError> {
            self.proof.verify(&self.pp, self.proof.step_num())?;
            Ok(())
        }
    }
}

use nexus_api::prover::types::{IVCProof, SeqPP};

#[derive(Default, Clone)]
pub struct NovaProver {
    pp: SeqPP,
    proof: IVCProof,
}

// todo: replace with inherent associated types (https://github.com/rust-lang/rust/issues/8995)
trait NovaTypes {
    type G1: SWCurveConfig;
    type G2: SWCurveConfig;
    type C1: CommitmentScheme<Projective<Self::G1>>;
    type C2: CommitmentScheme<Projective<Self::G2>>;
    type RO: CryptographicSponge + Sync;
}

impl NovaTypes for NovaProver {
    type G1 = nexus_api::prover::types::G1; // ark_bn254::g1::Config
    type G2 = nexus_api::prover::types::G2; // ark_grumpkin::GrumpkinConfig
    type C1 = nexus_api::prover::types::C1; // PedersenCommitment<ark_bn254::G1Projective>
    type C2 = nexus_api::prover::types::C2; // PedersenCommitment<ark_grumpkin::Projective>
    type RO = nexus_api::prover::types::RO; // PoseidonSponge<ark_bn254::Fr>
}

impl Prover for NovaProver {
    type MemoryProof = Path;

    fn generate(&self, k: usize) -> Result<(), NexusError> {
        custom::NovaProver::<Self::G1, Self::G2, Self::C1, Self::C2, Self::RO, Self::MemoryProof>::generate(&self, k)
    }

    fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError> {
        custom::NovaProver::<Self::G1, Self::G2, Self::C1, Self::C2, Self::RO, Self::MemoryProof>::prove(&self, trace)
    }

    fn verify(&self) -> Result<(), NexusError> {
        custom::NovaProver::<Self::G1, Self::G2, Self::C1, Self::C2, Self::RO, Self::MemoryProof>::verify(&self)
    }
}
