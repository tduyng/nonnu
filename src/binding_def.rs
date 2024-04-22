use crate::{
    env::Env,
    expr::Expr,
    utils::{extract_ident, extract_whitespace, remove_tag},
};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = remove_tag("let", s)?;
        let (s, _) = extract_whitespace(s, Some("expected a space".to_string()))?;
        let (s, name) = extract_ident(s)?;
        let (s, _) = extract_whitespace(s, None)?;
        let s = remove_tag("=", s)?;
        let (s, _) = extract_whitespace(s, None)?;
        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_binding(self.name.clone(), self.val.eval(env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::*;

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            )),
        );
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(BindingDef::new("letaaa=1+2"), Err("expected a space".to_string()),);
    }
}
