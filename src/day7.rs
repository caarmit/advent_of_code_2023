use crate::day_runner::DayRunner;
use std::collections::HashMap;

pub struct Day7 {}

pub struct Hand {
    bet: u32,
    value: u64,
}

impl Hand {
    fn new(bet: u32, value: u64) -> Hand {
        return Hand {
            bet: bet,
            value: value,
        };
    }
}

impl DayRunner for Day7 {
    fn run_p1(&self, lines: Vec<String>) {
        let mut games = lines
            .iter()
            .map(|i| i.split_whitespace().collect::<Vec<_>>())
            .map(|split| {
                let cards: String = split[0].to_owned();
                let bet = split[1].parse::<u32>().unwrap();
                let value = get_hand_value(&cards);

                Hand::new(bet, value)
            })
            .collect::<Vec<_>>();

        games.sort_by(|a, b| a.value.cmp(&b.value));

        let mut winnings = 0;
        let mut game_id = 0;

        for game in games {
            game_id += 1;
            winnings += game_id * game.bet;
        }

        println!("Winnings: {}", winnings);
    }

    fn run_p2(&self, lines: Vec<String>) {
        let mut games = lines
            .iter()
            .map(|i| i.split_whitespace().collect::<Vec<_>>())
            .map(|split| {
                let cards = split[0].to_owned();
                let bet = split[1].parse::<u32>().unwrap();
                let (_, hand_value) = resolve_wildcards(&cards, &cards);
                Hand::new(bet, hand_value)
            })
            .collect::<Vec<_>>();

        games.sort_by(|a, b| a.value.cmp(&b.value));

        let mut winnings = 0;
        let mut game_id = 0;

        for game in games {
            game_id += 1;
            winnings += game_id * game.bet;
        }

        println!("Winnings: {}", winnings);
    }
}

fn resolve_wildcards(cards_for_pairs: &str, cards_for_values: &str) -> (String, u64) {
    if !cards_for_pairs.contains("J") {
        return (
            cards_for_pairs.to_owned(),
            get_hand_value_pt2(cards_for_pairs, cards_for_values),
        );
    }

    let best_possibility = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
        .iter()
        .map(|c| resolve_wildcards(&cards_for_pairs.replacen("J", c, 1), cards_for_values))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    return best_possibility;
}

fn get_hand_value(cards: &str) -> u64 {
    get_hand_type(cards) + get_card_strengths(cards) as u64
}

fn get_hand_value_pt2(cards_for_pairs: &str, cards_for_values: &str) -> u64 {
    get_hand_type(cards_for_pairs) + get_card_strengths_pt2(cards_for_values) as u64
}

fn get_hand_type(cards: &str) -> u64 {
    let map = get_card_counts(&cards);

    let mut has_three = false;
    let mut two_count: u8 = 0;

    for (_, amount) in map {
        if amount == 5 {
            return 1 << 63;
        }

        if amount == 4 {
            return 1 << 62;
        }

        if amount == 3 {
            has_three = true;
        }

        if amount == 2 {
            two_count += 1;
        }
    }

    if has_three {
        if two_count > 0 {
            return 1 << 61;
        } else {
            return 1 << 60;
        }
    }

    if two_count == 2 {
        return 1 << 59;
    }

    if two_count == 1 {
        return 1 << 58;
    }

    return 0;
}

fn get_card_strengths(cards: &str) -> u32 {
    let mut total: u32 = 0;
    let mut multiplier = 1;
    for card in cards.chars().rev() {
        total += get_card_strength(card) * multiplier;
        multiplier *= 13;
    }

    return total;
}

fn get_card_strengths_pt2(cards: &str) -> u32 {
    let mut total: u32 = 0;
    let mut multiplier = 1;

    for card in cards.chars().rev() {
        total += get_card_strength_pt2(card) * multiplier;
        multiplier *= 13;
    }

    return total;
}

fn get_card_counts(cards: &str) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in cards.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    return counts;
}

fn get_card_strength(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}

fn get_card_strength_pt2(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }
}