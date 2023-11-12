use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use toml;

static SBEMAILS_FILE: &'static[u8] = include_bytes!("sbemails.toml");

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
    let sbemails: Sbemails = toml::from_str(
            std::str::from_utf8(SBEMAILS_FILE).unwrap()
        )
        .expect("failed to deserialize sbemails.toml");
    let mut random_number = thread_rng();
    let sbemail = sbemails.emails.choose(&mut random_number).unwrap();
    println!("sbemail number: {:?}", sbemail.number);
    println!("sbemail title: {:?}", sbemail.title);
}

