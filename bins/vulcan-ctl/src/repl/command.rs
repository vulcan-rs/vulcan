pub type CommandResult = Result<(), CommandError>;

pub enum CommandError {}

pub struct CommandContext {}

pub struct Command<F>
where
    F: Fn(CommandContext) -> CommandResult,
{
    sub_commands: Vec<Command<F>>,
    name: String,
    run: impl Fn(CommandContext) -> CommandResult,
}

impl<F> Command<F>
where
    F: Fn(CommandContext) -> CommandResult,
{
    pub fn new(name: String, run: F) -> Self {
        Self {
            sub_commands: vec![],
            name,
            run,
        }
    }

    pub fn add_sub_commands(&mut self, commands: &mut Vec<Command<F>>) {
        self.sub_commands.append(commands)
    }
}
