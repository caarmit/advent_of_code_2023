use core::panic;
use std::collections::HashMap;

use crate::day_runner::DayRunner;

#[derive(Debug)]
pub struct Day2 {}

struct HandResult {
    amount: u32,
    color: String
}

impl DayRunner for Day2 {
    fn run_p1(&self, lines: Vec<String>) {
        let mut sum = 0;
        let mut game_id = 0;

        for line in lines {
            game_id += 1;

            let mut is_possible = true;
            for hand_result in iterate_results(line) {

                if hand_result.amount > match hand_result.color.as_str() {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Unknown color")
                } {
                    is_possible = false;
                    break;
                }
            }

            if is_possible {
                sum += game_id;
            }
        }

        println!("Result: {sum}");
    }

    fn run_p2(&self, lines: Vec<String>) {
        let mut sum = 0;

        for line in lines {

            let mut results: HashMap<String, u32> = vec![
                ("red".to_string(), 0),
                ("green".to_string(), 0),
                ("blue".to_string(), 0)
            ].into_iter().collect();

            for hand_result in iterate_results(line) {
                if hand_result.amount > results[&hand_result.color] {
                    results.insert(hand_result.color, hand_result.amount);
                }
            }

            sum += results.values().product::<u32>();
        }

        println!("Product of all games: {}", sum);
    }
}

fn iterate_results(game: String) -> impl Iterator<Item = HandResult> {
    let trimmed_game = game.split(":").nth(1).unwrap().replace(";", ",");

    trimmed_game.split(",").map(move |result| {
        let result = result.trim_start();

        let mut amount_and_color = result.split_whitespace();
        let amount = amount_and_color.next().unwrap().parse::<u32>().unwrap();
        let color = amount_and_color.next().unwrap().to_string();

        HandResult { amount, color }
    }).collect::<Vec<_>>().into_iter()
}