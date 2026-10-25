#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    MarketNotFound = 3,
    MarketClosed = 4,
    MarketAlreadyResolved = 5,
    MarketNotResolved = 6,
    InvalidOutcome = 7,
    InvalidAmount = 8,
    Unauthorized = 9,
    BetAlreadyClaimed = 10,
    NoWinningsToClaim = 11,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Outcome {
    Pending = 0,
    Yes = 1,
    No = 2,
    Invalid = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Market {
    pub id: u64,
    pub creator: Address,
    pub resolver: Address,
    pub title: String,
    pub description: String,
    pub deadline: u64,
    pub total_yes_amount: i128,
    pub total_no_amount: i128,
    pub outcome: Outcome,
    pub resolved_at: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Bet {
    pub user: Address,
    pub market_id: u64,
    pub outcome: Outcome,
    pub amount: i128,
    pub claimed: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    MarketCounter,
    Market(u64),
    UserBet(Address, u64),
}

const STORAGE_TTL_THRESHOLD: u32 = 17280; // ~1 day in ledgers
const STORAGE_TTL_EXTEND: u32 = 518400;   // ~30 days in ledgers

#[contract]
pub struct PredictionMarketContract;

#[contractimpl]
impl PredictionMarketContract {
    /// Initialize contract with admin address
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::MarketCounter, &0u64);
        env.storage().instance().extend_ttl(STORAGE_TTL_THRESHOLD, STORAGE_TTL_EXTEND);
        Ok(())
    }

    /// Create a new binary prediction market
    pub fn create_market(
        env: Env,
        creator: Address,
        resolver: Address,
        title: String,
        description: String,
        deadline: u64,
    ) -> Result<u64, Error> {
        creator.require_auth();

        if deadline <= env.ledger().timestamp() {
            return Err(Error::MarketClosed);
        }

        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::MarketCounter)
            .unwrap_or(0);
        counter += 1;

        let market = Market {
            id: counter,
            creator,
            resolver,
            title,
            description,
            deadline,
            total_yes_amount: 0,
            total_no_amount: 0,
            outcome: Outcome::Pending,
            resolved_at: 0,
        };

        env.storage().persistent().set(&DataKey::Market(counter), &market);
        env.storage().instance().set(&DataKey::MarketCounter, &counter);
        env.storage().instance().extend_ttl(STORAGE_TTL_THRESHOLD, STORAGE_TTL_EXTEND);

        env.events().publish((symbol_short!("mkt_new"), counter), market.deadline);

        Ok(counter)
    }

    /// Place a bet on an outcome (Yes = 1, No = 2)
    pub fn place_bet(
        env: Env,
        user: Address,
        market_id: u64,
        outcome_choice: u32,
        amount: i128,
    ) -> Result<(), Error> {
        user.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let chosen_outcome = match outcome_choice {
            1 => Outcome::Yes,
            2 => Outcome::No,
            _ => return Err(Error::InvalidOutcome),
        };

        let mut market: Market = env
            .storage()
            .persistent()
            .get(&DataKey::Market(market_id))
            .ok_or(Error::MarketNotFound)?;

        if env.ledger().timestamp() >= market.deadline {
            return Err(Error::MarketClosed);
        }

        if market.outcome != Outcome::Pending {
            return Err(Error::MarketAlreadyResolved);
        }

        // Record bet
        let bet_key = DataKey::UserBet(user.clone(), market_id);
        let mut user_bet = env
            .storage()
            .persistent()
            .get(&bet_key)
            .unwrap_or(Bet {
                user: user.clone(),
                market_id,
                outcome: chosen_outcome,
                amount: 0,
                claimed: false,
            });

        // Ensure user does not cross-bet in this simple parimutuel version
        if user_bet.amount > 0 && user_bet.outcome != chosen_outcome {
            return Err(Error::InvalidOutcome);
        }

        user_bet.outcome = chosen_outcome;
        user_bet.amount += amount;

        // Update market totals
        match chosen_outcome {
            Outcome::Yes => market.total_yes_amount += amount,
            Outcome::No => market.total_no_amount += amount,
            _ => return Err(Error::InvalidOutcome),
        }

        env.storage().persistent().set(&DataKey::Market(market_id), &market);
        env.storage().persistent().set(&bet_key, &user_bet);

        env.events().publish(
            (symbol_short!("bet_new"), market_id, user),
            (outcome_choice, amount),
        );

        Ok(())
    }

    /// Resolve a market (Resolver authorization required)
    pub fn resolve_market(
        env: Env,
        caller: Address,
        market_id: u64,
        winning_outcome: u32,
    ) -> Result<(), Error> {
        caller.require_auth();

        let mut market: Market = env
            .storage()
            .persistent()
            .get(&DataKey::Market(market_id))
            .ok_or(Error::MarketNotFound)?;

        if caller != market.resolver {
            let admin: Address = env
                .storage()
                .instance()
                .get(&DataKey::Admin)
                .ok_or(Error::NotInitialized)?;
            if caller != admin {
                return Err(Error::Unauthorized);
            }
        }

        if market.outcome != Outcome::Pending {
            return Err(Error::MarketAlreadyResolved);
        }

        market.outcome = match winning_outcome {
            1 => Outcome::Yes,
            2 => Outcome::No,
            3 => Outcome::Invalid,
            _ => return Err(Error::InvalidOutcome),
        };
        market.resolved_at = env.ledger().timestamp();

        env.storage().persistent().set(&DataKey::Market(market_id), &market);

        env.events().publish((symbol_short!("mkt_res"), market_id), winning_outcome);

        Ok(())
    }

    /// Claim winnings for a resolved market
    pub fn claim_winnings(env: Env, user: Address, market_id: u64) -> Result<i128, Error> {
        user.require_auth();

        let market: Market = env
            .storage()
            .persistent()
            .get(&DataKey::Market(market_id))
            .ok_or(Error::MarketNotFound)?;

        if market.outcome == Outcome::Pending {
            return Err(Error::MarketNotResolved);
        }

        let bet_key = DataKey::UserBet(user.clone(), market_id);
        let mut bet: Bet = env
            .storage()
            .persistent()
            .get(&bet_key)
            .ok_or(Error::NoWinningsToClaim)?;

        if bet.claimed {
            return Err(Error::BetAlreadyClaimed);
        }

        let total_pool = market.total_yes_amount + market.total_no_amount;
        let mut payout: i128 = 0;

        if market.outcome == Outcome::Invalid {
            // Refund initial bet
            payout = bet.amount;
        } else if bet.outcome == market.outcome {
            let winning_pool = match market.outcome {
                Outcome::Yes => market.total_yes_amount,
                Outcome::No => market.total_no_amount,
                _ => 0,
            };

            if winning_pool > 0 {
                // Parimutuel share: user_share = (user_bet * total_pool) / winning_pool
                payout = (bet.amount * total_pool) / winning_pool;
            }
        } else {
            return Err(Error::NoWinningsToClaim);
        }

        if payout <= 0 {
            return Err(Error::NoWinningsToClaim);
        }

        bet.claimed = true;
        env.storage().persistent().set(&bet_key, &bet);

        env.events().publish((symbol_short!("mkt_clm"), market_id, user), payout);

        Ok(payout)
    }

    /// Read market details
    pub fn get_market(env: Env, market_id: u64) -> Result<Market, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Market(market_id))
            .ok_or(Error::MarketNotFound)
    }

    /// Read user bet details
    pub fn get_bet(env: Env, user: Address, market_id: u64) -> Option<Bet> {
        env.storage().persistent().get(&DataKey::UserBet(user, market_id))
    }

    /// Read market counter
    pub fn get_market_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::MarketCounter).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

