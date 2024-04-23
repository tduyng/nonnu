use crate::{
    stmt::Stmt,
    utils::{extract_ident, extract_whitespace, remove_tag, sequence},
    Env,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Stmt>,
}

impl FuncDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = remove_tag("fn", s)?;
        let (s, _) = extract_whitespace(s, Some("expect a space".to_string()))?;

        let (s, name) = extract_ident(s)?;
        let (s, _) = extract_whitespace(s, None)?;

        let (s, params) = sequence(
            |s| extract_ident(s).map(|(s, ident)| (s, ident.to_string())),
            |s, error_msg| extract_whitespace(s, error_msg).map(|(s, _)| (s, "".to_string())),
            s,
            Some("Expected parameter list".to_string()),
        )?;

        let s = remove_tag("=>", s)?;
        let (s, _) = extract_whitespace(s, None)?;

        let (s, body) = Stmt::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params,
                body: Box::new(body),
            },
        ))
    }

    pub fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_func(self.name.clone(), self.params.clone(), *self.body.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BindingUsage, Block, Expr, Op};

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() }))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_def_with_one_param_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn greet name => {}"),
            Ok((
                "",
                FuncDef {
                    name: "greet".to_string(),
                    params: vec!["name".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() }))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage { name: "x".to_string() })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage { name: "y".to_string() })),
                        op: Op::Add,
                    })),
                },
            )),
        );
    }
}
