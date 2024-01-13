//Accounts 

//StakingPool State
pub struct StakingPool {
  pub owner :  Pubkey,
  pub name: String,
  pub totalStaked: u64,
  pub usernumbers: u64,
}

//User State
pub struct user {
pub owner: Pubkey,
pub name: String,
pub amount: u64,
}

//User Dashboard State
pub struct Dashboard {
pub owner: Pubkey,
pub name: String,
pub total_staked: u64,
}
