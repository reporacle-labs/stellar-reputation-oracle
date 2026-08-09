#![allow(dead_code)]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, String, Map, Symbol};

#[contracttype]
pub enum DataKey {
    Admin,
    Attesters,
    NextInteractionId,
    Interactions(Address),
    Interaction(u64),
    Scores(Address),
}

#[contracttype]
pub struct Interaction {
    pub id: u64,
    pub attester: Address,
    pub subject: Address,
    pub interaction_type: String,
    pub outcome: bool, // true = positive, false = negative
    pub weight: u64,
    pub timestamp: u64,
}

#[contracttype]
pub struct ReputationScore {
    pub score: u64,       // 0-100
    pub total_interactions: u64,
    pub positive: u64,
    pub negative: u64,
}

#[contract]
pub struct ReputationOracle;

#[contractimpl]
impl ReputationOracle {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().set(&DataKey::Admin, &admin);
        env.storage().set(&DataKey::Attesters, &Map::<Address, String>::new(&env));
        env.storage().set(&DataKey::NextInteractionId, &1u64);
    }

    pub fn register_attester(env: Env, attester: Address, protocol_name: String) {
        let admin: Address = env.storage().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut attesters: Map<Address, String> = env.storage().get(&DataKey::Attesters).unwrap();
        attesters.set(attester, protocol_name);
        env.storage().set(&DataKey::Attesters, &attesters);
    }

    pub fn submit_interaction(
        env: Env,
        subject: Address,
        interaction_type: String,
        outcome: bool,
        weight: u64,
    ) -> u64 {
        let caller = env.invoker();

        let attesters: Map<Address, String> = env.storage().get(&DataKey::Attesters).unwrap();
        if !attesters.contains_key(caller.clone()) {
            panic!("caller is not an authorized attester");
        }
        caller.require_auth();

        let mut next_id: u64 = env.storage().get(&DataKey::NextInteractionId).unwrap();
        let interaction = Interaction {
            id: next_id,
            attester: caller,
            subject: subject.clone(),
            interaction_type,
            outcome,
            weight,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().set(&DataKey::Interaction(next_id), &interaction);

        // Append to subject's interaction list
        let mut interactions: Vec<u64> = env.storage()
            .get(&DataKey::Interactions(subject.clone()))
            .unwrap_or(Vec::new(&env));
        interactions.push_back(next_id);
        env.storage().set(&DataKey::Interactions(subject.clone()), &interactions);

        // Recompute score
        Self::recompute_score(&env, &subject);

        next_id + 1
    }

    fn recompute_score(env: &Env, subject: &Address) {
        let interactions: Vec<u64> = env.storage()
            .get(&DataKey::Interactions(subject.clone()))
            .unwrap_or(Vec::new(env));

        let mut positive_weight: u64 = 0;
        let mut negative_weight: u64 = 0;
        let mut pos_count: u64 = 0;
        let mut neg_count: u64 = 0;

        for id in interactions.iter() {
            let interaction: Interaction = env.storage().get(&DataKey::Interaction(id)).unwrap();
            if interaction.outcome {
                positive_weight += interaction.weight;
                pos_count += 1;
            } else {
                negative_weight += interaction.weight;
                neg_count += 1;
            }
        }

        let total_weight = positive_weight + negative_weight;
        let score = if total_weight == 0 {
            50u64 // neutral default
        } else {
            (positive_weight * 100) / total_weight
        };

        let rep = ReputationScore {
            score,
            total_interactions: pos_count + neg_count,
            positive: pos_count,
            negative: neg_count,
        };
        env.storage().set(&DataKey::Scores(subject.clone()), &rep);
    }

    pub fn get_score(env: Env, subject: Address) -> ReputationScore {
        env.storage()
            .get(&DataKey::Scores(subject))
            .unwrap_or(ReputationScore {
                score: 50,
                total_interactions: 0,
                positive: 0,
                negative: 0,
            })
    }

    pub fn get_interactions(env: Env, subject: Address) -> Vec<Interaction> {
        let ids: Vec<u64> = env.storage()
            .get(&DataKey::Interactions(subject))
            .unwrap_or(Vec::new(&env));
        let mut result = Vec::new(&env);
        for id in ids.iter() {
            let interaction: Interaction = env.storage().get(&DataKey::Interaction(id)).unwrap();
            result.push_back(interaction);
        }
        result
    }

    pub fn get_attesters(env: Env) -> Map<Address, String> {
        env.storage().get(&DataKey::Attesters).unwrap()
    }
}
