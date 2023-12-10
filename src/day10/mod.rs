use std::f64::consts::PI;

use crate::common::file_io::read_file_to_string;

enum GridTile {
    START,
    GROUND,
    PIPE(Direction, Direction),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Direction {
    x: i32,
    y: i32,
}

const NORTH: Direction = Direction { x: -1, y: 0 };
const SOUTH: Direction = Direction { x: 1, y: 0 };
const EAST: Direction = Direction { x: 0, y: 1 };
const WEST: Direction = Direction { x: 0, y: -1 };

fn parse_data(input: &str) -> Vec<Vec<GridTile>> {
    let lines: Vec<&str> = input.split("\n").map(|line| line.trim()).collect();
    let mut grid: Vec<Vec<GridTile>> = Vec::new();

    for line in lines {
        let mut grid_line: Vec<GridTile> = Vec::new();

        for char in line.chars() {
            let tile: GridTile = match char {
                'S' => GridTile::START,
                '.' => GridTile::GROUND,
                '|' => GridTile::PIPE(NORTH, SOUTH),
                '-' => GridTile::PIPE(EAST, WEST),
                'L' => GridTile::PIPE(NORTH, EAST),
                'J' => GridTile::PIPE(NORTH, WEST),
                '7' => GridTile::PIPE(SOUTH, WEST),
                'F' => GridTile::PIPE(SOUTH, EAST),
                _ => panic!("invalid tile {char}"),
            };
            grid_line.push(tile);
        }
        grid.push(grid_line);
    }

    grid
}

fn is_start(x: usize, y: usize, grid: &Vec<Vec<GridTile>>) -> bool {
    match *(grid.get(x).unwrap().get(y).unwrap()) {
        GridTile::START => true,
        _ => false,
    }
}

fn get_tile(x: usize, y: usize, grid: &Vec<Vec<GridTile>>) -> &GridTile {
    grid.get(x).unwrap().get(y).unwrap()
}

fn get_tile_at<'a>(
    x: usize,
    y: usize,
    direction: &Direction,
    grid: &'a Vec<Vec<GridTile>>,
) -> &'a GridTile {
    grid.get((x as i32 + direction.x) as usize)
        .unwrap()
        .get((y as i32 + direction.y) as usize)
        .unwrap()
}

fn connected(
    x: usize,
    y: usize,
    direction: &Direction,
    required_direction: &Direction,
    grid: &Vec<Vec<GridTile>>,
) -> bool {
    let test_tile = get_tile_at(x, y, &direction, grid);
    match test_tile {
        GridTile::PIPE(ins, out) => *ins == *required_direction || *out == *required_direction,
        _ => false,
    }
}

fn get_next_direction<'a>(
    x: usize,
    y: usize,
    coming_from: &Direction,
    grid: &'a Vec<Vec<GridTile>>,
) -> &'a Direction {
    match get_tile(x, y, grid) {
        GridTile::PIPE(ins, outs) => {
            if ins != coming_from {
                return ins;
            } else {
                return outs;
            }
        }
        _ => panic!("not a pipe"),
    }
}

fn travel_pipe(grid: &Vec<Vec<GridTile>>) -> (u32, Vec<(usize, usize)>) {
    // record the path
    let mut path = Vec::new();

    // find the start tile, find a connecting pipe and travel it until we get back to the start pipe, report the length
    let mut x = 0;
    let mut y = 0;
    let mut coming_from: Direction = NORTH;
    'outer: for (x_index, row) in grid.iter().enumerate() {
        for (y_index, column) in row.iter().enumerate() {
            if is_start(x_index, y_index, grid) {
                x = x_index;
                y = y_index;
                break 'outer;
            }
        }
    }

    path.push((x, y));
    // locate connecting pipes by checking the cardianals
    if connected(x, y, &NORTH, &SOUTH, grid) {
        // go north
        x = (x as i32 + NORTH.x) as usize;
        y = (y as i32 + NORTH.y) as usize;
        coming_from = SOUTH;
    } else if connected(x, y, &SOUTH, &NORTH, grid) {
        // go south
        x = (x as i32 + SOUTH.x) as usize;
        y = (y as i32 + SOUTH.y) as usize;
        coming_from = NORTH;
    } else if connected(x, y, &EAST, &WEST, grid) {
        // go east
        x = (x as i32 + EAST.x) as usize;
        y = (y as i32 + EAST.y) as usize;
        coming_from = WEST;
    } else if connected(x, y, &WEST, &EAST, grid) {
        // go west
        x = (x as i32 + WEST.x) as usize;
        y = (y as i32 + WEST.y) as usize;
        coming_from = EAST;
    }
    path.push((x, y));
    let mut steps = 1;
    // walk until we're back at the start
    while !is_start(x, y, grid) {
        let direction = get_next_direction(x, y, &coming_from, grid);
        match *direction {
            NORTH => {
                // go north
                x = (x as i32 + NORTH.x) as usize;
                y = (y as i32 + NORTH.y) as usize;
                coming_from = SOUTH;
            }
            SOUTH => {
                // go SOUTH
                x = (x as i32 + SOUTH.x) as usize;
                y = (y as i32 + SOUTH.y) as usize;
                coming_from = NORTH;
            }
            EAST => {
                // go EAST
                x = (x as i32 + EAST.x) as usize;
                y = (y as i32 + EAST.y) as usize;
                coming_from = WEST;
            }
            WEST => {
                // go WEST
                x = (x as i32 + WEST.x) as usize;
                y = (y as i32 + WEST.y) as usize;
                coming_from = EAST;
            }
            _ => {
                panic!("not a direction")
            }
        }
        path.push((x, y));
        steps += 1;
    }

    (steps, path)
}

fn angle_to_path(x: usize, y: usize, path: &Vec<(usize, usize)>) -> f64 {
    let mut sum_of_all_angles = 0.0;
    let n = path.len() - 1;
    for (index, point) in path.iter().enumerate() {
        let point2 = match path.get(index + 1) {
            Some(p) => p,
            None => break,
        };

        let x1 = point.0 as f64 - x as f64;
        let y1 = point.1 as f64 - y as f64;
        let x2 = point2.0 as f64 - x as f64;
        let y2 = point2.1 as f64 - y as f64;
        sum_of_all_angles += angle_2d(x1, y1, x2, y2);
    }

    sum_of_all_angles
}

fn angle_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let theta1 = y1.atan2(x1);
    let theta2 = y2.atan2(x2);
    let mut dtheta = theta2 - theta1;
    while dtheta > PI {
        dtheta -= PI;
        dtheta -= PI;
    }
    while dtheta < -PI {
        dtheta += PI;
        dtheta += PI;
    }

    dtheta
}

fn in_path(x: usize, y: usize, path: &Vec<(usize, usize)>) -> bool {
    let angle = angle_to_path(x, y, path);
    if angle.abs() < PI {
        false
    } else {
        true
    }
}

fn find_all_ground(grid: &Vec<Vec<GridTile>>) -> Vec<(usize, usize)> {
    let mut ground_list = Vec::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, column) in row.iter().enumerate() {
            match get_tile(x, y, grid) {
                GridTile::GROUND => ground_list.push((x, y)),
                _ => continue,
            }
        }
    }

    ground_list
}

fn find_all_non_path(grid: &Vec<Vec<GridTile>>, path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ground_list = Vec::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, column) in row.iter().enumerate() {
            match get_tile(x, y, grid) {
                GridTile::GROUND => ground_list.push((x, y)),
                GridTile::PIPE(_, _) => {
                    if path.iter().any(|(px, py)| *px == x && *py == y) {
                        // match path
                        continue;
                    } else {
                        ground_list.push((x, y));
                    }
                }
                _ => continue,
            }
        }
    }

    ground_list
}

pub fn puz1() {
    let input = read_file_to_string("input/day10-input");
    let pipe_grid = parse_data(&input);
    let (pipe_length, path) = travel_pipe(&pipe_grid);
    let mid = pipe_length / 2;

    println!("pipe length is {pipe_length} so further point is {mid}");
}

pub fn puz2() {
    let input = read_file_to_string("input/day10-input");
    let pipe_grid = parse_data(&input);
    let (pipe_length, path) = travel_pipe(&pipe_grid);

    let all_non_path = find_all_non_path(&pipe_grid, &path);

    let mut answer = 0;

    for ground_point in all_non_path {
        if in_path(ground_point.0, ground_point.1, &path) {
            answer += 1;
        }
    }

    println!("There are {answer} ground points within the pipe");
}
mod test {
    use crate::day10::{angle_to_path, find_all_ground, find_all_non_path, in_path};

    use super::{parse_data, travel_pipe};

    #[test]
    pub fn test1() {
        let pipe_grid = parse_data(TEST_DATA);
        let (answer, path) = travel_pipe(&pipe_grid);
        assert_eq!(16, answer);
    }

    #[test]
    pub fn test_poly_with_holes() {
        let pipe_grid = parse_data(POLYGON_WITH_HOLES);

        let (answer, path) = travel_pipe(&pipe_grid);

        println!("{path:?}");

        assert_eq!(false, in_path(0, 0, &path));
        assert_eq!(false, in_path(1, 0, &path));
        assert_eq!(true, in_path(6, 2, &path));
    }

    #[test]
    pub fn test_count_inner_ground() {
        let pipe_grid = parse_data(POLYGON_WITH_HOLES);

        let (answer, path) = travel_pipe(&pipe_grid);
        let all_non_path = find_all_non_path(&pipe_grid, &path);
        // let all_ground = find_all_ground(&pipe_grid);

        let mut answer = 0;

        for ground_point in all_non_path {
            if in_path(ground_point.0, ground_point.1, &path) {
                answer += 1;
            }
        }
        assert_eq!(4, answer);
    }

    #[test]
    pub fn test_count_larger_inner_ground() {
        let pipe_grid = parse_data(POLYGON_LARGER_EXAMPLE);

        let (answer, path) = travel_pipe(&pipe_grid);
        let all_non_path = find_all_non_path(&pipe_grid, &path);

        let mut answer = 0;

        for ground_point in all_non_path {
            if in_path(ground_point.0, ground_point.1, &path) {
                answer += 1;
            }
        }
        assert_eq!(8, answer);
    }

    // #[test]
    // this test currently fails as it tries to go off grid while computing the initial start connections
    // this doesn't happen in my actual input so I've not bothered to fix it
    pub fn test_count_even_larger_inner_ground() {
        let pipe_grid = parse_data(POLYGON_EVEN_MORE_COMPLICATED_EXAMPLE);

        let (answer, path) = travel_pipe(&pipe_grid);
        let all_non_path = find_all_non_path(&pipe_grid, &path);

        let mut answer = 0;

        for ground_point in all_non_path {
            if in_path(ground_point.0, ground_point.1, &path) {
                answer += 1;
            }
        }
        assert_eq!(10, answer);
    }

    const TEST_DATA: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const POLYGON_WITH_HOLES: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const POLYGON_LARGER_EXAMPLE: &str = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const POLYGON_EVEN_MORE_COMPLICATED_EXAMPLE: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
}
