use std::str::FromStr;

pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct Position {
    horizontal: i32,
    depth: i32, // increases with depth
    aim: i32,
}

impl Position {
    const ORIGIN: Position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    fn change_aim(&self, adjustment: i32) -> Position {
        Position {
            horizontal: self.horizontal,
            depth: self.depth,
            aim: self.aim + adjustment,
        }
    }
}

pub fn solve(commands: Vec<Command>) -> i32 {
    let final_position = commands
        .iter()
        .fold(Position::ORIGIN, |pos, command| match command {
            // increases horizontal position by X units
            // increases depth by your aim multiplied by X units
            Command::Forward(units) => Position {
                horizontal: pos.horizontal + *units as i32,
                depth: pos.depth + (pos.aim * *units as i32),
                aim: pos.aim,
            },

            // increases your aim by X units
            Command::Down(units) => pos.change_aim(*units as i32),

            // decreases aim by X units
            Command::Up(units) => pos.change_aim(-(*units as i32)),
        });

    final_position.horizontal * final_position.depth
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();

        fn parse_units(units: &str, input: &str) -> Result<u32, ParseCommandError> {
            units
                .parse()
                .map_err(|_e| ParseCommandError(String::from(input)))
        }

        match split.as_slice() {
            ["forward", units] => parse_units(units, s).map(|u| Command::Forward(u)),
            ["down", units] => parse_units(units, s).map(|u| Command::Down(u)),
            ["up", units] => parse_units(units, s).map(|u| Command::Up(u)),
            _ => Err(ParseCommandError(String::from(s))),
        }
    }
}

#[derive(Debug)]
pub struct ParseCommandError(String);
