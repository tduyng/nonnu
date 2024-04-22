use crate::{env::Env, utils::extract_ident, val::Val};

mod binding_usage;

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    pub name: String,
}

impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = extract_ident(s)?;
        Ok((s, Self { name: name.to_string() }))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding_value(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                },
            )),
        );
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string()
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_not_exist_binding_usage(){
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage{
                name: "not-exist".to_string()
            }.eval(&empty_env),
            Err("binding with name 'not-exist' does not exist".to_string())
        )
    }
}
