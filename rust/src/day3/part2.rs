use std::collections::HashMap;

#[derive(Debug)]
struct CurrentLocation {
    x: i32,
    y: i32,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn parse_input_char(c: char) -> Direction {
    match c {
        '^' => Direction::UP,
        'v' => Direction::DOWN,
        '<' => Direction::LEFT,
        '>' => Direction::RIGHT,
        _ => panic!("Invalid character")
    }
}

fn move_direction(current_location: &mut CurrentLocation, direction: &Direction) {
    match direction {
        Direction::UP => current_location.y += 1,
        Direction::DOWN => current_location.y -= 1,
        Direction::LEFT => current_location.x -= 1,
        Direction::RIGHT => current_location.x += 1,
    }
}

pub fn run(directions_raw: &String) {
    let directions: Vec<Direction> = directions_raw.chars().map(parse_input_char).collect();

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    grid.insert((0, 0), 2);
    let mut santa_current_location = CurrentLocation { x: 0, y: 0 };
    let mut robot_santa_current_location = CurrentLocation { x: 0, y: 0 };

    directions.iter().enumerate().for_each(|(i, x)| {
        let mut location_to_mutate = if i % 2 == 0 { &mut santa_current_location } else { &mut robot_santa_current_location };
        move_direction(&mut location_to_mutate, x);
        grid.entry((location_to_mutate.x, location_to_mutate.y)).and_modify(|e| *e += 1).or_insert(1);
    });

    let total: i32 = grid.iter().count() as i32;
    println!("Answer part 2: {}", total);
}
