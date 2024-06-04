use super::*;

struct NovaProver {
    pp: PublicParams,
    proof: IVCProof,
}

impl Prover for NovaProver {

    fn generate(&self, k: usize) -> Result<(), Error> {
        self.pp = prover::setup::gen_vm_pp(k, &())?;
        Ok(())
    }

    fn prove(&self, trace: Trace) -> Result<(), Error> {
        self.proof = prover::prover::prove_seq(&self.pp, trace);
        Ok(())
    }

    fn verify(&self) -> Result<(), Error> {
        self.proof.verify(&self.pp, proof.step_num())?;
        Ok(())
    }

}

impl Default for NovaProver {}

impl NovaProver {

    fn new() -> Self {

    }

}
