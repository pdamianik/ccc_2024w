use std::str::Lines;
use eyre::{eyre, Context};
use itertools::Itertools;
use crate::input::{CountedInput, FromLines, Subtask};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Room {
    width: usize,
    height: usize,
}

impl Subtask for Room { }

impl FromLines for Room {
    fn from_lines(lines: &mut Lines) -> eyre::Result<Self> {
        let s = lines.next().ok_or(eyre!("No line available to parse"))?;

        let (width, height) = s.split(" ").next_tuple().ok_or(eyre!("Room line does not contain a tuple"))?;

        Ok(Room {
            width: width.parse().wrap_err("Failed to parse room width")?,
            height: height.parse().wrap_err("Failed to parse room height")?,
        })
    }
}

pub type Input = CountedInput<Room>;

pub fn map(room: &Room) -> eyre::Result<String> {
    Ok((room.width / 3 * room.height).to_string())
}

pub fn reduce(results: Vec<String>) -> String {
    let mut result = results.join("\r\n");

    result.push_str("\r\n");

    result
}

pub fn verify(_input: &Room, output: &str) -> eyre::Result<()> {
    output.parse::<usize>()
        .map(|_| ())
        .map_err(|_| eyre!("Output is not an usize"))
}
