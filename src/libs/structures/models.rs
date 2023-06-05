use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HelloWorldResponse {
    msg: String,
}

impl HelloWorldResponse {
    pub fn create(msg: Option<String>) -> Self {
        let msg = match msg {
            Some(msg) => msg,
            None => "".to_string(),
        };
        Self { msg }
    }
}

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

#[derive(Deserialize, Serialize)]
pub struct UserForm {
    name: String,
    email: String,
}

impl UserForm {
    pub fn create(name: String, email: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        Ok(Self { name, email })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Deserialize, Serialize)]
pub struct Error {
    message: String,
    status: u16,
}

impl Error {
    pub fn create(message: String, status: Option<u16>) -> Self {
        let status = match status {
            Some(status) => status,
            None => 400,
        };
        Self { message, status }
    }
}
