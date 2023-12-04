mod common;
mod day1;
mod day2;
mod day3;
mod day4;
use std::{
    env,
    time::{self, Instant},
};
fn main() {
    let day = "4";
    let args: Vec<String> = env::args().collect();
    let day = if args.len() >= 2 { &args[1] } else { day };

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
        _ => {}
    }
}

#[cfg(test)]
mod tests {}
