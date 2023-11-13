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
    let the_paper = include_str!("sbemails.toml");
    let sbemails: Sbemails =
        toml::from_str(the_paper).expect("failed to deserialize sbemails.toml");
    let mut random_number = thread_rng();
    let sbemail = sbemails.emails.choose(&mut random_number).unwrap();
    println!("sbemail #{}: {}", sbemail.number, sbemail.title);
    let sbemail_content = sbemail.content.split("\n");
    for part in sbemail_content {
        println!("{}", part);
    }
}
