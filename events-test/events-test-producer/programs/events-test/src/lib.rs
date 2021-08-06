use anchor_lang::prelude::*;
use serum_dex::matching::{OrderType as SerumOrderType, Side as SerumSide};


#[program]
pub mod events_test {
    use super::*;

    pub fn initialize<'info>(
        ctx: Context<'_, '_, '_, 'info, Initialize<'info>>,
        own_address: Pubkey,
    ) -> Result<()> {

        let stoploss = &mut ctx.accounts.stoploss_state;
        stoploss.own_address = own_address;

        emit!(StoplossOrderUpdate {
            own_address: own_address,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    stoploss_state: ProgramAccount<'info, StoplossState>,
    rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Debug)]
pub struct StoplossState {
    pub own_address: Pubkey,
}

#[event]
#[derive(Debug, Clone)]
pub struct StoplossOrderUpdate {
    pub own_address: Pubkey,
}


#[error]
pub enum ErrorCode {
    #[msg("Error")]
    SomeErr,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum OrdStatus {
    New = 0,
}
