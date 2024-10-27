use std::{iter, str::FromStr};

use itertools::Itertools;

struct Input {
    count: usize,
    rooms: Vec<Room>,
}

impl FromStr for Input {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().ok_or(eyre::eyre!("No first count line"))?;
        let count = first_line.parse()?;
        let rooms = lines.map(Room::from_str).collect::<Result<Vec<_>, _>>()?;

        Ok(Input { count, rooms })
    }
}

struct Room {
    width: usize,
    height: usize,
    desk_count: usize,
}

impl FromStr for Room {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(" ");
        let width = values.next().ok_or(eyre::eyre!("Invalid width"))?;
        let height = values.next().ok_or(eyre::eyre!("Invalid height"))?;
        let desk_count = values.next().ok_or(eyre::eyre!("Invalid desk count"))?;

        Ok(Room {
            width: width.parse()?,
            height: height.parse()?,
            desk_count: desk_count.parse()?,
        })
    }
}

pub fn run(input: &'static str) -> eyre::Result<String> {
    let input: Input = input.parse()?;

    let mut result = Vec::new();

    for room in input.rooms {
        let mut id = 1;
        let mut room_result = String::new();
        for _ in 0..room.height {
            let mut row = Vec::new();
            for _ in 0..room.width/3 {
                // build table
                let table = iter::repeat(id).take(3).join(" ");
                row.push(table);
                id += 1;
            }
            room_result.push_str(&format!("{}\r\n", row.join(" ")));
        }

        result.push(room_result);
    }


    Ok(result.into_iter().join("\r\n"))
}
