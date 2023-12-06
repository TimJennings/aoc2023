use std::{i64::MAX, time::Instant};

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

fn seed_trace(
    seed: i64,
    seed_soil_map: &Vec<AlmanacMap>,
    soil_fert_map: &Vec<AlmanacMap>,
    fert_water_map: &Vec<AlmanacMap>,
    water_light_map: &Vec<AlmanacMap>,
    light_temperature_map: &Vec<AlmanacMap>,
    temp_humid_map: &Vec<AlmanacMap>,
    humid_loc_map: &Vec<AlmanacMap>,
) -> (i64, i64) {
    {
        let now = Instant::now();
        let mut soil = seed;
        for map in seed_soil_map {
            if seed >= map.source_range_start && seed <= map.source_range_start + map.range_length {
                soil = (seed - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let mut fert = soil;
        for map in soil_fert_map {
            if soil >= map.source_range_start && soil <= map.source_range_start + map.range_length {
                fert = (soil - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let mut water = fert;
        for map in fert_water_map {
            if fert >= map.source_range_start && fert <= map.source_range_start + map.range_length {
                water = (fert - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let mut light = water;
        for map in water_light_map {
            if water >= map.source_range_start && water <= map.source_range_start + map.range_length
            {
                light = (water - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let mut temperature = light;
        for map in light_temperature_map {
            if light >= map.source_range_start && light <= map.source_range_start + map.range_length
            {
                temperature = (light - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let mut humidity = temperature;
        for map in temp_humid_map {
            if temperature >= map.source_range_start
                && temperature <= map.source_range_start + map.range_length
            {
                humidity = (temperature - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let mut location = humidity;
        for map in humid_loc_map {
            if humidity >= map.source_range_start
                && humidity <= map.source_range_start + map.range_length
            {
                location = (humidity - map.source_range_start) + map.destination_range_start;
                break;
            }
        }

        let elapsed_time = now.elapsed();
        println!(
            "Running slow_function() took {} nanos.",
            elapsed_time.as_nanos()
        );

        (seed, location)
    }
}

fn location_traceseed(
    location: i64,
    seed_soil_map: &Vec<AlmanacMap>,
    soil_fert_map: &Vec<AlmanacMap>,
    fert_water_map: &Vec<AlmanacMap>,
    water_light_map: &Vec<AlmanacMap>,
    light_temperature_map: &Vec<AlmanacMap>,
    temp_humid_map: &Vec<AlmanacMap>,
    humid_loc_map: &Vec<AlmanacMap>,
) -> (i64, i64) {
    let mut humid = location;
    for map in humid_loc_map {
        if location >= map.destination_range_start
            && location < map.destination_range_start + map.range_length
        {
            humid = (location - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    let mut temp = humid;
    for map in temp_humid_map {
        if humid >= map.destination_range_start
            && humid < map.destination_range_start + map.range_length
        {
            temp = (humid - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    let mut light = temp;
    for map in light_temperature_map {
        if temp >= map.destination_range_start
            && temp < map.destination_range_start + map.range_length
        {
            light = (temp - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    let mut water = light;
    for map in water_light_map {
        if light >= map.destination_range_start
            && light < map.destination_range_start + map.range_length
        {
            water = (light - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    let mut fert = water;
    for map in fert_water_map {
        if water >= map.destination_range_start
            && water < map.destination_range_start + map.range_length
        {
            fert = (water - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    let mut soil = fert;
    for map in soil_fert_map {
        if fert >= map.destination_range_start
            && fert < map.destination_range_start + map.range_length
        {
            soil = (fert - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    let mut seed = soil;
    for map in seed_soil_map {
        if soil >= map.destination_range_start
            && soil < map.destination_range_start + map.range_length
        {
            seed = (soil - map.destination_range_start) + map.source_range_start;
            break;
        }
    }

    (seed, location)
}

pub fn puz1() {
    let input = read_file_to_string("input/day5-input");

    let (input, seed_list) = Almanac::seeds(&input).unwrap();
    let (input, seed_soil_map) = Almanac::seed_to_soil(&input).unwrap();
    let (input, soil_fert_map) = Almanac::soil_to_fertilizer(&input).unwrap();
    let (input, fert_water_map) = Almanac::fertilizer_to_water(&input).unwrap();
    let (input, water_light_map) = Almanac::water_to_light(&input).unwrap();
    let (input, light_temperature_map) = Almanac::light_to_temperature(&input).unwrap();
    let (input, temp_humid_map) = Almanac::temperature_to_humidity(&input).unwrap();
    let (input, humid_loc_map) = Almanac::humidity_to_location(&input).unwrap();

    println!("{seed_list:?}");
    println!("{seed_soil_map:?}");
    println!("{soil_fert_map:?}");
    println!("{fert_water_map:?}");
    println!("{water_light_map:?}");
    println!("{light_temperature_map:?}");
    println!("{temp_humid_map:?}");
    println!("{humid_loc_map:?}");
    println!("{input:?}");

    let seed_location_list: Vec<(i64, i64)> = seed_list
        .iter()
        .map(|s| s.parse().expect("error parsing seed number"))
        .map(|seed| {
            seed_trace(
                seed,
                &seed_soil_map,
                &soil_fert_map,
                &fert_water_map,
                &water_light_map,
                &light_temperature_map,
                &temp_humid_map,
                &humid_loc_map,
            )
        })
        .collect();

    println!("{seed_location_list:?}");

    let mut min_loc: i64 = MAX;
    let mut min_seed: (i64, i64) = (0, 0);
    for (seed, location) in seed_location_list {
        if (location < min_loc) {
            min_loc = location;
            min_seed = (seed, location);
        }
    }

    println!("{min_seed:?}");
}

pub fn puz2() {
    let input = read_file_to_string("input/day5-input");
    reverse_brute_force(&input);
}

fn reverse_brute_force(input: &str) -> (i64, i64) {
    let (input, seed_list) = Almanac::seeds(&input).unwrap();
    let (input, seed_soil_map) = Almanac::seed_to_soil(&input).unwrap();
    let (input, soil_fert_map) = Almanac::soil_to_fertilizer(&input).unwrap();
    let (input, fert_water_map) = Almanac::fertilizer_to_water(&input).unwrap();
    let (input, water_light_map) = Almanac::water_to_light(&input).unwrap();
    let (input, light_temperature_map) = Almanac::light_to_temperature(&input).unwrap();
    let (input, temp_humid_map) = Almanac::temperature_to_humidity(&input).unwrap();
    let (input, humid_loc_map) = Almanac::humidity_to_location(&input).unwrap();

    let mut seed_itr = seed_list.iter();
    let mut next_seed = seed_itr.next();
    let mut seed_targets: Vec<(i64, i64)> = Vec::new();
    while next_seed != None {
        let start: i64 = next_seed.unwrap().parse().expect("failed to parse seed");
        next_seed = seed_itr.next();
        let increment: i64 = next_seed.unwrap().parse().expect("failed to parse seed");

        seed_targets.push((start, start + increment));
        next_seed = seed_itr.next();
    }

    println!("{seed_targets:?}");

    let mut location_start: i64 = 0;
    let mut seed = (MAX, MAX);
    'outer: loop {
        seed = location_traceseed(
            location_start,
            &seed_soil_map,
            &soil_fert_map,
            &fert_water_map,
            &water_light_map,
            &light_temperature_map,
            &temp_humid_map,
            &humid_loc_map,
        );

        for seed_range in &seed_targets {
            if seed.0 >= seed_range.0 && seed.0 < seed_range.1 {
                // found a seed in range
                break 'outer;
            }
        }

        location_start = location_start + 1;
    }

    println!("Found lowest seed location map: {seed:?}");
    seed
}

#[derive(Debug)]
struct AlmanacMap {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

struct Almanac {}

impl Almanac {
    pub fn seeds(i: &str) -> IResult<&str, Vec<&str>> {
        let seed_parser = pair(tag("seeds: "), separated_list0(multispace0, digit1));

        map(seed_parser, |(_, seed_ids)| Vec::from(seed_ids))(i)
    }

    pub fn seed_to_soil(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\nseed-to-soil map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }

    pub fn soil_to_fertilizer(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\nsoil-to-fertilizer map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }

    pub fn fertilizer_to_water(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\nfertilizer-to-water map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }

    pub fn water_to_light(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\nwater-to-light map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }

    pub fn light_to_temperature(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\nlight-to-temperature map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }

    pub fn temperature_to_humidity(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\ntemperature-to-humidity map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }

    pub fn humidity_to_location(i: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let seed_soil_parser = pair(
            tag::<&str, &str, Error<_>>("\n\nhumidity-to-location map:\n"),
            separated_list0(
                tag("\n"),
                tuple((digit1, multispace1, digit1, multispace1, digit1)),
            ),
        );

        let maps: Result<(&str, Vec<AlmanacMap>), nom::Err<_>> =
            map(seed_soil_parser, |(_, maps)| {
                maps.iter()
                    .map(
                        |(d_range_start, _, s_range_start, _, range_length)| AlmanacMap {
                            destination_range_start: d_range_start.parse().expect(""),
                            source_range_start: s_range_start.parse().expect(""),
                            range_length: range_length.parse().expect(""),
                        },
                    )
                    .collect()
            })(i);
        maps
    }
}

mod tests {
    use std::i64::MAX;

    use crate::day5::Almanac;

    use super::reverse_brute_force;

    #[test]
    fn test_total_points() {
        let input = create_test_input();

        let (input, seed_list) = Almanac::seeds(&input).unwrap();
        let (input, seed_soil_map) = Almanac::seed_to_soil(&input).unwrap();
        let (input, soil_fert_map) = Almanac::soil_to_fertilizer(&input).unwrap();
        let (input, fert_water_map) = Almanac::fertilizer_to_water(&input).unwrap();
        let (input, water_light_map) = Almanac::water_to_light(&input).unwrap();
        let (input, light_temperature_map) = Almanac::light_to_temperature(&input).unwrap();
        let (input, temp_humid_map) = Almanac::temperature_to_humidity(&input).unwrap();
        let (input, humid_loc_map) = Almanac::humidity_to_location(&input).unwrap();

        println!("{seed_list:?}");
        println!("{seed_soil_map:?}");
        println!("{soil_fert_map:?}");
        println!("{fert_water_map:?}");
        println!("{water_light_map:?}");
        println!("{light_temperature_map:?}");
        println!("{temp_humid_map:?}");
        println!("{humid_loc_map:?}");
        println!("{input:?}");

        let seed_location_list: Vec<(i64, i64)> = seed_list
            .iter()
            .map(|s| s.parse().expect("error parsing seed number"))
            .map(|seed: i64| {
                let mut soil = seed;
                for map in &seed_soil_map {
                    if seed >= map.source_range_start
                        && seed <= map.source_range_start + map.range_length
                    {
                        soil = (seed - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                let mut fert = soil;
                for map in &soil_fert_map {
                    if soil >= map.source_range_start
                        && soil <= map.source_range_start + map.range_length
                    {
                        fert = (soil - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                let mut water = fert;
                for map in &fert_water_map {
                    if fert >= map.source_range_start
                        && fert <= map.source_range_start + map.range_length
                    {
                        water = (fert - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                let mut light = water;
                for map in &water_light_map {
                    if water >= map.source_range_start
                        && water <= map.source_range_start + map.range_length
                    {
                        light = (water - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                let mut temperature = light;
                for map in &light_temperature_map {
                    if light >= map.source_range_start
                        && light <= map.source_range_start + map.range_length
                    {
                        temperature =
                            (light - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                let mut humidity = temperature;
                for map in &temp_humid_map {
                    if temperature >= map.source_range_start
                        && temperature <= map.source_range_start + map.range_length
                    {
                        humidity =
                            (temperature - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                let mut location = humidity;
                for map in &humid_loc_map {
                    if humidity >= map.source_range_start
                        && humidity <= map.source_range_start + map.range_length
                    {
                        location =
                            (humidity - map.source_range_start) + map.destination_range_start;
                        break;
                    }
                }

                (seed, location)
            })
            .collect();

        println!("{seed_location_list:?}");

        let mut min_loc: i64 = MAX;
        let mut min_seed: (i64, i64) = (0, 0);
        for (seed, location) in seed_location_list {
            if (location < min_loc) {
                min_loc = location;
                min_seed = (seed, location);
            }
        }

        println!("{min_seed:?}");
    }

    #[test]
    fn test_reverse_brute() {
        let input = create_test_input();
        let seed = reverse_brute_force(&input);
    }
    fn create_test_input() -> String {
        String::from(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        )
    }
}
