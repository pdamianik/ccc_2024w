use std::{char::UNICODE_VERSION, iter, str::FromStr};

use color_eyre::owo_colors::OwoColorize;
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

const TABLE_WIDTH: usize = 2;

impl Room {
    pub fn generate_room_table_matrix(&self) -> eyre::Result<String> {
        let results = vec![self.generate_room_table_matrix_for_uneven_width()?, self.generate_room_table_matrix_for_uneven_height()?, self.generate_room_table_matrix_for_uneven_width()?, self.generate_room_table_matrix_for_even_height()?];
        let Some((result, number_of_placed_desks)) = results.iter().max_by(|a, b| a.1.cmp(&b.1)) else { unreachable!()};
        if *number_of_placed_desks != self.desk_count {
            return Err(eyre::eyre!(format!(
                "Not all desks were placed in the room ({} out of {}, room size {}x{}): \n{}",
                number_of_placed_desks, self.desk_count, self.width, self.height, result,
            )));
        }
        Ok(result.clone())
    }

    // fn generate_room_table_matrix_for_uneven_height_simple(&self) -> eyre::Result<(String, usize)> {
    //     let mut matrix = Vec::new();
    //     let mut number_of_placed_desks = 0;

    //     let line = iter::repeat("XX").take(self.width / 3).join(".");
    //     let vertical_rest = match self.width % 3 {
    //         1 => ".",
    //         2 => ".X",
    //     };

    //     let block = {
    //         let mut block = Vec::new();

    //         block.push(format!("{line}{vertical_rest}"));
    //         block.push(format!("{}{vertical_rest}", ".".repeat(self.width - vertical_rest.len())));
    //         block.push(format!("{line}{}", ".".repeat)
    //     };
    // }

    fn generate_room_table_matrix_for_even_width(&self) -> eyre::Result<(String, usize)> {
        let mut matrix = Vec::new();
        let mut number_of_placed_desks = 0;

        let block_height = 7;
        let block_count = self.height / block_height;
        let extra_rows = self.height % block_height;

        let block: Vec<String> = {
            let mut block = Vec::new();
            let horizontal_desks = (self.width - 2) / 2;
            number_of_placed_desks += horizontal_desks;
            let mut first_line = "X.".repeat((self.width - 2) / 2);
            first_line.push_str("XX");
            block.push(first_line);
            let mut second_line = "X.".repeat((self.width - 2) / 2);
            second_line.push_str("..");
            block.push(second_line);
            let mut last_line = ".".repeat(self.width - 2);
            number_of_placed_desks += 2;
            last_line.push_str("XX");
            block.push(last_line);
            block
        };

        let block_mirror: Vec<String> = block
            .iter()
            .rev()
            .map(|line| line.chars().rev().collect::<String>())
            .collect();

        for _ in 0..block_count {
            matrix.append(&mut block.clone());
            matrix.append(&mut block_mirror.clone());
            matrix.push(".".repeat(self.width));
        }

        let mut full_block = Vec::new();
        full_block.append(&mut block.clone());
        full_block.append(&mut block_mirror.clone());
        if extra_rows % 2 == 0 {
            matrix.extend_from_slice(&full_block[..extra_rows]);
            number_of_placed_desks += (self.width / 2) * if extra_rows == 6 { 2 } else { 1 };
            if extra_rows != 2 {
                number_of_placed_desks += 2;
            }
        } else if extra_rows == 1 {
            let mut row = String::new();

            row.push_str(&iter::repeat("XX.").take(self.width / 3).join(""));
            number_of_placed_desks += self.width / 3;
            row.push_str(&".".repeat(self.width % 3));
            matrix.push(row);
        } else {
            matrix.extend_from_slice(&full_block[..extra_rows - 1]);
            if extra_rows == 3 {
                number_of_placed_desks += (self.width / 2) + 1;
            } else {
                number_of_placed_desks += (self.width / 2) + 3;
            }
            let mut row = String::new();
            row.push_str(&".".repeat(self.width - 2));
            row.push_str("XX");
            matrix.push(row);
        }

        Ok((matrix.join("\r\n"), number_of_placed_desks))
    }

    fn generate_room_table_matrix_for_even_height(&self) -> eyre::Result<(String, usize)> {
        let width = self.height;
        let height = self.width;
        let mut matrix = Vec::new();
        let mut number_of_placed_desks = 0;

        let block_height = 7;
        let block_count = height / block_height;
        let extra_rows = height % block_height;

        let block: Vec<String> = {
            let mut block = Vec::new();
            let horizontal_desks = (width - 2) / 2;
            number_of_placed_desks += horizontal_desks;
            let mut first_line = "X.".repeat((width - 2) / 2);
            first_line.push_str("XX");
            block.push(first_line);
            let mut second_line = "X.".repeat((width - 2) / 2);
            second_line.push_str("..");
            block.push(second_line);
            let mut last_line = ".".repeat(width - 2);
            number_of_placed_desks += 2;
            last_line.push_str("XX");
            block.push(last_line);
            block
        };

        let block_mirror: Vec<String> = block
            .iter()
            .rev()
            .map(|line| line.chars().rev().collect::<String>())
            .collect();

        for _ in 0..block_count {
            matrix.append(&mut block.clone());
            matrix.append(&mut block_mirror.clone());
            matrix.push(".".repeat(width));
        }

        let mut full_block = Vec::new();
        full_block.append(&mut block.clone());
        full_block.append(&mut block_mirror.clone());
        if extra_rows % 2 == 0 {
            matrix.extend_from_slice(&full_block[..extra_rows]);
            number_of_placed_desks += (width / 2) * if extra_rows == 6 { 2 } else { 1 };
            if extra_rows != 2 {
                number_of_placed_desks += 2;
            }
        } else if extra_rows == 1 {
            let mut row = String::new();

            row.push_str(&iter::repeat("XX.").take(width / 3).join(""));
            number_of_placed_desks += width / 3;
            row.push_str(&".".repeat(width % 3));
            matrix.push(row);
        } else {
            matrix.extend_from_slice(&full_block[..extra_rows - 1]);
            if extra_rows == 3 {
                number_of_placed_desks += (width / 2) + 1;
            } else {
                number_of_placed_desks += (width / 2) + 3;
            }
            let mut row = String::new();
            row.push_str(&".".repeat(width - 2));
            row.push_str("XX");
            matrix.push(row);
        }

        let bytes = matrix
            .iter()
            .flat_map(|s| s.clone().into_bytes())
            .collect::<Vec<u8>>();
        let mut transposed: Vec<u8> = vec![0; bytes.len()];
        transpose::transpose(&bytes, &mut transposed, width, height);
        let mut matrix = transposed
            .chunks(height)
            .map(|chunk| String::from_utf8(chunk.to_owned()).unwrap())
            .collect::<Vec<String>>();
        matrix
            .iter_mut()
            .for_each(|string| *string = string.reversed().to_string());

        Ok((matrix.join("\r\n"), number_of_placed_desks))
    }
    fn generate_room_table_matrix_for_uneven_width(&self) -> eyre::Result<(String, usize)> {
        let mut matrix = Vec::new();
        let mut number_of_placed_desks = 0;
        let number_of_vertical_blocks = self.height / (TABLE_WIDTH + 1);

        let single_line = {
            let mut single_line = "X.".repeat(self.width / 2);
            if self.width % 2 == 1 {
                single_line.push_str("X");
            }
            single_line
        };

        let trimmed_block: Vec<String> = {
            let mut lines = Vec::new();

            for _ in 0..TABLE_WIDTH {
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
            number_of_placed_desks += self.width / TABLE_WIDTH + self.width % TABLE_WIDTH;
        }

        let rest_height = self.height % (TABLE_WIDTH + 1);

        if rest_height == 2 {
            matrix.append(&mut trimmed_block.clone());
            number_of_placed_desks += self.width / TABLE_WIDTH + self.width % TABLE_WIDTH;
        } else if rest_height > 0 {
            let mut row = String::new();
            let number_of_horizontal_blocks = self.width / (TABLE_WIDTH + 1);

            let trimmed_table = "XX";
            let table = "XX.";

            for _ in 0..number_of_horizontal_blocks {
                row.push_str(table);
                number_of_placed_desks += 1;
            }

            let rest_width = self.width - number_of_horizontal_blocks * (TABLE_WIDTH + 1);

            if rest_width == 2 {
                row.push_str(trimmed_table);
                number_of_placed_desks += 1;
            } else {
                row.push_str(&".".repeat(rest_width));
            }

            matrix.push(row);
        };

        Ok((matrix.join("\r\n"), number_of_placed_desks))
    }

    fn generate_room_table_matrix_for_uneven_height(&self) -> eyre::Result<(String, usize)> {
        let width = self.height;
        let height = self.width;
        let mut matrix = Vec::new();
        let mut number_of_placed_desks = 0;
        let number_of_vertical_blocks = height / (TABLE_WIDTH + 1);

        let single_line = {
            let mut single_line = "X.".repeat(width / 2);
            if width % 2 == 1 {
                single_line.push_str("X");
            }
            single_line
        };

        let trimmed_block: Vec<String> = {
            let mut lines = Vec::new();

            for _ in 0..TABLE_WIDTH {
                lines.push(single_line.clone());
            }

            lines
        };

        let block: Vec<String> = {
            let mut block = trimmed_block.clone();
            block.push(".".repeat(width));
            block
        };

        for _ in 0..number_of_vertical_blocks {
            matrix.append(&mut block.clone());
            number_of_placed_desks += width / TABLE_WIDTH + width % TABLE_WIDTH;
        }

        let rest_height = height % (TABLE_WIDTH + 1);

        if rest_height == 2 {
            matrix.append(&mut trimmed_block.clone());
            number_of_placed_desks += width / TABLE_WIDTH + width % TABLE_WIDTH;
        } else if rest_height > 0 {
            let mut row = String::new();
            let number_of_horizontal_blocks = width / (TABLE_WIDTH + 1);

            let trimmed_table = "XX";
            let table = "XX.";

            for _ in 0..number_of_horizontal_blocks {
                row.push_str(table);
                number_of_placed_desks += 1;
            }

            let rest_width = width - number_of_horizontal_blocks * (TABLE_WIDTH + 1);

            if rest_width == 2 {
                row.push_str(trimmed_table);
                number_of_placed_desks += 1;
            } else {
                row.push_str(&".".repeat(rest_width));
            }

            matrix.push(row);
        };

        let bytes = matrix
            .iter()
            .flat_map(|s| s.clone().into_bytes())
            .collect::<Vec<u8>>();
        let mut transposed: Vec<u8> = vec![0; bytes.len()];
        transpose::transpose(&bytes, &mut transposed, width, height);
        let mut matrix = transposed
            .chunks(height)
            .map(|chunk| String::from_utf8(chunk.to_owned()).unwrap())
            .collect::<Vec<String>>();
        matrix
            .iter_mut()
            .for_each(|string| *string = string.reversed().to_string());

        Ok((matrix.join("\r\n"), number_of_placed_desks))
    }
}

pub fn run(input: &'static str) -> eyre::Result<String> {
    let input: Input = input.parse()?;

    let mut result = input
        .rooms
        .iter()
        .enumerate()
        .map(|(i, room)| {
            room.generate_room_table_matrix().map_err(|e| {
                e.wrap_err(format!("Failed to generate desk matrix for room {}", i + 1))
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .join("\r\n\r\n");

    result.push_str("\r\n\r\n");

    Ok(result)
}
