use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PrimeNumberResponse {
    arg: u64,
    primes: Vec<u64>,
}

impl PrimeNumberResponse {
    pub fn create(arg: u64, primes: Vec<u64>) -> Self {
        Self { arg, primes }
    }
}
