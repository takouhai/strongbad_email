use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
struct Sbemails {
    emails: Vec<Emails>,
}

#[derive(Debug, Deserialize)]
struct Emails {
    number: u8,
    title: String,
}

fn main() {
    println!("Preeeeow, world.");
    let toml_file = fs::read_to_string("src/sbemails.toml")
        .expect("failed to read sbemails.toml");
    let sbemails: Sbemails = toml::from_str(&toml_file)
        .expect("failed to deserialize sbemails.toml");
    let mut random_number = thread_rng();
    let sbemail = sbemails.emails.choose(&mut random_number).unwrap();
    println!("sbemail number: {:?}", sbemail.number);
    println!("sbemail title: {:?}", sbemail.title);
}

