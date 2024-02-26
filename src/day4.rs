use std::collections::HashSet;

use crate::day_runner::DayRunner;

pub struct Day4 {}

impl DayRunner for Day4 {
    fn run_p1(&self, lines: Vec<String>) {
        let scores: Vec<usize> = get_card_matches(&lines);

        let mut total_score = 0;

        for matches in scores {

            if matches > 0 {
                let mut game_score = 1;

                for _i in 1..matches {
                    game_score *= 2;
                }

                total_score += game_score;
            }
        }

        println!("Game Score: {}", total_score);
    }

    fn run_p2(&self, lines: Vec<String>) {
        let scores: Vec<usize> = get_card_matches(&lines);

        let mut ticket_count = 0;
        let mut scratchcard_counts: Vec<usize> = (0..lines.len()).map(|_i|1).collect::<Vec<_>>();

        for scratchcard_number in 0..scratchcard_counts.len() {
            let cards_held = scratchcard_counts[scratchcard_number];
            ticket_count += cards_held;
            
            let match_count = scores[scratchcard_number];

            for i in 0..match_count {

                let won_number = scratchcard_number + i + 1;
                if won_number < scores.len() {
                    scratchcard_counts[won_number] += cards_held;
                }
            }
        }

        println!("Total Tickets: {}", ticket_count);
    }
}

fn get_card_matches(lines: &Vec<String>) -> Vec<usize> {
    let games = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .skip(line.chars().position(|c| c == ':').unwrap() + 1)
                .collect::<String>()
        })
        .map(|game| {
            game.split('|')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut results: Vec<usize> = Vec::new();

    for game_numbers in games {
        // println!("Game: '{}' | '{}'", &game_numbers[0], &game_numbers[1]);

        let my_numbers = &game_numbers[0]
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let winning_numbers = &game_numbers[1]
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        results.push(my_numbers.intersection(winning_numbers).count());
    }

    return results;
}
