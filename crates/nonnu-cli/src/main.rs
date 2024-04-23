use std::io::{self, Write};

use nonnu::{Env, Val};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut input = String::new();
    let mut env = Env::default();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => writeln!(stdout, "{}", val)?,
            Ok(None) => {}
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }

        input.clear();
    }
}

fn run(input: &str, env: &mut Env) -> Result<Option<Val>, String> {
    let parse = nonnu::parse(input).map_err(|msg| format!("parse error: {}", msg))?;
    let evaluated = parse.eval(env).map_err(|msg| format!("evaluation error: {}", msg))?;

    if evaluated == nonnu::Val::Unit {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}
