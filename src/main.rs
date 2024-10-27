use color_eyre::Help;
use eyre::Context;

mod input;
mod levels;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    std::fs::create_dir_all("out").wrap_err("Can not create 'out' directory")?;

    let result = levels::run_level1();

    result.map_err(|errors| {
        let error_count = errors.len();
        let first_error = errors.into_iter().next().unwrap();
        first_error.section(format!("And {} more errors", error_count - 1))
    })
}

// mod test {
//     use color_eyre::owo_colors::{AnsiColors, OwoColorize};
//     use color_eyre::Help;
//     use eyre::WrapErr;
//
//     use crate::CURRENT_LEVEL_NUMBER;
//     use crate::LEVELS;
//
//     #[test]
//     fn run_current_example() -> eyre::Result<()> {
//         color_eyre::install()?;
//         let level = &LEVELS[CURRENT_LEVEL_NUMBER - 1];
//
//         let examples_result = (level.run)(level.inputs.example_in).wrap_err(format!(
//             "Encountered error when running example for level {CURRENT_LEVEL_NUMBER}"
//         ))?;
//
//         if !examples_result.eq(level.inputs.example_out) {
//             return Err(eyre::eyre!(format!(
//                 "Example run for level {CURRENT_LEVEL_NUMBER} produced wrong output"
//             ))
//             .with_section(|| {
//                 let mut section = String::new();
//                 section.push_str(&"Expected:".color(AnsiColors::Green).to_string());
//                 section.push_str("\n");
//                 section.push_str(level.inputs.example_out);
//
//                 section
//             }))
//             .with_section(|| {
//                 let mut section = String::new();
//                 section.push_str(&"Got:".color(AnsiColors::Red).to_string());
//                 section.push_str("\n");
//                 section.push_str(&examples_result);
//
//                 section
//             })
//             .with_section(|| {
//                 let mut section = String::new();
//                 section.push_str(&"Diff:".color(AnsiColors::Blue).to_string());
//                 section.push_str("\n");
//                 section.push_str(&format!("{}", Diff { expected: level.inputs.example_out, actual: &examples_result}));
//
//                 section
//             });
//         }
//
//         Ok(())
//     }
// }
