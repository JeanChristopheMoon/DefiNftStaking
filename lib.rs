use anchor_lang::prelude::*;
// Need to reconsideration with more details
use solana_program::instruction::Instruction;
use spl_token::instruction as token_instruction;

declare_id!("x");


#[program]
mod simple_defi {
    use super::*;        

    

    //Adding "minting account" capable of minting Nfts 
    pub fn initialize_mint(
        token_program_id: &Pubkey,
        mint_pubkey: &Pubkey,
        mint_authority_pubkey: &Pubkey,
        freeze_authority_pubkey: Option<&Pubkey>,
        decimals: u8,
        max_amount: u64,
        count : u64,
        price: u64,
        description: String,
        release_threshold: u64

        ) -> Result<Instruction, ProgramError> {
            
        // Initialize the mint
        let instruction = initialize_mint(
            &token_program_id(&ctx).key(),
            &mint_pubkey(&ctx).key(),
            &mint_authority_pubkey(&ctx).key(),
            None, // Optional freeze authority
            decimals(&ctx),
        )?;
        invoke_signed(&instruction, &[
            mint_account(&ctx).to_account_info(),
            rent_sysvar::id(),
        ])?;
    }

    pub fn mint_to(
        token_program_id: &Pubkey,
        mint_pubkey: &Pubkey,
        account_pubkey: &Pubkey,
        owner_pubkey: &Pubkey,
        signer_pubkeys: &[&Pubkey],
        amount: u64
    ) -> Result<Instruction, ProgramError>{
        let instruction = mint_to(
            &token_program_id(&ctx).key(),
            &mint_pubkey(&ctx).key(),
            &account_pubkey(&ctx).key(),
            &owner_pubkey(&ctx).key(),
            signer_pubkeys,
            amount(&ctx),
        )?;
        invoke_signed(&instruction, &[
            mint_account(&ctx).to_account_info(),
            account_pubkey(&ctx).to_account_info(),
            owner_pubkey(&ctx).to_account_info(),
        ])?;
        Ok(())
    }
        // Optionally initialize the target account if needed
    pub fn initialize_account(
        token_program_id: &Pubkey,
        account_pubkey: &Pubkey,
        mint_pubkey: &Pubkey,
        owner_pubkey: &Pubkey
    ) -> Result<Instruction, ProgramError>{
        let instruction = initialize_account(
            &token_program_id(&ctx).key(),
            &account_pubkey(&ctx).key(),
            &mint_pubkey(&ctx).key(),
            &owner_pubkey(&ctx).key(),
        )?;
        invoke_signed(&instruction, &[
            account_pubkey(&ctx).to_account_info(),
            mint_account(&ctx).to_account_info(),
            rent_sysvar::id(),
        ])?;
        Ok(())


    pub fn release_coupons(nft: Pubkey, name: String ) -> Result<Instruction, ProgramError>{
        





        Ok(())
    }    
}   

// ... other instructions 








    // Accounts

    // StakingPool State
    #[account]
    pub struct Staking {
        pub owner: Pubkey,
        pub name: String,
        pub totalstaked: u64,
        pub usernumbers: u64,
    }
    #[account]
    pub struct NftCuoponMetadata {
        pub owner: Pubkey,
        pub restuarant_name: String,
        pub buyers: Vec<u64>,
        pub max_amount: u64,
        pub count: u64,
        pub price: u64,
        pub image: String,
        pub description: String,
        pub release_threshold: u64,
    

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
