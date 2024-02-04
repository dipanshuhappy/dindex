use anchor_lang::prelude::*;

declare_id!("EXgQRoZFHkpCoYeYmV4b2Mpx65uAsEqViYGQjzHhkUQs");

#[program]
pub mod dindex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
