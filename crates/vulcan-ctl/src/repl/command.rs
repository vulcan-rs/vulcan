use std::fmt::Display;

pub trait Command: Display {
    /// Returns the name of the command
    fn name(&self) -> String;

    /// Runs the command with the provided arguments
    fn run<A, R, E>(&self, args: A) -> Result<R, E>
    where
        A: IntoArgs,
        R: Display + Sized,
        E: Display;
}

pub struct CommandError {}

pub trait IntoArgs {
    fn args(&self) -> Vec<Argument>;
}

impl IntoArgs for String {
    fn args(&self) -> Vec<Argument> {
        self.split(" ").into_iter().map(|a| Argument {
            display: Some(a.to_string()),
            ty: ArgumentType::String,
            name: a.to_string(),
        });
    }
}

pub struct Argument {
    display: Option<String>,
    ty: ArgumentType,
    name: String,
}

pub enum ArgumentType {
    Integer,
    String,
}
