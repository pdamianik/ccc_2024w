use std::fmt::Debug;
use std::str::{FromStr, Lines};
use eyre::{eyre, Context};

pub trait FromLines: Sized {
    fn from_lines(lines: &mut Lines<'_>) -> Result<Self, eyre::Report>;
}

#[allow(dead_code)]
pub struct Inputs {
    pub example_in: &'static str,
    pub example_out: &'static str,
    pub tasks: [&'static str; 5],
}

pub trait Input {
    type Subtask: Eq + PartialEq + Clone + Debug;

    fn subtasks(&self) -> impl Iterator<Item = &Self::Subtask>;
}

pub trait Subtask: Eq + PartialEq + Clone + Debug + FromLines {

}

#[allow(dead_code)]
pub struct CountedInput<TSubtask: Subtask> {
    count: usize,
    tasks: Vec<TSubtask>,
}

impl<TSubtask: Subtask> Input for CountedInput<TSubtask> {
    type Subtask = TSubtask;

    fn subtasks(&self) -> impl Iterator<Item=&Self::Subtask> {
        self.tasks.iter()
    }
}

impl<TSubtask: Subtask> FromStr for CountedInput<TSubtask> {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().ok_or(eyre!("No first count line"))?;
        let count = first_line.parse().context("Failed to parse count from first input line")?;
        let mut tasks = Vec::with_capacity(count);

        for _ in 0..count {
            tasks.push(TSubtask::from_lines(&mut lines)?);
        }

        if tasks.len() == count {
            Ok(CountedInput { count, tasks })
        } else {
            Err(eyre!("Parsed subtask count does not match given count"))
        }
    }
}

