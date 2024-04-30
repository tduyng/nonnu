mod db;
pub use db::Db;

use la_arena::Idx;

type ExprIdx = Idx<Expr>;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VariableDef { name: String, value: Expr },
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Missing,
    Binary { op: BinaryOp, lhs: ExprIdx, rhs: ExprIdx },
    Literal { n: Option<u64> },
    Unary { op: UnaryOp, expr: ExprIdx },
    VariableRef { var: String },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> (Db, Vec<Stmt>) {
    let mut db = Db::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();
    (db, stmts)
}
