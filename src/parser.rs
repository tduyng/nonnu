use std::{cell::Cell, path::PathBuf};

use crate::lexer::{lex, Loc, Token, TokenKind};

const INITIAL_FUEL: u8 = 255;

pub fn parse(input: &str, file: PathBuf) -> Ast {
	let tokens = lex(input, file);
	Parser::new(tokens).parse()
}

#[derive(Clone, PartialEq, Eq)]
pub struct Ast {
	pub definitions: Vec<Definition>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Definition {
	Procedure(Procedure),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Procedure {
	pub name: String,
	pub parameters: Vec<Parameter>,
	pub return_ty: Option<Ty>,
	pub body: Statement,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parameter {
	pub name: String,
	pub ty: Ty,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Statement {
	kind: StatementKind,
	loc: Loc,
}

#[derive(Clone, PartialEq, Eq)]
pub enum StatementKind {
	Expression(Expression),
	Block(Vec<Statement>),
	LocalDeclaration { name: String, ty: Ty },
	LocalDefinition { name: String, value: Expression },
	Assignment { lhs: Expression, rhs: Expression },
	Return { value: Option<Expression> },
}

#[derive(Clone, PartialEq, Eq)]
pub struct Expression {
	kind: ExpressionKind,
	loc: Loc,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ExpressionKind {
	Integer(u64),
	Variable(String),
	True,
	False,
	Binary { lhs: Box<Expression>, operator: BinaryOperator, rhs: Box<Expression> },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,
	ShiftLeft,
	ShiftRight,
	BitAnd,
	BitOr,
	BitXor,
	And,
	Or,
	Equal,
	NotEqual,
	Less,
	Greater,
	LessEqual,
	GreaterEqual,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Ty {
	Int,
}

struct Parser {
	tokens: Vec<Token>,
	cursor: usize,
	fuel: Cell<u8>,
}

impl Parser {
	fn new(tokens: Vec<Token>) -> Parser {
		Parser { tokens, cursor: 0, fuel: Cell::new(INITIAL_FUEL) }
	}

	fn parse(mut self) -> Ast {
		let mut definitions = Vec::new();

		while !self.at_eof() {
			definitions.push(self.parse_definition());
		}

		Ast { definitions }
	}

	fn parse_definition(&mut self) -> Definition {
		match self.current() {
			TokenKind::ProcKw => self.parse_procedure(),
			_ => self.error("expected definition".to_string()),
		}
	}

	fn parse_procedure(&mut self) -> Definition {
		self.bump(TokenKind::ProcKw);
		let name = self.expect_text(TokenKind::Identifier);

		self.expect(TokenKind::LParen);
		let mut parameters = Vec::new();

		while !self.at_eof() && !self.at(TokenKind::RParen) {
			let parameter_name = self.expect_text(TokenKind::Identifier);
			let parameter_ty = self.parse_ty();
			parameters.push(Parameter { name: parameter_name, ty: parameter_ty });

			match (self.current(), self.lookahead()) {
				(TokenKind::RParen, _) => {}
				(TokenKind::Comma, TokenKind::RParen) => self.bump(TokenKind::Comma),
				_ => self.expect(TokenKind::Comma),
			}

			if self.at(TokenKind::RParen) {
				self.eat(TokenKind::Comma);
			}
		}

		self.expect(TokenKind::RParen);

		let mut return_ty = None;
		if !self.at(TokenKind::LBrace) {
			return_ty = Some(self.parse_ty());
		}

		let body = self.parse_block();

		Definition::Procedure(Procedure { name, parameters, return_ty, body })
	}

	fn parse_statement(&mut self) -> Statement {
		match self.current() {
			TokenKind::VarKw => self.parse_local_declaration(),
			TokenKind::ReturnKw => self.parse_return(),
			TokenKind::LBrace => self.parse_block(),
			_ if self.lookahead() == TokenKind::ColonEqual => self.parse_local_definition(),
			_ => {
				if self.lookahead() == TokenKind::ColonEqual {
					return self.parse_local_declaration();
				}

				let loc = self.current_loc();
				let lhs = self.parse_expression();
				let operator_kind = self.current();
				let operator = match assignment_token_kind_to_operator(operator_kind) {
					Some(op) => op,
					None => return Statement { kind: StatementKind::Expression(lhs), loc },
				};
				self.bump(operator_kind);

				let rhs = self.parse_expression();

				match operator {
					Some(operator) => Statement {
						kind: StatementKind::Assignment {
							lhs: lhs.clone(),
							rhs: Expression {
								kind: ExpressionKind::Binary { lhs: Box::new(lhs), operator, rhs: Box::new(rhs) },
								loc: loc.clone(),
							},
						},
						loc,
					},
					None => Statement { kind: StatementKind::Assignment { lhs, rhs }, loc },
				}
			}
		}
	}

	fn parse_local_declaration(&mut self) -> Statement {
		let loc = self.current_loc();
		self.bump(TokenKind::VarKw);
		let name = self.expect_text(TokenKind::Identifier);
		let ty = self.parse_ty();
		Statement { kind: StatementKind::LocalDeclaration { name, ty }, loc }
	}

	fn parse_local_definition(&mut self) -> Statement {
		let loc = self.current_loc();
		let name = self.expect_text(TokenKind::Identifier);
		self.bump(TokenKind::ColonEqual);

		let value = self.parse_expression();

		Statement { kind: StatementKind::LocalDefinition { name, value }, loc }
	}

	fn parse_return(&mut self) -> Statement {
		let loc = self.current_loc();
		self.bump(TokenKind::ReturnKw);
		let value = if self.at_expression() { Some(self.parse_expression()) } else { None };

		Statement { kind: StatementKind::Return { value }, loc }
	}

	fn parse_block(&mut self) -> Statement {
		let loc = self.current_loc();
		self.bump(TokenKind::LBrace);

		let mut statements = Vec::new();
		while !self.at_eof() && !self.at(TokenKind::RBrace) {
			statements.push(self.parse_statement());
		}

		self.expect(TokenKind::RBrace);

		Statement { kind: StatementKind::Block(statements), loc }
	}

	fn parse_expression(&mut self) -> Expression {
		self.parse_expression_bp(0)
	}

	fn parse_expression_bp(&mut self, bp: u8) -> Expression {
		let mut lhs = self.parse_atom();

		loop {
			let operator_kind = self.current();
			let operator = match token_kind_to_operator(operator_kind) {
				Some(o) => o,
				None => break,
			};
			let right_bp = operator_to_bp(operator);

			if right_bp <= bp {
				break;
			}
			self.bump(operator_kind);
			let loc = self.current_loc();
			lhs = Expression {
				kind: ExpressionKind::Binary {
					lhs: Box::new(lhs),
					operator,
					rhs: Box::new(self.parse_expression_bp(right_bp)),
				},
				loc,
			}
		}
		lhs
	}

	fn parse_atom(&mut self) -> Expression {
		assert!(self.at_expression());
		match self.current() {
			TokenKind::Integer => {
				let loc = self.current_loc();
				let text = self.expect_text(TokenKind::Integer);
				Expression { kind: ExpressionKind::Integer(text.parse().unwrap()), loc }
			}
			TokenKind::Identifier => {
				let loc = self.current_loc();
				let text = self.expect_text(TokenKind::Identifier);
				Expression { kind: ExpressionKind::Variable(text), loc }
			}
			TokenKind::TrueKw => {
				let loc = self.current_loc();
				self.bump(TokenKind::TrueKw);
				Expression { kind: ExpressionKind::True, loc }
			}

			TokenKind::FalseKw => {
				let loc = self.current_loc();
				self.bump(TokenKind::FalseKw);
				Expression { kind: ExpressionKind::False, loc }
			}
			_ => self.error("expected expression".to_string()),
		}
	}

	fn parse_ty(&mut self) -> Ty {
		let text = self.expect_text(TokenKind::Identifier);
		match text.as_str() {
			"int" => Ty::Int,
			_ => self.error("expected type".to_string()),
		}
	}

	fn expect_text(&mut self, kind: TokenKind) -> String {
		let text = self.tokens[self.cursor].text.clone();
		self.expect(kind);
		text
	}

	fn expect(&mut self, kind: TokenKind) {
		if !self.eat(kind) {
			self.error(format!("expected {kind:?} but found {:?}", self.current()));
		}
	}

	fn eat(&mut self, kind: TokenKind) -> bool {
		if self.at(kind) {
			self.bump(kind);
			return true;
		}
		false
	}

	fn bump(&mut self, kind: TokenKind) {
		assert!(self.at(kind));
		self.cursor += 1;
		self.fuel.set(INITIAL_FUEL);
	}

	fn at_expression(&self) -> bool {
		matches!(self.current(), TokenKind::Integer | TokenKind::Identifier | TokenKind::TrueKw | TokenKind::FalseKw)
	}

	fn at(&self, kind: TokenKind) -> bool {
		self.current() == kind
	}

	fn current(&self) -> TokenKind {
		let remaining_fuel = self.fuel.get();
		if remaining_fuel == 0 {
			panic!("parser ran out of fuel");
		}
		self.fuel.set(remaining_fuel - 1);

		if self.at_eof() {
			return TokenKind::Eof;
		}
		self.tokens[self.cursor].kind
	}

	fn current_loc(&self) -> Loc {
		self.tokens[self.cursor].loc.clone()
	}

	fn lookahead(&self) -> TokenKind {
		if self.cursor + 1 >= self.tokens.len() {
			return TokenKind::Eof;
		}
		self.tokens[self.cursor + 1].kind
	}

	fn at_eof(&self) -> bool {
		self.cursor >= self.tokens.len()
	}

	fn error(&self, msg: String) -> ! {
		let loc = if self.at_eof() {
			self.tokens[self.tokens.len() - 1].loc.clone()
		} else {
			self.tokens[self.cursor].loc.clone()
		};

		crate::error(loc, msg);
	}
}

fn token_kind_to_operator(kind: TokenKind) -> Option<BinaryOperator> {
	Some(match kind {
		TokenKind::Plus => BinaryOperator::Add,
		TokenKind::Minus => BinaryOperator::Subtract,
		TokenKind::Star => BinaryOperator::Multiply,
		TokenKind::Slash => BinaryOperator::Divide,
		TokenKind::Percent => BinaryOperator::Modulo,
		TokenKind::LessLess => BinaryOperator::ShiftLeft,
		TokenKind::GreaterGreater => BinaryOperator::ShiftRight,
		TokenKind::And => BinaryOperator::BitAnd,
		TokenKind::Pipe => BinaryOperator::BitOr,
		TokenKind::Caret => BinaryOperator::BitXor,
		TokenKind::AndAnd => BinaryOperator::And,
		TokenKind::PipePipe => BinaryOperator::Or,
		TokenKind::EqualEqual => BinaryOperator::Equal,
		TokenKind::BangEqual => BinaryOperator::NotEqual,
		TokenKind::Less => BinaryOperator::Less,
		TokenKind::Greater => BinaryOperator::Greater,
		TokenKind::LessEqual => BinaryOperator::LessEqual,
		TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
		_ => return None,
	})
}

fn operator_to_bp(operator: BinaryOperator) -> u8 {
	match operator {
		BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => 9,

		BinaryOperator::BitAnd => 8,

		BinaryOperator::BitXor => 7,

		BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => 6,

		BinaryOperator::Add | BinaryOperator::Subtract => 5,

		BinaryOperator::BitOr => 4,

		BinaryOperator::Equal
		| BinaryOperator::NotEqual
		| BinaryOperator::Less
		| BinaryOperator::Greater
		| BinaryOperator::LessEqual
		| BinaryOperator::GreaterEqual => 3,

		BinaryOperator::And => 2,

		BinaryOperator::Or => 1,
	}
}

fn assignment_token_kind_to_operator(kind: TokenKind) -> Option<Option<BinaryOperator>> {
	if kind == TokenKind::Equal {
		return Some(None);
	}

	let operator = match kind {
		TokenKind::PlusEqual => BinaryOperator::Add,
		TokenKind::MinusEqual => BinaryOperator::Subtract,
		TokenKind::StarEqual => BinaryOperator::Multiply,
		TokenKind::SlashEqual => BinaryOperator::Divide,
		TokenKind::PercentEqual => BinaryOperator::Modulo,
		TokenKind::LessLessEqual => BinaryOperator::ShiftLeft,
		TokenKind::GreaterGreaterEqual => BinaryOperator::ShiftRight,
		TokenKind::AndEqual => BinaryOperator::BitAnd,
		TokenKind::PipeEqual => BinaryOperator::BitOr,
		TokenKind::CaretEqual => BinaryOperator::BitXor,
		TokenKind::AndAndEqual => BinaryOperator::And,
		TokenKind::PipePipeEqual => BinaryOperator::Or,
		_ => return None,
	};

	Some(Some(operator))
}

impl Ast {
	pub fn pretty_print(&self) -> String {
		let mut ctx = PrettyPrintCtx { buf: String::new(), indentation: 0 };
		ctx.print_ast(self);
		ctx.buf
	}
}

struct PrettyPrintCtx {
	buf: String,
	indentation: usize,
}

impl PrettyPrintCtx {
	fn print_ast(&mut self, ast: &Ast) {
		for definition in &ast.definitions {
			match definition {
				Definition::Procedure(proc) => self.print_procedure(proc),
			}
		}
	}

	fn print_procedure(&mut self, proc: &Procedure) {
		self.s("proc ");
		self.s(&proc.name);
		self.s("(");

		for (i, parameter) in proc.parameters.iter().enumerate() {
			if i != 0 {
				self.s(", ");
			}

			self.s(&parameter.name);
			self.s(" ");
			self.print_ty(&parameter.ty);
		}

		self.s(")");

		if let Some(return_ty) = &proc.return_ty {
			self.s(" ");
			self.print_ty(return_ty);
			self.s(" ");
		}

		if proc.body.kind == StatementKind::Block(Vec::new()) {
			if proc.return_ty.is_none() {
				self.s(" ");
			}
			self.s("{}");
		} else {
			self.newline();
			self.print_statement(&proc.body);
		}

		self.newline()
	}

	fn print_statement(&mut self, statement: &Statement) {
		match &statement.kind {
			StatementKind::LocalDeclaration { name, ty } => {
				self.s("var ");
				self.s(name);
				self.s(" ");
				self.print_ty(ty);
			}
			StatementKind::LocalDefinition { name, value } => {
				self.s(name);
				self.s(" := ");
				self.print_expression(value);
			}
			StatementKind::Expression(e) => self.print_expression(e),
			StatementKind::Return { value } => {
				self.s("return");

				if let Some(value) = value {
					self.s(" ");
					self.print_expression(value);
				}
			}

			StatementKind::Block(statements) => {
				self.s("{");
				self.indentation += 1;

				for statement in statements {
					self.newline();
					self.print_statement(statement);
				}

				self.indentation -= 1;
				self.newline();
				self.s("}");
			}
			StatementKind::Assignment { lhs, rhs } => {
				self.print_expression(lhs);
				self.s(" = ");
				self.print_expression(rhs);
			}
		}
	}

	fn print_expression(&mut self, expression: &Expression) {
		match &expression.kind {
			ExpressionKind::Integer(i) => self.s(&format!("{i}")),
			ExpressionKind::Variable(name) => self.s(name),
			ExpressionKind::True => self.s("true"),
			ExpressionKind::False => self.s("false"),
			ExpressionKind::Binary { lhs, operator, rhs } => {
				self.s("(");
				self.print_expression(lhs);
				self.s(" ");

				let op = match operator {
					BinaryOperator::Add => "+",
					BinaryOperator::Subtract => "-",
					BinaryOperator::Multiply => "*",
					BinaryOperator::Divide => "/",
					BinaryOperator::Modulo => "%",
					BinaryOperator::ShiftLeft => "<<",
					BinaryOperator::ShiftRight => ">>",
					BinaryOperator::BitAnd => "&",
					BinaryOperator::BitOr => "|",
					BinaryOperator::BitXor => "^",
					BinaryOperator::And => "&&",
					BinaryOperator::Or => "||",
					BinaryOperator::Equal => "==",
					BinaryOperator::NotEqual => "!=",
					BinaryOperator::Less => "<",
					BinaryOperator::Greater => ">",
					BinaryOperator::LessEqual => "<=",
					BinaryOperator::GreaterEqual => ">=",
				};
				self.s(op);

				self.s(" ");
				self.print_expression(rhs);
				self.s(")");
			}
		}
	}

	fn print_ty(&mut self, ty: &Ty) {
		match ty {
			Ty::Int => self.s("int"),
		}
	}

	fn s(&mut self, s: &str) {
		self.buf.push_str(s);
	}

	fn newline(&mut self) {
		self.buf.push('\n');
		for _ in 0..self.indentation {
			self.buf.push('\t');
		}
	}
}

#[cfg(test)]
#[test]
fn tests() {
	crate::testing::run_tests("tests/parser", |input| {
		let ast = parse(input, PathBuf::from("test"));
		ast.pretty_print()
	});
}
