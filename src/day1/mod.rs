use crate::common::file_io;
use regex::Regex;

pub fn puz2() {
    let lines: Vec<String> = file_io::read_file_to_vec("input/day1-input");

    let mut calibration_value: u32 = 0;

    for line in lines {
        let line = &line;
        // find first match
        let matcher =
            Regex::new("[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)");
        let reverse_matcher =
            Regex::new("[0-9]|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)");
        let forward_re = matcher.unwrap();
        let backward_re = reverse_matcher.unwrap();

        let first = forward_re
            .find(&line)
            .expect("failed to find first")
            .as_str();
        let first = token_to_number(first);

        let reverse_line: String = line.chars().rev().collect();
        let last: String = backward_re
            .find(&reverse_line)
            .expect("failed to find last")
            .as_str()
            .chars()
            .rev()
            .collect();
        let last = token_to_number(last.as_str());

        let assembled_number = String::new() + &first.to_string() + &last.to_string();
        let number: u32 = assembled_number.parse().expect("Failed to parse number");

        calibration_value = calibration_value + number;
    }

    println!("The calibration value is : {calibration_value}");
}

fn token_to_number(token: &str) -> &str {
    let var_name = match token {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        c => c,
    };
    return var_name;
}

pub fn puz1() {
    let lines: Vec<String> = file_io::read_file_to_vec("input/day1-input");

    let mut calibration_value: u32 = 0;

    for line in lines {
        let line = &line;
        // find first and last number character, assemble into number
        let first_number = match line.chars().find(|c| c.is_numeric()) {
            Some(c) => c,
            None => continue,
        };

        let second_number = match line.chars().rev().find(|c| c.is_numeric()) {
            Some(c) => c,
            None => continue,
        };

        let assembled_number =
            String::new() + &first_number.to_string() + &second_number.to_string();

        // add to calibration number
        let number: u32 = assembled_number.parse().expect("Failed to parse number");

        calibration_value = calibration_value + number;
    }

    println!("The calibration value is : {calibration_value}");
}
