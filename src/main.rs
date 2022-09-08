use rand::seq::SliceRandom;
use std::{fs, io};

fn main() {
    let blacks = Deck::read("./data/blacks.txt").unwrap();
    let whites = Deck::read("./data/whites.txt").unwrap();
    println!("Cards Against Programmers\n");

    for card in blacks.iter() {
        let answer_count = card.matches('_').count();
        let mut sentence = card.to_string();

        for _ in 0..answer_count {
            let answer = whites.random();
            sentence = sentence.replacen('_', &answer, 1)
        }

        println!("{sentence}")
    }
}

#[derive(Debug)]
struct Deck<'deck> {
    cards: Vec<&'deck str>,
}

impl Deck<'_> {
    fn read(path: &str) -> io::Result<Self> {
        let raw = Box::leak(Box::new(fs::read_to_string(path)?));

        Ok(Self {
            cards: raw.lines().collect(),
        })
    }

    fn iter(&self) -> std::slice::Iter<&str> {
        self.cards.iter()
    }

    fn random(&self) -> &&str {
        self.cards.choose(&mut rand::thread_rng()).unwrap()
    }
}
