#![allow(warnings)]

use chrono::{DateTime, Utc};
use prime_checker::{check_if_anti_prime, check_if_prime, find_anti_primes_till, find_primes_till};
use rocket::{form::validate::contains, serde::json::*, *};

mod libs;

use crate::libs::structures::models::{PrimeNumberResponse, UserForm, Error, HelloWorldResponse};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> Json<HelloWorldResponse> {
    //! This controller is used to demonstrate how to read path variables
    //! from a request and how to return a basic `json` response.
    //! 
    //! Basically, it formats a string with the name and age of the user as provided in the path variables
    //! and formats a string with the two parameters as well as the current date and time.
    //! 
    //! It sends a `json` HTTP response with the formatted string.
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
    let msg = format!(
        "Hello, my name is {name} and I am {age} years old as of today ({datetime}).",
        age = age,
        name = name.to_uppercase(),
        datetime = datetime
    );

    Json(HelloWorldResponse::create(Some(msg)))
}

#[get("/find-prime/<val>")]
fn primes(val: u64) -> Json<PrimeNumberResponse> {
    let primes: Vec<u64> = find_primes_till(val);
    let response: PrimeNumberResponse = PrimeNumberResponse::create(val, primes);

    Json(response)
}

#[post("/user", data = "<user_form>")]
fn user(user_form: Json<UserForm>) -> Result<Json<UserForm>, Json<Error>>  {
    let user_form = match UserForm::create(user_form.name().to_string(), user_form.email().to_string()){
        Ok(val) => val,
        Err(e) => {
            let error = Error::create(e, None);
            return Err(Json(error));
        },
    };
    Ok(Json(user_form))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![primes])
        .mount("/", routes![user])
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
