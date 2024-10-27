use crate::input::Input;
use eyre::{WrapErr, Report};

macro_rules! include_tasks {
    ($path:expr => [$($task_name:expr),*]) => {
        include_tasks!($path => [$($task_name,)*])
    };
    ($path:expr => [$($task_name:expr,)*]) => {
        [
        $(
            include_tasks!($path => $task_name),
        )*
        ]
    };
    ($path:expr => $task_name:expr) => {
            include_str!(concat!($path, "_", $task_name))
    };
}

macro_rules! include_inputs {
    ($level: literal => []) => {
        include_inputs!($level => ["1", "2", "3", "4", "5"])
    };
    ($level: literal => [$($task:literal),*]) => {
        include_inputs!($level => [$($task,)*])
    };
    ($level: literal => [$($task:literal,)*]) => {
        crate::input::Inputs {
            example_in: include_tasks!(concat!(::location_macros::workspace_dir!(), "/inputs/", $level) => "example.in"),
            example_out: include_tasks!(concat!(::location_macros::workspace_dir!(), "/inputs/", $level) => "example.out"),
            tasks: include_tasks!(concat!(::location_macros::workspace_dir!(), "/inputs/", $level) => [$(concat!($task, ".in"),)*]),
        }
    };
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[cfg(test)]
macro_rules! tests {
    ($level:ident($name:literal, $number:literal, [])) => {
        tests!($level($name, $number, ["1", "2", "3", "4", "5"]));
    };
    ($level:ident($name:literal, $number:literal, [$($task:literal),*])) => {
        tests!($level($name, $number, [$($task,)*]));
    };
    ($level:ident($name:literal, $number:literal, [$($task:literal,)*])) => {
        pub mod $level {
            $(
                ::concat_idents::concat_idents!(fn_name = test_, $level, _, $task {
                    #[test]
                    pub fn fn_name() {
                        // TODO: write test function
                    }
                });
            )*
        }
    };
}

macro_rules! levels {
    ($($level:ident($name:literal, $number:literal, [$($task:literal,)*]))*) => {
        $(
            #[cfg(feature = $name)]
            pub mod $level;

            #[cfg(feature = $name)]
            pub fn $level(input: &'static str) -> Result<String, Vec<::eyre::Report>> {
                let input: $level::Input = input.parse().map_err(|err| vec![err])?;

                let mut results = Vec::new();
                let mut errors = Vec::new();

                for input in input.subtasks() {
                    let result = $level::map(input)
                        .wrap_err("Failed to map input to output")
                        .and_then(|result| {
                            $level::verify(input, &result)
                                .map(|_| result)
                        })
                        .wrap_err("Verification failed");
                    match result {
                        Ok(result) => results.push(result),
                        Err(error) => errors.push(error.wrap_err(format!("Subtask {input:?} has failed"))),
                    }
                }

                if errors.len() == 0 {
                    Ok($level::reduce(results))
                } else {
                    Err(errors)
                }
            }

            #[cfg(feature = $name)]
            ::concat_idents::concat_idents!(fn_name = run_, $level {
                pub fn fn_name() -> Result<(), Vec<::eyre::Report>> {
                    let tasks = &RAW_INPUTS[$number - 1].tasks;

                    let output_dir = concat!(::location_macros::workspace_dir!(), "/out/", $name, "/");
                    std::fs::create_dir_all(output_dir)
                        .wrap_err("Cannot crate '{output_dir}' directory")
                        .map_err(|err| vec![err])?;

                    let mut errors = Vec::new();
                    for (n, raw_input) in tasks.iter().enumerate() {
                        let result = $level(raw_input)
                            .map_err(|errors| {
                                errors.into_iter()
                                    .map(|error| (error as Report).wrap_err(format!("Failed to run task {}", n + 1)))
                                    .collect()
                            });

                        let result = result.and_then(|output| {
                            let out = format!(concat!(::location_macros::workspace_dir!(), "/out/", $name, "/", $name, "_{}.out"), n + 1);
                            ::tracing::info!("Writing {out}");
                            std::fs::write(&out, output)
                                .wrap_err(format!("Cannot write to '{out}'"))
                                .map_err(|err| vec![err])
                        });

                        if let Err(mut e) = result {
                            errors.append(&mut e);
                        }
                    }

                    match errors.len() {
                        0 => Ok(()),
                        _ => Err(
                            errors.into_iter()
                                .map(|error| error.wrap_err(concat!("Failed to run ", $name)))
                                .collect()
                        ),
                    }
                }
            });
        )*

        const RAW_INPUTS: [crate::input::Inputs; count!($($level)*)] = [
            $(
                #[cfg(feature = $name)]
                include_inputs!($name => [$($task,)*]),
            )*
        ];

        #[cfg(test)]
        pub mod test {
            #[allow(unused_imports)]
            use super::*;

            $(
                tests!($level($name, $number, [$($task,)*]));
            )*
        }
    };
}

levels!(
    level1("level1", 1, [])
    // level2("level2", 2)
);

