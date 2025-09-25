pub use satellite_lang::prelude::*;
declare_id!("86ecdec70fd08e7e664d3f527a2fe07ae913cc9a6db5909bd5daf40978250282");

#[program]
pub mod test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let config = &mut ctx.accounts.config;
        config.version = 1;
        config.bump = ctx.bumps.config;

        emit!(ProgramInitialized {
            version: config.version,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"config"],
        bump,
        payer = signer,
        space = ProgramConfig::LEN
    )]
    pub config: Account<'info, ProgramConfig>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    pub version: u16,
    pub bump: u8,
}

impl ProgramConfig {
    pub const LEN: usize = 8 + ProgramConfig::INIT_SPACE;
}

#[event]
pub struct ProgramInitialized {
    pub version: u16,
}