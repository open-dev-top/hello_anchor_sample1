use anchor_lang::prelude::*;

pub mod instructions;

pub use instructions::*;

declare_id!("GgYy3QMGzhfgcgFEP2LRXTadPyrzj4YdqeCB8Zj2RrpW");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
