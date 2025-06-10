use anchor_lang::prelude::*;

declare_id!("GgYy3QMGzhfgcgFEP2LRXTadPyrzj4YdqeCB8Zj2RrpW");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
