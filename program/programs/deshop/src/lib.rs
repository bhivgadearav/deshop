use anchor_lang::prelude::*;
mod error;
mod events;
mod instructions;
mod state;
use instructions::{handle_checkout, handle_initialize, handle_initialize_seller};

declare_id!("22XZU4FA95TFfgEgmASYutYwubKZvqaz94YF6qqNkJ3s");

#[program]
pub mod deshop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }

    pub fn initialize_seller(ctx: Context<InitializeSeller>) -> Result<()> {
        handle_initialize_seller(ctx)
    }

    pub fn checkout(ctx: Context<Checkout>) -> Result<()> {
        handle_checkout(ctx)
    }
}
