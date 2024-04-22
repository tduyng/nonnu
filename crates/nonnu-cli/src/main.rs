use nonnu::env::Env;
use std::io::{self, Write};

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
            Ok(()) => {}
            Err(msg) => {
                writeln!(stderr, "{}", msg)?;
                stderr.flush()?;
            }
        }

        input.clear();
    }
}

fn run(input: &str, env: &mut Env) -> Result<(), String> {
    let parse = nonnu::parse(input).map_err(|msg| format!("parse error: {}", msg))?;
    let evaluated = parse.eval(env).map_err(|msg| format!("evaluation error: {}", msg))?;

    dbg!(evaluated);

    Ok(())
}
