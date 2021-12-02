use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct ParseCommandError(String);

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
