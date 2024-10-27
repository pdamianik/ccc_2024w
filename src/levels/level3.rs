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

impl Room {
    pub fn generate_room_table_matrix(&self) -> String {
        let mut matrix = Vec::new();
        let mut table_id_counter = 1;

        let full_width = self.width / 3;
        let corner_table_count = self.width % 3;

        for _ in 0..self.height / 3 {
            let horizontal_tables = 3 * full_width;
            let corner_id = table_id_counter + horizontal_tables;
            let corner_tables = corner_id..(corner_id + corner_table_count);

            for _ in 0..3 {
                let mut matrix_row = Vec::new();

                for _ in 0..full_width {
                    // build table
                    let table = iter::repeat(table_id_counter).take(3).join(" ");
                    matrix_row.push(table);
                    table_id_counter += 1;
                }
                matrix_row.push(corner_tables.clone().into_iter().join(" "));
                matrix.push(matrix_row.join(" "));
            }
            table_id_counter += corner_table_count;
        }

        for _ in 0..self.height % 3 {
            let mut matrix_row = Vec::new();

            for _ in 0..self.width / 3 {
                matrix_row.push(iter::repeat(table_id_counter).take(3).join(" "));
                table_id_counter += 1;
            }
            matrix_row.push(iter::repeat(0).take(self.width % 3).join(" "));

            matrix.push(matrix_row.join(" "));
        }

        matrix.join("\r\n")
    }
}

pub fn run(input: &'static str) -> eyre::Result<String> {
    let input: Input = input.parse()?;

    let result = input.rooms.iter().map(|room| room.generate_room_table_matrix());

    Ok(result.into_iter().join("\r\n\r\n"))
}
