use regex::Regex;

use crate::common::file_io;

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

struct Pull {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

pub fn puz1() {
    let lines = file_io::read_file_to_vec("input/day2-input");

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut sum_of_game_ids = 0;

    for line in lines {
        let line = parse_line(&line);

        if line.red <= MAX_RED && line.blue <= MAX_BLUE && line.green <= MAX_GREEN {
            // game possible
            sum_of_game_ids = sum_of_game_ids + line.id;
        }
    }

    println!("puz1: Sum of possible game ids {sum_of_game_ids}");
}

pub fn puz2() {
    let lines = file_io::read_file_to_vec("input/day2-input");

    let mut sum_of_game_powers = 0;
    for line in lines {
        let line = parse_line(&line);

        let game_power = line.blue * line.green * line.red;
        sum_of_game_powers = sum_of_game_powers + game_power;
    }

    println!("puz2: Sum of game powers: {sum_of_game_powers}");
}

fn parse_line(line: &str) -> Game {
    let matcher = Regex::new("^Game ([0-9]*):(.*)$");
    let re = matcher.unwrap();
    let capture = re.captures(line).expect("failed to parse line");
    let id = capture.get(1).expect("couldnt find id").as_str();
    let pulls = capture.get(2).expect("couldn't get games").as_str();

    let parsed_pulls: Vec<Pull> = pulls
        .split(";")
        .map(|s| s.trim())
        .map(|s| parse_pull(s))
        .collect();

    // find highest number of each color in all games played
    let mut game = Game {
        id: id.parse().expect("game is not a number"),
        red: 0,
        green: 0,
        blue: 0,
    };
    for pull in parsed_pulls {
        if pull.red.unwrap_or(0) > game.red {
            game.red = pull.red.unwrap()
        }

        if pull.blue.unwrap_or(0) > game.blue {
            game.blue = pull.blue.unwrap()
        }

        if pull.green.unwrap_or(0) > game.green {
            game.green = pull.green.unwrap()
        }
    }
    game
}

fn parse_pull(game: &str) -> Pull {
    let pulls: Vec<&str> = game.split(",").map(|s| s.trim()).collect();
    let mut parsed_pull = Pull {
        red: None,
        blue: None,
        green: None,
    };

    for pull in pulls {
        let split: Vec<&str> = pull.split(" ").collect();
        let number: u32 = split[0].parse().expect("failed to parse number of cubes");
        let color = split[1];

        match color {
            "red" => parsed_pull.red = Some(number),
            "blue" => parsed_pull.blue = Some(number),
            "green" => parsed_pull.green = Some(number),
            _ => panic!("unexpected color {color}"),
        }
    }
    parsed_pull
}
