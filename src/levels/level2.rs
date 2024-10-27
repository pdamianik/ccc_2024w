use crate::input::{CountedInput, FromLines, Subtask};
use eyre::{eyre, WrapErr};
use itertools::Itertools;
use std::str::Lines;
use std::iter;

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
            // build table
            let table = iter::repeat(id).take(3).join(" ");
            row.push(table);
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
    Ok(())
}

#[cfg(test)]
pub fn split_example(input: &str) -> impl Iterator<Item = &str> {
    input.split("\r\n\r\n")
}
