pub mod ast;
pub mod indexer;
pub mod lexer;
pub mod parser;
pub mod resolver;

fn error(loc: lexer::Loc, msg: String) -> ! {
	panic!("{loc}: error: {msg}\n");
}

#[cfg(test)]
mod testing;
