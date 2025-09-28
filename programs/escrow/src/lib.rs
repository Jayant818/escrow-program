pub mod constants;
pub mod error;
pub mod handler;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use handler::*;
pub use state::*;

declare_id!("bKjQMDCtnywc7dGc3YQCx9DtgA97BS7qJpWKBq77Qdu");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(context:Context<MakeOffer>,id:u64,token_a_offered_amount:u64,token_b_wanted_amount:u64)->Result<()>{

        handler::make_offer(context, id, token_a_offered_amount, token_b_wanted_amount)?;

        Ok(())
    }
}

// #[cfg(test)]
// mod escrow_test_helper;

// #[cfg(test)]
// mod tests;
