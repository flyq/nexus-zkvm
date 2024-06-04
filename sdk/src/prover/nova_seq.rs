use super::*;

use nexus_api::prover::types::CommitmentScheme;
use nexus_api::prover::types::StepCircuit;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_ec::short_weierstrass::{Projective, SWCurveConfig};

pub mod custom {
    use super::*;

    use nexus_api::prover::types::seq::{PublicParams as SeqPP, IVCProof, SC};

    pub struct NovaProver<G1, G2, C1, C2, RO>
    where
        G1: SWCurveConfig,
        G2: SWCurveConfig<BaseField = G1::ScalarField, ScalarField = G1::BaseField>,
        C1: CommitmentScheme<Projective<G1>>,
        C2: CommitmentScheme<Projective<G2>>,
        RO: CryptographicSponge + Sync,
        RO::Config: CanonicalSerialize + CanonicalDeserialize + Sync,
    {
        pp: SeqPP<G1, G2, C1, C2, RO, SC>,
        proof: IVCProof<G1, G2, C1, C2, RO, SC>,
    }

    impl<G1, G2, C1, C2, RO> Prover for NovaProver<G1, G2, C1, C2, RO>
    where
        G1: SWCurveConfig,
        G2: SWCurveConfig<BaseField = G1::ScalarField, ScalarField = G1::BaseField>,
        C1: CommitmentScheme<Projective<G1>>,
        C2: CommitmentScheme<Projective<G2>>,
        RO: CryptographicSponge + Sync,
        RO::Config: CanonicalSerialize + CanonicalDeserialize + Sync,
    {

        fn generate(&self, k: usize) -> Result<(), NexusError> {
            self.pp = nexus_api::prover::setup::gen_vm_pp(k, &())?;
            Ok(())
        }

        fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError> {
            self.proof = nexus_api::prover::prover::prove_seq(&self.pp, trace);
            Ok(())
        }

        fn verify(&self) -> Result<(), NexusError> {
            self.proof.verify(&self.pp, self.proof.step_num())?;
            Ok(())
        }
    }

    impl<G1, G2, C1, C2, RO> Default for NovaProver<G1, G2, C1, C2, RO>
    where
        G1: SWCurveConfig,
        G2: SWCurveConfig<BaseField = G1::ScalarField, ScalarField = G1::BaseField>,
        C1: CommitmentScheme<Projective<G1>>,
        C2: CommitmentScheme<Projective<G2>>,
        RO: CryptographicSponge + Sync,
        RO::Config: CanonicalSerialize + CanonicalDeserialize + Sync,
    {}

    impl<G1, G2, C1, C2, RO> NovaProver<G1, G2, C1, C2, RO>
    where
        G1: SWCurveConfig,
        G2: SWCurveConfig<BaseField = G1::ScalarField, ScalarField = G1::BaseField>,
        C1: CommitmentScheme<Projective<G1>>,
        C2: CommitmentScheme<Projective<G2>>,
        RO: CryptographicSponge + Sync,
        RO::Config: CanonicalSerialize + CanonicalDeserialize + Sync,
    {

        fn new() -> Self {

        }
    }

}

use nexus_api::prover::types::{SeqPP, IVCProof};

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

    fn generate(&self, k: usize) -> Result<(), NexusError> {
        custom::NovaProver::<Self::G1, Self::G2, Self::C1, Self::C2, Self::RO>::generate(&self, k)
    }

    fn prove(&self, trace: Trace<Self::MemoryProof>) -> Result<(), NexusError> {
        custom::NovaProver::<Self::G1, Self::G2, Self::C1, Self::C2, Self::RO>::prove(&self, trace)
    }

    fn verify(&self) -> Result<(), NexusError> {
        custom::NovaProver::<Self::G1, Self::G2, Self::C1, Self::C2, Self::RO>::verify(&self)
    }

}

impl Default for NovaProver {}

impl NovaProver {

    fn new() -> Self {

    }
}
