use rustyline::{completion::Pair, Context, Result};

#[derive(Debug, Default)]
pub struct ReplCompleter {}

impl ReplCompleter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        let mut pairs = vec![Pair {
            display: String::from("Display"),
            replacement: String::from("Replacement"),
        }];

        Ok((0, pairs))
    }
}
