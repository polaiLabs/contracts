#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Env, String};

#[test]
fn test_prediction_market_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);

    let contract_id = env.register(PredictionMarketContract, ());
    let client = PredictionMarketContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let creator = Address::generate(&env);
    let resolver = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    // Initialize
    client.init(&admin);

    // Create Market (deadline at 5000)
    let market_id = client.create_market(
        &creator,
        &resolver,
        &String::from_str(&env, "Will Soroban reach 50M transactions?"),
        &String::from_str(&env, "Prediction on Stellar network activity"),
        &5000,
    );
    assert_eq!(market_id, 1);

    // Place Bets
    // user1 bets 100 on Yes (1)
    client.place_bet(&user1, &market_id, &1, &100);
    // user2 bets 300 on No (2)
    client.place_bet(&user2, &market_id, &2, &300);

    let market = client.get_market(&market_id);
    assert_eq!(market.total_yes_amount, 100);
    assert_eq!(market.total_no_amount, 300);
    assert_eq!(market.outcome, Outcome::Pending);

    // Fast forward past deadline
    env.ledger().set_timestamp(6000);

    // Resolve Market in favor of YES (1)
    client.resolve_market(&resolver, &market_id, &1);

    let resolved_market = client.get_market(&market_id);
    assert_eq!(resolved_market.outcome, Outcome::Yes);

    // User1 claims winnings
    // Total pool is 400. User1 owned 100% of YES bets, so payout = 400
    let payout = client.claim_winnings(&user1, &market_id);
    assert_eq!(payout, 400);

    // User2 lost, claiming fails
    let err = client.try_claim_winnings(&user2, &market_id);
    assert!(err.is_err());
}
