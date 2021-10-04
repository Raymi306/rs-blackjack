use std::io;
use std::fmt;
use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(Debug)]
enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

#[derive(Debug)]
struct Card {
    value: i8,
    suit: Suit,
}

fn new_card(value: i8, suit: Suit) -> Card {
    Card {
        value,
        suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name: String;
        match self.value {
            1 => name = "Ace".to_string(),
            2 => name = "Two".to_string(),
            3 => name = "Three".to_string(),
            4 => name = "Four".to_string(),
            5 => name = "Five".to_string(),
            6 => name = "Six".to_string(),
            7 => name = "Seven".to_string(),
            8 => name = "Eight".to_string(),
            9 => name = "Nine".to_string(),
            10 => name = "Ten".to_string(),
            11 => name = "Jack".to_string(),
            12 => name = "Queen".to_string(),
            13 => name = "King".to_string(),
            _ => panic!("Illegal card value"),
        };
        match self.suit {
            Suit::Diamonds => name = name.to_owned() + " of Diamonds",
            Suit::Hearts => name = name.to_owned() + " of Hearts",
            Suit::Clubs => name = name.to_owned() + " of Clubs",
            Suit::Spades => name = name.to_owned() + " of Spades",
        };
        write!(f, "{}", name)
    }
}

fn new_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for i in 1..=13 {
        deck.push(new_card(i, Suit::Diamonds));
    }
    for i in 1..=13 {
        deck.push(new_card(i, Suit::Hearts));
    }
    for i in 1..=13 {
        deck.push(new_card(i, Suit::Clubs));
    }
    for i in 1..=13 {
        deck.push(new_card(i, Suit::Spades));
    }
    deck
}

fn get_value(cards: &Vec<Card>) -> i16 {
    let mut acc: i16 = 0;
    let mut aces: i16 = 0;
    for card in cards {
        match card.value as i16 {
            11..=13 => acc += 10,
            1 => aces += 1,
            val => acc += val,
        };
    }
    if aces > 0
    {
        if aces > 1
        {
            if acc + 11 > 21
            {
                acc += 1;
            }
            else
            {
                acc += 11;
            }
        }
        else
        {
            if acc + 11 + aces - 1 > 21
            {
                acc += aces;
            }
            else
            {
                acc += 11 + aces - 1
            }
        }
    }
    acc
}

fn initial_deal(deck: &mut Vec<Card>, dealer_hand: &mut Vec<Card>, player_hand: &mut Vec<Card>) {
    player_hand.push(deck.pop().unwrap());
    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
}

fn reshuffle(deck: &mut Vec<Card>, hand_1: &mut Vec<Card>, hand_2: &mut Vec<Card>) {
    deck.append(&mut hand_1.drain(..).collect());
    deck.append(&mut hand_2.drain(..).collect());
    deck.shuffle(&mut thread_rng());
}

fn player_actions(deck: &mut Vec<Card>, player_hand: &mut Vec<Card>) {
    loop {
        match get_choice(vec!["Hit (h)", "Stand (s)",]).trim() {
            "h" => player_hand.push(deck.pop().unwrap()),
            "s" => break,
            _ => continue,
        };
        print_vec("PLAYER HAND", &player_hand);
        if get_value(player_hand) > 21
        {
            break;
        }
    }
}

fn dealer_resolution(deck: &mut Vec<Card>, dealer_hand: &mut Vec<Card>) {
    while get_value(dealer_hand) < 17 {
        dealer_hand.push(deck.pop().unwrap());
        print_vec("DEALER HAND", &dealer_hand);
    }
}

fn print_vec<T: fmt::Display>(label: &str, vec: &Vec<T>) {
    println!("{}", label);
    for item in vec {
        println!("{}", item);
    }
    println!("");
}

fn get_choice(prompts: Vec<&str>) -> String {
    for prompt in prompts {
        println!("{}", prompt);
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn play_again() -> bool {
    match get_choice(vec!["Play again? (y/n)"]).trim() {
        "n" => return false,
        _ => {
            println!("");
            return true;
        },
    };
}


fn main() {
    let mut deck = new_deck();
    let mut player_hand: Vec<Card> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();
    loop {
        reshuffle(&mut deck, &mut player_hand, &mut dealer_hand);
        initial_deal(&mut deck, &mut dealer_hand, &mut player_hand);
        print_vec("DEALER:", &dealer_hand);
        print_vec("PLAYER:", &player_hand);
        if get_value(&dealer_hand) == 21 {
            println!("Dealer wins");
            if !play_again() 
            {
                break;
            }
            else
            {
                continue
            }
        }
        player_actions(&mut deck, &mut player_hand);
        let player_score = get_value(&player_hand);
        if player_score > 21
        {
            println!("Dealer wins");
            if !play_again() 
            {
                break;
            }
            else
            {
                continue
            }
}
        dealer_resolution(&mut deck, &mut dealer_hand);
        let dealer_score = get_value(&dealer_hand);
        println!("DEALER SCORE: {}, PLAYER SCORE: {}", dealer_score, player_score);
        if (dealer_score == 21 || player_score > 21 || dealer_score >= player_score) && dealer_score < 21
        {
            println!("Dealer wins");
        }
        else
        {
            println!("Player wins");
        }
        if !play_again() {
            break;
        };
    }
    println!("Fin.");
}
