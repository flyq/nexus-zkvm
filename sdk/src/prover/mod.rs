pub trait Prover: Default + Clone {

    /// Generate public parameters for proving.
    fn generate(&self, k: usize) -> Result<(), Error>;

    /// Prove a VM trace.
    fn prove(&self, trace: Trace) -> Result<(), Error>;

    /// Verify a proven VM execution.
    fn verify(&self) -> Result<(), Error>;

}
