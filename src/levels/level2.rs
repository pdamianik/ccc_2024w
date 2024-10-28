use crate::input::{CountedInput, FromLines, Subtask};
use eyre::{eyre, WrapErr};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use std::str::Lines;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Room {
    width: usize,
    height: usize,
    desk_count: usize,
}

impl Subtask for Room {}

impl FromLines for Room {
    fn from_lines(lines: &mut Lines) -> Result<Self, eyre::Report> {
        let line = lines.next().ok_or(eyre!("No line available to parse"))?;
        let mut values = line.split(" ");
        let width = values.next().ok_or(eyre!("Missing width"))?;
        let height = values.next().ok_or(eyre!("Missing height"))?;
        let desk_count = values.next().ok_or(eyre!("Missing desk count"))?;

        Ok(Room {
            width: width.parse().wrap_err("Invalid width")?,
            height: height.parse().wrap_err("Invalid height")?,
            desk_count: desk_count.parse().wrap_err("Invalid desk count")?,
        })
    }
}

pub type Input = CountedInput<Room>;

pub fn map(room: &Room) -> eyre::Result<String> {
    let mut id = 1;
    let mut room_result = Vec::new();
    for _ in 0..room.height {
        let mut row = Vec::new();
        for _ in 0..room.width/3 {
            // build desk
            let desk = iter::repeat(id).take(3).join(" ");
            row.push(desk);
            id += 1;
        }
        room_result.push(row.join(" "));
    }

    let mut result = room_result.join("\r\n");
    result.push_str("\r\n");
    Ok(result)
}

pub fn reduce(results: Vec<String>) -> String {
    results.into_iter().join("\r\n")
}

pub fn verify(input: &Room, output: &str) -> eyre::Result<()> {
    let output_height = output.lines().count();
    if output_height != input.height {
        return Err(eyre!("Output does not match room height"));
    }

    let mut matrix = Vec::with_capacity(input.width * input.height);

    for row in output.lines() {
        let mut desk_ids= row.split(' ').map(|item|
                item.parse::<usize>()
                    .wrap_err("The desk id of a cell is not numeric")
            )
            .collect::<Result<Vec<_>, _>>()?;

        if desk_ids.len() != input.width {
            return Err(eyre!("Output contains {} desk ids for a {} wide room", desk_ids.len(), input.width));
        }

        matrix.append(&mut desk_ids);
    }

    let mut encountered_ids = HashSet::new();

    for row in (0..(input.height * input.width)).step_by(input.width) {
        'matrix: for col in 0..input.width - 2 {
            if matrix[row + col] == 0 {
                continue;
            }
            let id = matrix[row + col];

            let positions = (0..3)
                .map(|val| row + col + val);

            for position in positions.clone() {
                if matrix[position] != id {
                    continue 'matrix;
                }
            }

            for position in positions {
                matrix[position] = 0;
            }

            if encountered_ids.contains(&id) {
                return Err(eyre!("Encountered desk id {id} twice"));
            }
            encountered_ids.insert(id);
        }
    }

    for row in (0..((input.height - 2) * input.width)).step_by(input.width) {
        'matrix: for col in 0..input.width {
            if matrix[row + col] == 0 {
                continue;
            }
            let id = matrix[row + col];

            let positions = (0..3 * input.width)
                .step_by(input.width)
                .map(|val| row + col + val);

            for position in positions.clone() {
                if matrix[position] != id {
                    continue 'matrix;
                }
            }

            for position in positions {
                matrix[position] = 0;
            }

            if encountered_ids.contains(&id) {
                return Err(eyre!("Encountered desk id {id} twice"));
            }
            encountered_ids.insert(id);
        }
    }

    if let Some(id) = matrix.iter().find(|item| **item != 0) {
        return if encountered_ids.contains(id) {
            Err(eyre!("Desk {id} has more ids than it should"))
        } else {
            Err(eyre!("Invalid desk {id} encountered"))
        }
    }

    if encountered_ids.len() != input.desk_count {
        return Err(eyre!("Placed desk count {} does not match input desk count {}", encountered_ids.len(), input.desk_count));
    }

    Ok(())
}

#[cfg(test)]
pub fn split_example(input: &str) -> impl Iterator<Item = &str> {
    input.split("\r\n\r\n")
}
