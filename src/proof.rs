use risc0_zkp::{prove, verify, Digest};
use risc0_zkvm::{Executor, Program};
use crate::ERC721;
use crate::error::ERC721Error;

pub fn generate_proof(erc721: &ERC721, token_id: u64) -> Result<Digest, ERC721Error> {
    let owner = erc721.owner_of(token_id)?;
    let program = Program::from_slice(include_bytes!("path/to/zk_program.risc0")).unwrap();
    let input = vec![token_id as u32, owner.as_bytes().to_vec()];

    let executor = Executor::new(program).unwrap();
    let (proof, output) = executor.prove(input).unwrap();
    
    verify(proof, output).unwrap();
    Ok(output)
}