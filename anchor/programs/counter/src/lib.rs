#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod micropayments {
    use super::*;

    pub fn initialize_poll (_ctx: Context<initializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }

}


#[derive(Accounts)]
