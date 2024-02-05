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

        let coupon = &mut ctx.accounts.NftCuoponMetadata;
        coupon.release_threshold = release_threshold;
        coupon.count = count;

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
        Ok(());


    // Purchase Coupon function (handles purchasing portions of the NFT)
    pub fn purchase_coupon(ctx: Context<PurchaseCoupon>) -> Result<()> {
    // Access accounts
    let purchase_count_account = &mut ctx.accounts.purchase_count;
    let mint_account = &mut ctx.accounts.mint;
    let token_program = ctx.accounts.token_program.clone();
    let payer = &mut ctx.accounts.payer;
    let vault = &mut ctx.accounts.vault;

    // Calculate purchase price based on coupon amount and portion size
    let portion_size = 1; // Adjust based on desired granularity
    let purchase_price = mint_account.price / mint_account.coupon_amount * portion_size;

    // Transfer tokens from payer to vault
    token_program.transfer(
        &ctx.accounts.token_program.key(),
        payer.key(),
        vault.key(),
        purchase_price,
    )?;
    // Increment purchase count
    purchase_count_account.purchase_count += portion_size;

    // Check for release threshold
    if purchase_count_account.purchase_count >= mint_account.release_threshold {
        release_coupons(ctx.accounts.mint.key())?;
    }
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

    #[account]
    pub struct purchase_count {

        purchase_count: u64,


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
    #[derive(Accounts)]
        pub struct PurchaseCoupon<'info> {
        
            pub payer: Signer<'info>
    
    
        }