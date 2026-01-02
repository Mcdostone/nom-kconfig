use crate::{
    assert_parsing_eq, assert_parsing_fail,
    attribute::{AndExpression, Atom, Expression, Term},
    entry::{parse_menu, Menu},
    symbol::Symbol,
};

#[test]
fn test_parse_menu() {
    let input = "menu \"BPF subsystem\" endmenu";
    assert_parsing_eq!(
        parse_menu,
        input,
        Ok((
            "",
            Menu {
                prompt: "BPF subsystem".to_string(),
                visible: None,
                ..Default::default()
            }
        ))
    )
}

#[test]
fn test_parse_menu_visible() {
    let input = "menu \"BPF subsystem\" visible if EXPERT endmenu";
    assert_parsing_eq!(
        parse_menu,
        input,
        Ok((
            "",
            Menu {
                prompt: "BPF subsystem".to_string(),
                visible: Some(Some(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::NonConstant("EXPERT".to_string()))
                ))))),
                ..Default::default()
            }
        ))
    )
}

#[test]
fn test_parse_menu_forbidden_attribute() {
    let input = "menu \"BPF subsystem\" select EXPERT endmenu";
    assert_parsing_fail!(parse_menu, input)
}

#[test]
fn test_parse_menu_depends_on() {
    let input = "menu \"BPF subsystem\"     depends on MODULES endmenu";
    assert_parsing_eq!(
        parse_menu,
        input,
        Ok((
            "",
            Menu {
                prompt: "BPF subsystem".to_string(),
                visible: None,
                depends_on: vec!(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::NonConstant("MODULES".to_string()))
                )))),
                ..Default::default()
            }
        ))
    )
}
