pub mod ast;
pub mod indexer;
pub mod lexer;
pub mod parser;
pub mod resolver;

fn error(loc: lexer::Loc, msg: String) -> ! {
	panic!("{:?}: error: {}\n", loc, msg);
}

#[cfg(test)]
mod testing;
