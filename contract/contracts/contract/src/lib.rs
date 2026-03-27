#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    pub author: Address,
    pub rating: u32,
    pub comment: String,
}

#[contracttype]
pub enum DataKey {
    Reviews(Symbol),
}

#[contract]
pub struct ReviewSystem;

#[contractimpl]
impl ReviewSystem {
    /// Adds a review for a specific subject (e.g., a product or service).
    pub fn add_review(env: Env, subject: Symbol, author: Address, rating: u32, comment: String) {
        author.require_auth();

        if rating > 5 {
            panic!("Rating must be between 0 and 5");
        }

        let key = DataKey::Reviews(subject.clone());
        let mut reviews: Vec<Review> = env.storage().persistent().get(&key).unwrap_or(Vec::new(&env));

        reviews.push_back(Review {
            author,
            rating,
            comment,
        });

        env.storage().persistent().set(&key, &reviews);
    }

    /// Fetches all reviews for a specific subject.
    pub fn get_reviews(env: Env, subject: Symbol) -> Vec<Review> {
        let key = DataKey::Reviews(subject);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }
}