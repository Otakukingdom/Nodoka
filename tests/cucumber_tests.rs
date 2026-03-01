//! Cucumber e2e test runner
//!
//! Runs behavior-driven acceptance tests using cucumber-rust.
//! Execute with: cargo test --test cucumber_tests

// Allow unwrap, expect, and indexing in test code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::indexing_slicing)]

mod cucumber;

// Import World trait from cucumber crate
use ::cucumber::World;
// Import our World implementation from local module
use self::cucumber::NodokaWorld;

#[tokio::main]
async fn main() {
    NodokaWorld::cucumber()
        .run_and_exit("tests/cucumber/features")
        .await;
}
