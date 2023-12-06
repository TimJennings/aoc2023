use nom::{
    bytes::complete::tag,
    bytes::complete::take,
    bytes::complete::take_while1,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res},
    error::Error,
    multi::separated_list0,
    number,
    sequence::pair,
    sequence::tuple,
    IResult,
};

use crate::common::file_io::read_file_to_string;

pub fn puz1() {
    let input = read_file_to_string("input/day6-input");
    let answer = calc_ans1(&input);
    println!("multiply race win options: {answer}");
}

pub fn puz2() {
    let input = read_file_to_string("input/day6-input");
    let answer = calc_ans2(&input);
    println!("multiply race win options: {answer}");
}

fn calc_ans2(input: &str) -> u64 {
    let input = input.replace(" ", "");
    let (input, time) = parse_time2(&input).unwrap();
    let (input, distance) = parse_distance2(&input).unwrap();

    println!("{time:?}");
    println!("{distance:?}");

    let duration = time.parse().unwrap();
    let length_to_beat: i64 = distance.parse().unwrap();

    let mut hold_time: i64 = 1;

    let mut number_of_options = 0;
    while hold_time < duration {
        // for this hold time calculate distance travelled
        let distance = (duration - hold_time) * hold_time;

        if distance > length_to_beat {
            number_of_options += 1;
        }
        hold_time += 1;
    }

    let answer = number_of_options;
    answer
}

fn calc_ans1(input: &str) -> u32 {
    let (input, times) = parse_time(input).unwrap();
    let (input, distance) = parse_distance(input).unwrap();

    let mut races: Vec<(u32, u32)> = Vec::new();

    let mut index = 0;
    for time in times {
        races.push((time.parse().unwrap(), distance[index].parse().unwrap()));
        index += 1;
    }

    println!("{races:?}");

    let mut multiply_race_win_options: u32 = 0;

    for race in races {
        let duration = race.0;
        let length_to_beat = race.1;

        let mut hold_time = 1;

        let mut winning_times: Vec<(u32, u32)> = Vec::new();
        while hold_time < duration {
            // for this hold time calculate distance travelled
            let distance = (duration - hold_time) * hold_time;

            if distance > length_to_beat {
                winning_times.push((hold_time, distance));
            }
            hold_time += 1;
        }

        if multiply_race_win_options == 0 {
            multiply_race_win_options = winning_times.len() as u32;
        } else {
            multiply_race_win_options *= winning_times.len() as u32;
        }
        println!("found the following winning times {winning_times:?}");
    }

    let answer = multiply_race_win_options;
    answer
}

fn parse_time2(input: &str) -> IResult<&str, &str> {
    let time_parser = pair(tag("Time:"), digit1);

    map(time_parser, |(_, times)| times)(input)
}

fn parse_distance2(input: &str) -> IResult<&str, &str> {
    let time_parser = pair(tag("\nDistance:"), digit1);

    map(time_parser, |(_, times)| times)(input)
}

fn parse_time(input: &str) -> IResult<&str, Vec<&str>> {
    let time_parser = pair(
        pair(tag("Time:"), multispace0),
        separated_list0(multispace1, digit1),
    );

    map(time_parser, |(_, times)| Vec::from(times))(input)
}

fn parse_distance(input: &str) -> IResult<&str, Vec<&str>> {
    let time_parser = pair(
        pair(tag("\nDistance:"), multispace1),
        separated_list0(multispace1, digit1),
    );

    map(time_parser, |(_, times)| Vec::from(times))(input)
}

mod test {
    use crate::day6::{calc_ans1, calc_ans2};

    #[test]
    pub fn test() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";

        let answer = calc_ans1(&input);
        assert_eq!(answer, 288);
    }

    #[test]
    pub fn test2() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";

        let answer = calc_ans2(&input);
        assert_eq!(answer, 71503);
    }
}
