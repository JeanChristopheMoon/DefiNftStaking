#[program]
pub mod simple_defi {
    use super::*;

pub fn initializing(ctx: Context<initilize>,usernumbers: u64,totalStaked: u64){
pub initilizngstake: &mut StakingPool = &mut ctx.accounts.staking_pool;

staking_pool.usernumbers = usernumbers;
staking_pool.totalStaked = totalStaked;


}
