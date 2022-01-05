enum Action {
    Drive,
    Turn(Direction),
    Stop
}

enum Direction {
    Left,
    Right
}

fn print_action (action: Action) {
    match action {
        Action::Drive => println!("Drive."),
        Action::Stop => println!("Stop."),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn left"),
            Direction::Right => println!("Turn right")
        }
    }
}

fn main () {
    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Stop
    ];

    for action in program {
        print_action(action);
    }
}
