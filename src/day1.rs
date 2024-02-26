use regex::Regex;

use crate::day_runner::DayRunner;

pub struct Day1 {}

impl DayRunner for Day1 {
    
    fn run_p1(&self, _: Vec<String>) {
        println!("Run");
    }

    fn run_p2(&self, lines: Vec<String>) {

        let mut sum = 0;
        let regex = Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap();

        for line in lines {
            sum += parse_line(&line, &regex);
        }

        println!("{sum}");    
    }
}

fn parse_line(line: &str, regex: &Regex) -> u32 {

    
    // let numbers: Vec<char> = line.chars().into_iter().filter(|c| c.is_numeric()).collect();
    
    let mut finds: Vec<&str> = vec![];
    let mut start = 0;

    loop {
        match regex.find_at(line, start) {
            Some(result) => {
                finds.push(result.as_str());
                start = start + 1;
            },
            None => {
                break
            }
        }
    }
 
    let first = parse_number(finds.first().unwrap());
    let last = parse_number(finds.last().unwrap());

    let result = format!("{}{}", first, last).parse::<u32>().unwrap();
    // println!("{},{} makes {} ({})", first, last, result, line);
    
    result
}

fn parse_number(num: &str) -> u32 {
    match num.parse::<u32>() {
        Ok(result) => {
            return result
        },
        Err(_)  => {

            return match num {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("Unknown number")
            }
        }
    }
}