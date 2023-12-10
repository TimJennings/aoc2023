use crate::common::file_io::read_file_to_vec;

pub fn puz1() {
    let lines = read_file_to_vec("input/day9-input");
    let lines = read_data(lines);

    let mut answer = 0;

    for line in lines {
        answer += predict_next_number(&line);
    }

    println!("sum of all next numbers is {answer}");
}

pub fn puz2() {
    let lines = read_file_to_vec("input/day9-input");
    let mut lines = read_data(lines);

    let mut answer = 0;

    for mut line in lines {
        line.reverse();
        answer += predict_next_number(&line);
    }

    println!("sum of all first numbers is {answer}");
}

fn predict_next_number(input: &Vec<i32>) -> i32 {
    println!("-----------");
    println!("{input:?}");
    let next_increment = predict_next_num_inner(input);

    input[input.len() - 1] + next_increment
}

fn predict_next_num_inner(input: &Vec<i32>) -> i32 {
    if input.iter().all(|n| *n == 0) {
        0
    } else {
        // construct the next line and call recusively
        let mut next_line = Vec::new();
        for (index, num) in input.iter().enumerate() {
            if index < input.len() - 1 {
                // combine with next index
                next_line.push(input[index + 1] - num);
            }
        }
        // println!("{next_line:?}");
        println!("{next_line:?}");
        let next_increment = predict_next_num_inner(&next_line);
        let last_num = next_line[next_line.len() - 1];
        next_line.push(last_num + next_increment);
        next_line[next_line.len() - 1]
    }
}

fn read_data(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in input {
        let nums: Vec<i32> = line
            .split(" ")
            .map(|l| l.trim())
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        result.push(nums);
    }

    result
}

mod test {
    use crate::day9::{predict_next_number, read_data};

    #[test]
    pub fn test1() {
        let lines = TEST_DATA.split("\n").map(|s| s.to_string()).collect();
        let lines = read_data(lines);

        assert_eq!(18, predict_next_number(&lines[0]));
        assert_eq!(28, predict_next_number(&lines[1]));
        assert_eq!(68, predict_next_number(&lines[2]));
    }

    #[test]
    pub fn testNeg() {
        let lines = vec![String::from(
            "14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 -1 -2 -3 -4 -5 -6",
        )];
        let lines = read_data(lines);

        assert_eq!(-7, predict_next_number(&lines[0]));
    }
    const TEST_DATA: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
}
