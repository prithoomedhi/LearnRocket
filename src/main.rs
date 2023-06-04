#![allow(warnings)]

use chrono::{DateTime, Utc};
use prime_checker::{check_if_anti_prime, check_if_prime, find_anti_primes_till, find_primes_till};
use rocket::{form::validate::contains, serde::json::*, *};

mod libs;

use crate::libs::structures::models::PrimeNumberResponse;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    let name = match clean_name(&name) {
        Ok(name) => name,
        Err(e) => panic!("{}", e),
    };

    let age = match clean_age(&age.to_string()) {
        Ok(age) => age,
        Err(e) => panic!("{}", e),
    };

    let datetime = match get_date_time() {
        Ok(datetime) => datetime,
        Err(e) => panic!("{}", e),
    };
    format!(
        "Hello, my name is {name} and I am {age} years old as of today ({datetime}).",
        age = age,
        name = name.to_uppercase(),
        datetime = datetime
    )
}

#[get("/find-prime/<val>")]
fn primes(val: u64) -> Json<PrimeNumberResponse> {
    let primes = find_primes_till(val);
    let response = PrimeNumberResponse::create(val, primes);

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![primes])
}

fn clean_name(name: &str) -> Result<String, String> {
    let name = match name.trim().parse::<String>() {
        Ok(mut val) => {
            if val.len() > 100 {
                val = val[0..100].to_string();
            }
            if val.contains("\"") {
                val = val.replace("\"", "");
            }
            val
        }
        Err(s) => return Err(s.to_string()),
    };
    return Ok(name);
}

fn clean_age(age: &str) -> Result<u8, String> {
    let age = match age.trim().parse::<u8>() {
        Ok(age) => {
            if age > 100 {
                100 as u8;
            }
            age
        }
        Err(s) => return Err(s.to_string()),
    };
    return Ok(age);
}

fn get_date_time() -> Result<String, String> {
    let current_datetime: DateTime<Utc> = Utc::now();

    // Format datetime as a string
    let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(formatted_datetime)
}
