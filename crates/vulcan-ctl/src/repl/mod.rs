use rustyline::{error::ReadlineError, Editor};

mod command;
mod completer;
mod error;
mod helper;
mod hinter;
mod validator;

pub use command::*;
pub use completer::*;
pub use error::*;
pub use helper::*;
pub use hinter::*;
pub use validator::*;

pub struct Repl<'a> {
    commands: Vec<Box<dyn Command>>,
    prompt: &'a str,
    state: State,
}

impl<'a> Repl<'a> {
    pub fn new(prompt: &'a str) -> Self {
        Self {
            state: State::default(),
            commands: vec![],
            prompt,
        }
    }

    pub fn run(&mut self) -> Result<(), ReplError> {
        let mut repl = match Editor::<ReplHelper>::new() {
            Ok(editor) => editor,
            Err(err) => return Err(err.into()),
        };

        let helper = ReplHelper::new();
        repl.set_helper(Some(helper));

        loop {
            let readline = repl.readline(">> ");
            match readline {
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    self.process_input(line)?
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Received CTRL-C");

                    #[cfg(debug_assertions)]
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Received CTRL-D");

                    #[cfg(debug_assertions)]
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }

        Ok(())
    }

    pub fn add_command(&mut self, command: impl Command) {}

    fn process_input(&mut self, input: String) -> Result<(), ReplError> {
        println!("Received: {}", input);

        let (cmd, args) = match self.get_command_and_args(input) {
            Some(parts) => parts,
            None => todo!(),
        };

        cmd.run(args);

        Ok(())
    }

    fn get_command_and_args(&self, input: String) -> Option<(&impl Command, String)> {
        let input = input.trim();

        let (cmd, args) = match input.split_once(" ") {
            Some(parts) => parts,
            None => todo!(),
        };

        for command in &self.commands {
            if cmd == command.name() {
                return Some((command, args.to_string()));
            }
        }

        None
    }
}

pub enum State {
    Initial,
    Typing,
    Running,
}

impl Default for State {
    fn default() -> Self {
        Self::Initial
    }
}
