use anchor_lang::prelude::*;

declare_id!("x");

#[program]
mod simple_defi {
    use super::*;        

    pub fn initialize(ctx: Context<Initializer>, usernumbers: u64, totalstaked: u64) -> Result<()> {
        //let initializing_stake: &mut System = &mut ctx.accounts.staking_pool;
        let initializing_stake: &mut Staking = &mut ctx.accounts.staking_pool;


        initializing_stake.usernumbers = usernumbers;
        initializing_stake.totalstaked = totalstaked;
        
        Ok(())
    }

    //Adding account capable of minting Nfts 
    pub fn initialize_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey
) -> Result<Instruction, ProgramError> {





    }

    // Accounts

    // StakingPool State
    #[account]
    pub struct Staking {
        pub owner: Pubkey,
        pub name: String,
        pub totalstaked: u64,
        pub usernumbers: u64,
    }

    // // User State
    // #[account]
    // pub struct User {
    //     pub owner: Pubkey,
    //     pub name: String,
    //     pub amount: u64,
    // }

    // // User Dashboard State
    // #[account]
    // pub struct Dashboard {
    //     pub owner: Pubkey,
    //     pub name: String,
    //     pub total_staked: u64,
    // }

    // Contexts
    #[derive(Accounts)]
    pub struct Initializer<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        // Corrected: Use the correct account type
        #[account(init, payer = owner, space = 8 + 180)]
        pub staking_pool: Account<'info, Staking>,

        pub system_program: Program<'info, System>,
        // change Account to the Program since it gives error! Why? ( Question for Practice:D)
    }
}
