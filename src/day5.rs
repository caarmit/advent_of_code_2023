use rayon::prelude::*;
use std::collections::VecDeque;

use crate::DayRunner;

pub struct Day5 {}

pub struct Mapping {
    dest_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl DayRunner for Day5 {
    fn run_p1(&self, lines: Vec<String>) {
        let seeds = lines
            .first()
            .unwrap()
            .replace("seeds:", "")
            .trim()
            .split(" ")
            .map(|seed| seed.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let maps = read_maps(&lines);

        let min = seeds
            .iter()
            .map(|seed| get_location(*seed, &maps))
            .min()
            .unwrap();

        println!("Min: {}", min);
    }

    fn run_p2(&self, lines: Vec<String>) {
        let mut seeds_ranges = lines
            .first()
            .unwrap()
            .replace("seeds:", "")
            .trim()
            .split(" ")
            .map(|seed| seed.parse::<i64>().unwrap())
            .collect::<VecDeque<_>>();

        let maps = read_maps(&lines);

        let _pairs: Vec<(i64, i64)> = Vec::new();
        let mut min = i64::MAX;

        while seeds_ranges.len() > 0 {
            let range_1 = seeds_ranges.pop_front().unwrap();
            let range_2 = seeds_ranges.pop_front().unwrap();

            let new_min = (range_1..(range_2 + range_1))
                .into_par_iter()
                .map(|element| get_location(element, &maps))
                .min()
                .unwrap();

            if new_min < min {
                min = new_min;
            }
        }

        println!("Overall Min: {}", min);
    }
}

fn get_location(seed: i64, maps: &Vec<Vec<Mapping>>) -> i64 {
    let mut current_step = seed;

    for map in maps.iter() {
        let mapping = map.iter().find(|mapping| {
            mapping.source_range_start <= current_step
                && mapping.source_range_start + mapping.range_length > current_step
        });

        let map_offset: i64 = match mapping {
            Some(remapping) => remapping.dest_range_start - remapping.source_range_start,
            None => 0,
        };

        current_step = current_step + map_offset;
    }

    return current_step;
}

fn read_maps(lines: &Vec<String>) -> Vec<Vec<Mapping>> {
    let mut maps: Vec<Vec<Mapping>> = Vec::new();

    let mut has_current_map = false;
    let mut current_map_name: String = "".to_string();
    let mut current_map: Vec<Mapping> = Vec::new();

    for line in lines.iter().map(|line| line.trim()) {
        if line.is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            if has_current_map {
                maps.push(current_map);
            }

            has_current_map = true;
            current_map_name = line.replace(" map:", "");
            current_map = Vec::new();

            continue;
        }

        if !has_current_map {
            continue;
        }

        let map_split = line.split(" ").collect::<Vec<_>>();
        let dest_range_start = map_split[0].parse::<i64>().unwrap();
        let source_range_start = map_split[1].parse::<i64>().unwrap();
        let range_length = map_split[2].parse::<i64>().unwrap();

        current_map.push(Mapping {
            dest_range_start: dest_range_start,
            source_range_start: source_range_start,
            range_length: range_length,
        });

        println!(
            "Mapping {} {} {} for {}",
            dest_range_start, source_range_start, range_length, current_map_name
        );
    }

    if has_current_map {
        maps.push(current_map);
    }

    return maps;
}
