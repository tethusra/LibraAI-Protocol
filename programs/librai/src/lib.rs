use anchor_lang::prelude::*;

declare_id!("Libra111111111111111111111111111111111111");

#[program]
pub mod librai {
    use super::*;

    pub fn initialize_agent(ctx: Context<InitializeAgent>, config: AgentConfig) -> Result<()> {
        msg!("🧠 Agent initialized.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAgent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AgentConfig {
    pub thinking_speed: u64,
    pub model: String,
}
