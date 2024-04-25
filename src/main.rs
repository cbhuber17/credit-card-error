use std::{collections::HashMap, io};

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    println!("\nCredit card info: {result:?}");
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Card {
    let card_string = credit_cards.get(name).unwrap();

    let card = parse_card(card_string);

    card
}

fn parse_card(card: &str) -> Card {
    let mut numbers = parse_card_numbers(card);

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Card {
        number,
        exp: Expiration { year, month },
        cvv,
    }
}

fn parse_card_numbers(card: &str) -> Vec<u32> {
    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    numbers
}
