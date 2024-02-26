// Note: CRIMES WERE COMMITTED ON THIS DAY. THIS AIN'T RIGHT MAN...

use std::{vec, collections::HashMap};
use crate::day_runner::DayRunner;

pub struct Day3;

struct Part {
    part_number: u32,
    touching: Vec<(char, usize, usize)>
}

impl DayRunner for Day3 {
    fn run_p1(&self, lines: Vec<String>) {
        
        let parts = extract_parts(lines);
        let mut sum = 0;

        for part in &parts {
            sum += part.part_number;
        }

        println!("Result: {}", sum);
    }

    fn run_p2(&self, lines: Vec<String>) {

        let parts = extract_parts(lines);
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        
        for part in &parts {
            
            for gear in (&part).touching.iter().filter(|gear|gear.0 == '*') {

                let key = (gear.1, gear.2);

                match gears.get_mut(&key) {
                    Some(parts_on_gear) => {
                        parts_on_gear.push(part.part_number);
                    }
                    None => {
                        gears.insert(key, vec![part.part_number]);
                    }
                }
            }
        }

        let mut sum = 0;

        for adjacents in gears.values().into_iter() {
            if adjacents.len() == 2 {
                sum += adjacents.iter().product::<u32>();
            }
        }

        println!("Result {}", sum);
    }

}

fn extract_parts(lines: Vec<String>) -> Vec<Part> { 

    // Pad this so we can avoid doing really annoying bounds checks later... Fell over for ages so came to this hack :(.
    let mut char_lines: Vec<Vec<char>> = vec![];
    char_lines.push(".".repeat(lines[1].chars().count() + 2).chars().collect());

    for line in &lines {
        char_lines.push(format!(".{}.", line.to_string()).chars().collect());
    }

    char_lines.push(".".repeat(lines[1].chars().count() + 2).chars().collect());

    let mut line_number: usize = 0;
    let mut parts: Vec<Part> = vec![];

    for line in &char_lines {

        let mut reading_number = false;
        let mut start_index = 0;
        let mut char_index = 0;

        for character in line {
            
            if character.is_alphanumeric() {

                if !reading_number {
                    reading_number = true;
                    start_index = char_index;
                }
            } else {
                if reading_number {
                    let str: String = line[start_index..char_index].iter().collect();
                    
                    let touching_symbols = get_touching_symbols(line_number, start_index, char_index - 1, &char_lines);

                    if touching_symbols.len() > 0 {
                        parts.push(Part {
                            part_number: str.parse::<u32>().unwrap(),
                            touching: touching_symbols
                        });
                    }

                    reading_number = false;
                    start_index = 0;
                }
            }

            char_index += 1;
        }

        line_number += 1;
    }

    return parts;
}

fn get_touching_symbols(line_number: usize, start: usize, end: usize, lines: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {

    let mut results: Vec<(char, usize, usize)> = vec![];

    for x in start-1..end + 2 { // end is exclusive.....
        for y in line_number-1..line_number+2 {
            let char = lines[y][x];
            
            if !char.is_alphanumeric() && char != '.' {
                results.push((char, y, x));
            }
        }
    }

    return results;
}