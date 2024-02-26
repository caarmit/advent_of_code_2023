use std::collections::VecDeque;

use crate::day_runner::DayRunner;

pub struct Day6 {}

struct Race {
    time: u64,
    distance: u64
}

impl DayRunner for Day6 {
    fn run_p1(&self, lines: Vec<String>) {
        let races = get_race_data(lines);
        solve_race(races);
    }

    fn run_p2(&self, lines: Vec<String>) {
        let races = get_race_data_pt2(lines);
        solve_race(races);
    }
}

fn solve_race(races: Vec<Race>) {

    let mut i = 0;
    let mut total_possibilities: i32 = 1;

    for race in races {
        i+=1;

        let mut possibilities = 0;
        
        for holding_time in 1..race.time {
            let time_remaining = race.time - holding_time;
            let speed = holding_time;

            let time_taken = (race.distance as f64) / (speed as f64);
            
            if time_taken > time_remaining as f64 {
                continue;
            }

            let distance_travelled = speed * time_remaining;
            if distance_travelled > race.distance {
                possibilities += 1;
            }
        }

        total_possibilities *= possibilities;

        println!("{} possibilities for race {}", possibilities, i);
    }

    println!("Total Possibilities {}", total_possibilities);
}

fn get_race_data_pt2(lines: Vec<String>) -> Vec<Race> {
    let l1 = lines[0].replace("Time:", "").replace(" ", "").trim().parse::<u64>().unwrap();
    let l2 = lines[1].replace("Distance:", "").replace(" ", "").trim().parse::<u64>().unwrap();
    
    return vec![Race {
        time: l1,
        distance: l2
    }];
}

fn get_race_data(lines: Vec<String>) -> Vec<Race> {

    let mut l1 = lines[0].replace("Time:", "").trim().split_whitespace().map(|n|n.parse::<u64>()).collect::<VecDeque<_>>();
    let mut l2 = lines[1].replace("Distance:", "").trim().split_whitespace().map(|n|n.parse::<u64>()).collect::<VecDeque<_>>();

    let mut races: Vec<Race> = Vec::new();

    for _i in 0..l1.len() {
        let race = Race {
            time: l1.pop_front().unwrap().unwrap(),
            distance: l2.pop_front().unwrap().unwrap()
        };

        races.push(race);
    }

    return races;
}
