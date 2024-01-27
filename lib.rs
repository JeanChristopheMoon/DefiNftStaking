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

    //Adding "minting account" capable of minting Nfts 
    pub fn initialize_mint(
        token_program_id: &Pubkey,
        mint_pubkey: &Pubkey,
        mint_authority_pubkey: &Pubkey,
        freeze_authority_pubkey: Option<&Pubkey>,
        decimals: u8,
    ) -> Result<Instruction, ProgramError> {
        check_program_account(token_program_id)?;
        let freeze_authority = freeze_authority_pubkey.cloned().into();
        //Creating instace of InitializeMint operation from TokenInstruction enum
        let data = TokenInstruction::InitializeMint {
            mint_authority: *mint_authority_pubkey,
            freeze_authority,
            decimals,
        }
        .pack();
    
        let accounts = vec![
            AccountMeta::new(*mint_pubkey, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ];
    
        Ok(Instruction {
            program_id: *token_program_id,
            accounts,
            data,
        })
    }

        /// Adding a "Token Account"
pub fn initialize_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(token_program_id)?;
    let data = TokenInstruction::InitializeAccount.pack();

    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

    /// Minting function after MintAccount and TokenAccount
pub fn mint_to(
    token_program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    check_program_account(token_program_id)?;
    let data = TokenInstruction::MintTo { amount }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*mint_pubkey, false));
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}




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
