use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        AndExpression, Atom, Expression, OrExpression, Term,
    },
    entry::{parse_menu_config, MenuConfig},
    symbol::Symbol,
};

#[test]
fn test_parse_menuconfig() {
    let input = "menuconfig VIRTUALIZATION def_bool y";
    assert_parsing_eq!(
        parse_menu_config,
        input,
        Ok((
            "",
            MenuConfig {
                symbol: "VIRTUALIZATION".to_string(),
                r#type: ConfigType {
                    r#type: Type::DefBool(Expression(OrExpression::Term(AndExpression::Term(
                        Term::Atom(Atom::Symbol(Symbol::Constant("y".to_string())))
                    )))),
                    prompt: None,
                    r#if: None,
                },
                attributes: vec!()
            }
        ))
    )
}
