use rustyline::{
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Context, Result,
};
use rustyline_derive::Helper;

use crate::repl::{ReplCompleter, ReplHinter, ReplValidator};

#[derive(Helper)]
pub struct ReplHelper {
    completer: ReplCompleter,
    validator: ReplValidator,
    hinter: ReplHinter,
}

impl ReplHelper {
    pub fn new() -> Self {
        Self {
            completer: ReplCompleter::new(),
            validator: ReplValidator::new(),
            hinter: ReplHinter::new(),
        }
    }
}

impl Highlighter for ReplHelper {}

impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Validator for ReplHelper {}

impl Hinter for ReplHelper {
    type Hint = String;
}
