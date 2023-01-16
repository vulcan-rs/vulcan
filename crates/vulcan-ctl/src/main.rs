mod repl;

use repl::Repl;

fn main() {
    let mut repl = Repl::new(">> ");

    match repl.run() {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
