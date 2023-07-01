use crate::{
    assert_parsing_eq,
    attribute::{
        def_bool::DefBool,
        expression::{Expression, Term},
        Attribute,
    },
    entry::menuconfig::{parse_menu_config, MenuConfig},
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
                attributes: vec!(Attribute::DefBool(DefBool {
                    expression: Expression::Term(Term::Symbol(Symbol::Constant("y".to_string()))),
                    ..Default::default()
                })),
            }
        ))
    )
}
