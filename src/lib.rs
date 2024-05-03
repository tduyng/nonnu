pub mod lexer;
pub mod parser;

fn error(loc: lexer::Loc, msg: String) -> ! {
	panic!("{:?}: error: {}\n", loc, msg);
}

#[cfg(test)]
mod testing;
