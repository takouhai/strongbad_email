use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct Sbemails {
    emails: Vec<Emails>,
}

#[derive(Deserialize)]
struct Emails {
    number: u8,
    title: String,
    content: String,
}

fn main() {
    println!("Preeeeow, world.");
    let the_paper = include_str!("sbemails.toml");
    let sbemails: Sbemails = toml::from_str(the_paper)
        .expect("failed to deserialize sbemails.toml");
    let mut random_number = thread_rng();
    let sbemail = sbemails.emails.choose(&mut random_number).unwrap();
    println!("sbemail number: {:?}", sbemail.number);
    println!("sbemail title: {:?}", sbemail.title);
    println!("sbemail content: {:?}", sbemail.content);
}

