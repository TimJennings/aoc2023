mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
use std::{env, time::Instant};
fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if args.len() >= 2 { &args[1] } else { "6" };

    println!("Running day {day}");

    match day {
        "1" => {
            day1::puz1();
            day1::puz2();
        }
        "2" => {
            day2::puz1();
            day2::puz2();
        }
        "3" => {
            day3::puz1();
            day3::puz2();
        }
        "4" => {
            //day4::puz1();
            let now = Instant::now();
            day4::puz2();
            let elapsed_time = now.elapsed();
            println!(
                "Running slow_function() took {} milliseconds.",
                elapsed_time.as_millis()
            );
        }
        "5" => {
            let now = Instant::now();
            day5::puz2();
            let elapsed_time = now.elapsed();
            println!(
                "Running slow_function() took {} milliseconds.",
                elapsed_time.as_millis()
            );
        }
        "6" => {
            day6::puz1();
            day6::puz2();
        }
        "7" => {
            day7::puz1();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {}
