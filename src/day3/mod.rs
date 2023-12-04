use std::{char, collections::HashMap};

use crate::common::file_io::read_file_to_vec;

#[derive(Debug)]
enum GridElement {
    NUMBER(char),
    SYMBOL(char),
    BLANK,
}

impl GridElement {
    pub fn from_char(c: char) -> GridElement {
        match c {
            '.' => GridElement::BLANK,
            '\n' => GridElement::BLANK,
            '0'..='9' => GridElement::NUMBER(c),
            _ => GridElement::SYMBOL(c),
        }
    }
}

struct Grid {
    grid: Vec<Vec<GridElement>>,
}

impl Grid {
    fn is_symbol(&self, row: i32, column: i32) -> bool {
        // clamp the values
        if row < 0 {
            return false;
        }
        if column < 0 {
            return false;
        }

        let row: usize = row as usize;
        let column: usize = column as usize;

        if row >= self.grid.len() || column >= self.grid[row].len() {
            false
        } else {
            matches!(self.grid[row][column], GridElement::SYMBOL(_))
        }
    }

    fn is_gear(&self, row: i32, column: i32) -> bool {
        // clamp the values
        if row < 0 {
            return false;
        }
        if column < 0 {
            return false;
        }

        let row: usize = row as usize;
        let column: usize = column as usize;

        if row >= self.grid.len() || column >= self.grid[row].len() {
            false
        } else {
            matches!(self.grid[row][column], GridElement::SYMBOL('*'))
        }
    }

    fn is_adjacent(&self, row: usize, column: usize) -> bool {
        let row: i32 = row as i32;
        let column: i32 = column as i32;
        self.is_symbol(row - 1, column - 1)
            || self.is_symbol(row - 1, column)
            || self.is_symbol(row - 1, column + 1)
            || self.is_symbol(row, column - 1)
            // || self.is_symbol(row, column)
            || self.is_symbol(row, column + 1)
            || self.is_symbol(row + 1, column - 1)
            || self.is_symbol(row + 1, column)
            || self.is_symbol(row + 1, column + 1)
    }

    fn is_adjacent_to_gear(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        let row: i32 = row as i32;
        let column: i32 = column as i32;
        let check_grid = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut gear_locs: Vec<(usize, usize)> = Vec::new();
        for grid_coord in check_grid {
            if self.is_gear(row + grid_coord.0, column + grid_coord.1) {
                gear_locs.push((
                    (row + grid_coord.0) as usize,
                    (column + grid_coord.1) as usize,
                ));
            }
        }
        gear_locs
    }

    pub fn total_gear_ratio(&self) -> u32 {
        let mut parser_row_index = 0;
        let mut parser_column_index = 0;

        let mut in_number = false;
        let mut current_number = String::new();

        let mut total_gear_ratio: u32 = 0;

        let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        let mut current_number_gear_list = Vec::new();

        while parser_row_index < self.grid.len() {
            while parser_column_index < self.grid[parser_row_index].len() {
                let item = &self.grid[parser_row_index][parser_column_index];

                if in_number {
                    // if we're in a number and this is a number add it on
                    if let GridElement::NUMBER(n) = item {
                        current_number.push(*n);
                        in_number = true;
                        // check adj to gears and record
                        let gears = self.is_adjacent_to_gear(parser_row_index, parser_column_index);
                        for gear in gears {
                            if !current_number_gear_list.contains(&gear) {
                                current_number_gear_list.push(gear);
                            }
                        }
                    } else {
                        // compute full number
                        let number = current_number
                            .parse::<u32>()
                            .expect("Failed to parse number");

                        // record this number against all the gears we came across
                        for gear in &current_number_gear_list {
                            if gear_map.contains_key(&gear) {
                                let gear_list =
                                    gear_map.get_mut(&gear).expect("failed to get gear list");
                                gear_list.push(number);
                            } else {
                                let mut gear_list: Vec<u32> = Vec::new();
                                gear_list.push(number);
                                gear_map.insert(*gear, gear_list);
                            }
                        }

                        // no longer in a number
                        in_number = false;
                        current_number.clear();
                        current_number_gear_list.clear();
                    }
                    // check for adjacent symbols
                } else {
                    // start of number?
                    if let GridElement::NUMBER(n) = item {
                        current_number.push(*n);
                        in_number = true;
                        // check adj to gears and record
                        let gears = self.is_adjacent_to_gear(parser_row_index, parser_column_index);
                        for gear in gears {
                            if !current_number_gear_list.contains(&gear) {
                                current_number_gear_list.push(gear);
                            }
                        }
                    }
                }

                parser_column_index = parser_column_index + 1;
            }

            // if we're in a number at this point perform the end of line calc
            if in_number {
                // compute full number
                let number = current_number
                    .parse::<u32>()
                    .expect("Failed to parse number");

                // record this number against all the gears we came across
                for gear in &current_number_gear_list {
                    if gear_map.contains_key(&gear) {
                        let gear_list = gear_map.get_mut(&gear).expect("failed to get gear list");
                        gear_list.push(number);
                    } else {
                        let mut gear_list: Vec<u32> = Vec::new();
                        gear_list.push(number);
                        gear_map.insert(*gear, gear_list);
                    }
                }

                // no longer in a number
                in_number = false;
                current_number.clear();
                current_number_gear_list.clear();
            }

            parser_row_index = parser_row_index + 1;
            parser_column_index = 0;
            current_number.clear();
            in_number = false;
            current_number_gear_list.clear();
        }

        // having iterated we can go through the gear map to find all gears that were connected to exactly 2 numbers
        // multiply those together and sum for the gear ratio
        for (gear, number_list) in gear_map.iter() {
            println!("{gear:?} adjacent to numbers {number_list:?}");
            if number_list.len() == 2 {
                total_gear_ratio = total_gear_ratio + (number_list[0] * number_list[1]);
            }
        }

        total_gear_ratio
    }

    pub fn total_parts(&self) -> u32 {
        // when finding a symbol check adjasent locations for numbers, on finding a number
        // scan left and right to find the start and end of the number then parse that slice

        let mut parser_row_index = 0;
        let mut parser_column_index = 0;

        let mut in_number = false;
        let mut current_number = String::new();
        let mut current_number_symbol_adj = false;

        let mut total_parts: u32 = 0;

        while parser_row_index < self.grid.len() {
            while parser_column_index < self.grid[parser_row_index].len() {
                let item = &self.grid[parser_row_index][parser_column_index];

                if in_number {
                    // if we're in a number and this is a number add it on
                    if let GridElement::NUMBER(n) = item {
                        current_number.push(*n);
                        in_number = true;
                        // check adj to symbol
                        if self.is_adjacent(parser_row_index, parser_column_index) {
                            current_number_symbol_adj = true;
                        }
                    } else {
                        // compute full number

                        // if we were adjancent to a symbol at any point add to total
                        println!("{current_number} adj {current_number_symbol_adj}");
                        if current_number_symbol_adj {
                            total_parts = total_parts
                                + current_number
                                    .parse::<u32>()
                                    .expect("Failed to parse number");
                        }

                        // no longer in a number
                        in_number = false;
                        current_number.clear();
                        current_number_symbol_adj = false;
                    }
                    // check for adjacent symbols
                } else {
                    // start of number?
                    if let GridElement::NUMBER(n) = item {
                        current_number.push(*n);
                        in_number = true;
                        // check adj to symbol
                        if self.is_adjacent(parser_row_index, parser_column_index) {
                            current_number_symbol_adj = true;
                        }
                    }
                }

                parser_column_index = parser_column_index + 1;
            }

            // if we're in a number at this point perform the end of line calc
            if in_number {
                // compute full number

                // if we were adjancent to a symbol at any point add to total
                println!("{current_number} adj {current_number_symbol_adj}");
                if current_number_symbol_adj {
                    total_parts = total_parts
                        + current_number
                            .parse::<u32>()
                            .expect("Failed to parse number");
                }
            }

            parser_row_index = parser_row_index + 1;
            parser_column_index = 0;
            current_number.clear();
            in_number = false;
            current_number_symbol_adj = false;
        }
        total_parts
    }
}

pub fn puz1() {
    let grid = read_file_to_vec("input/day3-input");

    // scan vector for symbols (symbols are not numbers and not periods)
    let grid = parse_grid(grid);
    let sum = grid.total_parts();
    println!("Puz1: Sum of all parts {sum}");
}

pub fn puz2() {
    let grid = read_file_to_vec("input/day3-input");

    let grid = parse_grid(grid);
    let sum = grid.total_gear_ratio();
    println!("Puz1: Sum of all gear ratios {sum}");
}

fn parse_grid(grid: Vec<String>) -> Grid {
    let mut inner_grid: Vec<Vec<GridElement>> = Vec::new();

    for line in grid {
        // iterate the line and parse out the grid elements
        let grid_line: Vec<GridElement> = line
            .trim()
            .chars()
            .map(|c| GridElement::from_char(c))
            .collect();
        inner_grid.push(grid_line);
    }

    Grid { grid: inner_grid }
}

mod tests {
    use crate::common::file_io::read_file_to_vec;

    use super::parse_grid;

    #[test]
    fn test_total_parts() {
        let test_grid: Vec<String> = create_test_input();

        let grid = parse_grid(test_grid);
        let total = grid.total_parts();

        assert_eq!(4361, total);
    }

    #[test]
    fn test_total_parts_full() {
        let test_grid: Vec<String> = read_file_to_vec("input/day3-input");

        let grid = parse_grid(test_grid);
        let total = grid.total_parts();

        assert_eq!(538046, total);
    }

    #[test]
    fn test_gear_ratio() {
        let test_grid: Vec<String> = create_test_input();

        let grid = parse_grid(test_grid);
        let total = grid.total_gear_ratio();
        assert_eq!(467835, total)
    }

    #[test]
    fn test_gear_ratio_reddit() {
        let test_grid = vec![
            String::from("12.......*.."),
            String::from("+.........34"),
            String::from(".......-12.."),
            String::from("..78......11"),
            String::from("..*....60..."),
            String::from("78.........9"),
            String::from(".5.....23..$"),
            String::from("8...90*12..."),
            String::from("............"),
            String::from("2.2......12."),
            String::from(".*.........*"),
            String::from("1.1..503+.56"),
        ];

        let grid = parse_grid(test_grid);
        let total = grid.total_gear_ratio();

        assert_eq!(6756, total);
    }

    #[test]
    fn test_reddit_example() {
        let test_grid = vec![
            String::from("12.......*.."),
            String::from("+.........34"),
            String::from(".......-12.."),
            String::from("..78......11"),
            String::from("..*....60..."),
            String::from("78.........9"),
            String::from(".5.....23..$"),
            String::from("8...90*12..."),
            String::from("............"),
            String::from("2.2......12."),
            String::from(".*.........*"),
            String::from("1.1..503+.56"),
        ];

        let grid = parse_grid(test_grid);
        let total = grid.total_parts();

        assert_eq!(925, total);
    }

    fn create_test_input() -> Vec<String> {
        vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ]
    }
}
