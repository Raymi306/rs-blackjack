use std::io;
use std::fmt;
use rand::thread_rng;
use rand::prelude::SliceRandom;

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
    println!("");
    io::stdin().read_line(&mut input).unwrap();
    println!("");
    input
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
              match self {
                  Suit::Diamonds => "Diamonds",
                  Suit::Hearts => "Hearts",
                  Suit::Clubs => "Clubs",
                  Suit::Spades => "Spades",
              }
              )
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Value {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   Value::Ace => "Ace",
                   Value::Two => "2",
                   Value::Three => "3",
                   Value::Four => "4",
                   Value::Five => "5",
                   Value::Six => "6",
                   Value::Seven => "7",
                   Value::Eight => "8",
                   Value::Nine => "9",
                   Value::Ten => "10",
                   Value::Jack => "Jack",
                   Value::Queen => "Queen",
                   Value::King => "King",
               }
               )
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub const fn new(suit: Suit, value: Value) -> Self {
        Card {
            value,
            suit
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

const DECK: [Card; 52] = [
    Card::new(Suit::Clubs, Value::Ace),
    Card::new(Suit::Clubs, Value::Two),
    Card::new(Suit::Clubs, Value::Three),
    Card::new(Suit::Clubs, Value::Four),
    Card::new(Suit::Clubs, Value::Five),
    Card::new(Suit::Clubs, Value::Six),
    Card::new(Suit::Clubs, Value::Seven),
    Card::new(Suit::Clubs, Value::Eight),
    Card::new(Suit::Clubs, Value::Nine),
    Card::new(Suit::Clubs, Value::Ten),
    Card::new(Suit::Clubs, Value::Jack),
    Card::new(Suit::Clubs, Value::Queen),
    Card::new(Suit::Clubs, Value::King),
    Card::new(Suit::Diamonds, Value::Ace),
    Card::new(Suit::Diamonds, Value::Two),
    Card::new(Suit::Diamonds, Value::Three),
    Card::new(Suit::Diamonds, Value::Four),
    Card::new(Suit::Diamonds, Value::Five),
    Card::new(Suit::Diamonds, Value::Six),
    Card::new(Suit::Diamonds, Value::Seven),
    Card::new(Suit::Diamonds, Value::Eight),
    Card::new(Suit::Diamonds, Value::Nine),
    Card::new(Suit::Diamonds, Value::Ten),
    Card::new(Suit::Diamonds, Value::Jack),
    Card::new(Suit::Diamonds, Value::Queen),
    Card::new(Suit::Diamonds, Value::King),
    Card::new(Suit::Hearts, Value::Ace),
    Card::new(Suit::Hearts, Value::Two),
    Card::new(Suit::Hearts, Value::Three),
    Card::new(Suit::Hearts, Value::Four),
    Card::new(Suit::Hearts, Value::Five),
    Card::new(Suit::Hearts, Value::Six),
    Card::new(Suit::Hearts, Value::Seven),
    Card::new(Suit::Hearts, Value::Eight),
    Card::new(Suit::Hearts, Value::Nine),
    Card::new(Suit::Hearts, Value::Ten),
    Card::new(Suit::Hearts, Value::Jack),
    Card::new(Suit::Hearts, Value::Queen),
    Card::new(Suit::Hearts, Value::King),
    Card::new(Suit::Spades, Value::Ace),
    Card::new(Suit::Spades, Value::Two),
    Card::new(Suit::Spades, Value::Three),
    Card::new(Suit::Spades, Value::Four),
    Card::new(Suit::Spades, Value::Five),
    Card::new(Suit::Spades, Value::Six),
    Card::new(Suit::Spades, Value::Seven),
    Card::new(Suit::Spades, Value::Eight),
    Card::new(Suit::Spades, Value::Nine),
    Card::new(Suit::Spades, Value::Ten),
    Card::new(Suit::Spades, Value::Jack),
    Card::new(Suit::Spades, Value::Queen),
    Card::new(Suit::Spades, Value::King),
    ];

struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck(Vec::with_capacity(52));
        deck.0.extend(DECK.iter());
        deck
    }
}

struct Hand(Vec<Card>);

impl Hand {
    pub const fn new() -> Hand {
        let hand = Hand(Vec::new());
        hand
    }
    pub fn score(&self) -> i16 {
        let mut acc: i16 = 0;
        let mut aces: i16 = 0;
        for card in &self.0 {
            match card.value as i16 {
                11..=13 => acc += 10,
                1 => aces += 1,
                _ => acc += card.value as i16,
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
}

struct GameContext {
    deck: Deck,
    dealer: Hand,
    player: Hand,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            deck: Deck::new(),
            dealer: Hand::new(),
            player: Hand::new(),
        }
    }
}

enum GameState {
    Start,
    PlayerTurn,
    DealerTurn,
    Scoring,
    Continue,
    End,
}

fn initial_deal(ctx: &mut GameContext) {
    ctx.player.0.push(ctx.deck.0.pop().unwrap());
    ctx.player.0.push(ctx.deck.0.pop().unwrap());
    ctx.dealer.0.push(ctx.deck.0.pop().unwrap());
    ctx.dealer.0.push(ctx.deck.0.pop().unwrap());
}

fn reshuffle(ctx: &mut GameContext) {
    ctx.deck.0.append(&mut ctx.dealer.0.drain(..).collect());
    ctx.deck.0.append(&mut ctx.player.0.drain(..).collect());
    ctx.deck.0.shuffle(&mut thread_rng());
}

fn game_start(ctx: &mut GameContext) -> GameState {
    reshuffle(ctx);
    initial_deal(ctx);
    print_vec("PLAYER HAND", &ctx.player.0);
    print_vec("DEALER HAND", &ctx.dealer.0);
    if ctx.dealer.score() == 21
    {
        GameState::Scoring
    }
    else
    {
        GameState::PlayerTurn
    }
}

fn player_turn(ctx: &mut GameContext) -> GameState {
    loop {
        match get_choice(vec!["Hit (h)", "Stand (s)",]).trim() {
            "h" => ctx.player.0.push(ctx.deck.0.pop().unwrap()),
            "s" => return GameState::DealerTurn,
            "q" => return GameState::End,
            _ => continue,
        };
        print_vec("PLAYER HAND", &ctx.player.0);
        if ctx.player.score() > 21
        {
            return GameState::Scoring
        }
    }
}

fn dealer_turn(ctx: &mut GameContext) -> GameState {
    while ctx.dealer.score() < 17 {
        ctx.dealer.0.push(ctx.deck.0.pop().unwrap());
    }
    print_vec("DEALER HAND", &ctx.dealer.0);
    GameState::Scoring
}

fn scoring(ctx: &mut GameContext) -> GameState {
    println!("Dealer score: {}, Player score: {}\n", ctx.dealer.score(), ctx.player.score());
    if ctx.dealer.score() == 21 ||
        ctx.player.score() > 21 ||
        (ctx.dealer.score() >= ctx.player.score() && ctx.dealer.score() <= 21)
        {

            println!("Dealer wins\n");
        }
    else
    {
        println!("Player wins\n");
    }
    return GameState::Continue
}

fn play_again() -> GameState {
    match get_choice(vec!["Play again? (y/n)"]).trim() {
        "n" => GameState::End,
        _ => GameState::Start,
    }
}

fn main() {
    let mut ctx = GameContext::new();
    let mut state = GameState::Start;
    loop {
        state = match state {
            GameState::Start => game_start(&mut ctx),
            GameState::PlayerTurn => player_turn(&mut ctx),
            GameState::DealerTurn => dealer_turn(&mut ctx),
            GameState::Scoring => scoring(&mut ctx),
            GameState::Continue => play_again(),
            GameState::End => break,
        };
    }
    println!("Fin.");
}
