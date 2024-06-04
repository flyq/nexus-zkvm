// An example of loading and running the NVM.

use nexus_api::config::vm::NovaImpl;
use nexus_api::{
    config::vm::{ProverImpl, VmConfig},
    nvm::{self, memory::MerkleTrie, NexusVM, VMOpts},
    prover::{self},
};
use std::path::PathBuf;

const CONFIG: VmConfig = VmConfig {
    prover: ProverImpl::Nova(NovaImpl::Sequential),
    opts: VMOpts {
        k: 1,
        machine: None,
        file: None,
    },
};

fn main() {
    // expects example programs (`nexus-zkvm/examples`) to have been built with `cargo build -r`
    let pb = PathBuf::from(r"../target/riscv32i-unknown-none-elf/release/private_input");

    // nb: the tracing and proving infrastructure assumes use of MerkleTrie memory model

    println!("Setting up public parameters...");
    let public_params =
        prover::setup::gen_vm_pp(CONFIG.opts.k, &()).expect("error generating public parameters");

    println!("Reading and translating vm...");
    let mut vm: NexusVM<MerkleTrie> =
        nvm::interactive::load_elf(&pb).expect("error loading and parsing RISC-V instruction");

    vm.syscalls.set_input(&[0x06]);

    println!("Generating execution trace of vm...");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    let trace = nvm::interactive::trace(
        &mut vm,
        CONFIG.k,
        matches!(CONFIG.prover, ProverImpl::Nova(NovaImpl::Parallel)),
    )
    .expect("error generating execution trace");
    println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");

    println!("Proving execution of length {}...", trace.blocks.len());
    let proof = prover::prove::prove_seq(&public_params, trace).expect("error proving execution");

    print!("Verifying execution...");
    proof
        .verify(&public_params, proof.step_num() as _)
        .expect("error verifying execution");

    println!("  Succeeded!");
}
