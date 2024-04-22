use env::Env;
use val::Val;

pub mod binding_def;
pub mod env;
pub mod expr;
pub mod stmt;
pub mod val;

mod utils;

#[derive(Debug)]
pub struct Parse(stmt::Stmt);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;
    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}
