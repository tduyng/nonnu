mod db;
pub use db::Db;

use la_arena::Idx;

type ExprIdx = Idx<Expr>;

#[derive(Debug)]
pub enum Stmt {
    VariableDef { name: String, value: Expr },
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Missing,
    Binary { op: BinaryOp, lhs: ExprIdx, rhs: ExprIdx },
    Literal { n: u64 },
    Unary { op: UnaryOp, expr: ExprIdx },
    VariableRef { var: String },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> (Db, Vec<Stmt>) {
    let mut db = Db::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();
    (db, stmts)
}
