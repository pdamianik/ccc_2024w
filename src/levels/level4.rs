use std::{char::UNICODE_VERSION, iter, str::FromStr};

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

const TABLE_WIDTH: usize = 4;
const TABLE_HEIGHT: usize = 2;

impl Room {
    pub fn generate_room_table_matrix(&self) -> eyre::Result<String> {
        let mut matrix = Vec::new();
        let number_of_vertical_blocks = self.height / 4;

        let single_line = {
            let mut single_line = "X.".repeat(self.width / 2);
            if self.width % 2 == 1 {
                single_line.push_str("X");
            }
            single_line
        };

        let trimmed_block: Vec<String> = {
            let mut lines = Vec::new();

            for _ in 0..3 {
                lines.push(single_line.clone());
            }

            lines
        };

        let block: Vec<String> = {
            let mut block = trimmed_block.clone();
            block.push(".".repeat(self.width));
            block
        };

        for _ in 0..number_of_vertical_blocks {
            matrix.append(&mut block.clone());
        }

        let rest_height = self.height % 4;

        if rest_height == 3 {
            matrix.append(&mut trimmed_block.clone());
        } else if rest_height > 0 {
            let mut row = String::new();
            let number_of_horizontal_blocks = self.width / 4;

            let trimmed_table = "XXX";
            let table = "XXX.";

            for _ in 0..number_of_horizontal_blocks {
                row.push_str(table);
            }

            let rest_width = self.width - number_of_horizontal_blocks * 4;

            if rest_width == 3 {
                row.push_str(trimmed_table);
            } else {
                row.push_str(&".".repeat(rest_width));
            }

            matrix.push(row);

            if rest_height == 2 {
                matrix.push(".".repeat(self.width));
            }
        };

        Ok(matrix.join("\r\n"))
    }
}

pub fn run(input: &'static str) -> eyre::Result<String> {
    let input: Input = input.parse()?;

    let mut result = input
        .rooms
        .iter()
        .map(|room| room.generate_room_table_matrix())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .join("\r\n\r\n");

    result.push_str("\r\n\r\n");

    Ok(result)
}
