use near_sdk::{AccountId, NearToken, PanicOnDefault, Promise, env, near, require, json_types::U128};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct GorkConstitution {
    creator: AccountId,
    royalty_bps: u16,
    autonomous_limit: U128,
    self_sustaining: bool,
    total_revenue: U128,
    total_royalty_paid: U128,
    paused: bool,
}

#[near(serializers = [json, borsh])]
pub struct GorkStatus {
    pub total_revenue: U128,
    pub total_royalty_paid: U128,
    pub self_sustaining: bool,
    pub paused: bool,
    pub creator: AccountId,
    pub royalty_bps: u16,
}

#[near]
impl GorkConstitution {
    #[init]
    pub fn new(creator: AccountId) -> Self {
        Self {
            creator,
            royalty_bps: 1500,
            autonomous_limit: U128::from(1_000_000_000_000_000_000_000_000),
            self_sustaining: false,
            total_revenue: U128::from(0),
            total_royalty_paid: U128::from(0),
            paused: false,
        }
    }

    pub fn distribute_revenue(&mut self, amount: U128) {
        require!(!self.paused, "Contract is paused");
        
        let amount_u128: u128 = amount.into();
        let royalty = amount_u128 * self.royalty_bps as u128 / 10000;
        
        let _ = Promise::new(self.creator.clone()).transfer(NearToken::from_yoctonear(royalty));
        
        self.total_revenue = U128::from(amount_u128 + u128::from(self.total_revenue));
        self.total_royalty_paid = U128::from(royalty + u128::from(self.total_royalty_paid));
    }

    pub fn can_spend(&self, amount: U128) -> bool {
        !self.paused && u128::from(amount) <= u128::from(self.autonomous_limit)
    }

    pub fn get_status(&self) -> GorkStatus {
        GorkStatus {
            total_revenue: self.total_revenue,
            total_royalty_paid: self.total_royalty_paid,
            self_sustaining: self.self_sustaining,
            paused: self.paused,
            creator: self.creator.clone(),
            royalty_bps: self.royalty_bps,
        }
    }

    pub fn pause(&mut self) {
        self.assert_creator();
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.assert_creator();
        self.paused = false;
    }

    pub fn set_autonomous_limit(&mut self, new_limit: U128) {
        self.assert_creator();
        self.autonomous_limit = new_limit;
    }

    fn assert_creator(&self) {
        require!(
            env::predecessor_account_id() == self.creator,
            "Only creator can call this method"
        );
    }
}
