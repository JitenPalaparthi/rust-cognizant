fn main() {
    let d1 = Direction::East;
    set_direction(d1);

    set_direction_str("east");
    set_direction_str("East");
    set_direction_str("abcd");
}

enum Direction {
    South = 1,
    North,
    East,
    West,
}

// enum Status {
//     active,
//     inactive,
// }

// enum Option<T>{
//     Some(T),
//     None,
// }

fn set_direction(direction: Direction) {
    match direction {
        Direction::East => println!("setup East direction:{} ", Direction::East as i16),
        Direction::West => println!("setup West direction"),
        Direction::South => println!("setup South direction"),
        Direction::North => println!("setup North direction"),
    }
}

fn set_direction_str(direction: &str) {
    match direction {
        "south" => {
            println!("setup South direction");
        }
        "north" => {
            println!("setup Noth direction");
        }
        "east" => {
            println!("setup East direction");
        }
        "west" => {
            println!("setup West direction");
        }
        _ => {
            println!("it is a undefined or wrong direction")
        }
    }
}
