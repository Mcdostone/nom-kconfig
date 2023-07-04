use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
        visible::Visible,
    },
    entry::menu::{parse_menu, Menu},
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
                depends_on: vec!(),
                blocks: vec!()
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
                visible: Some(Visible {
                    r#if: Some(Expression(OrExpression::Term(AndExpression::Term(
                        Term::Atom(Atom::Symbol(Symbol::Constant("EXPERT".to_string())))
                    )))),
                }),
                depends_on: vec!(),
                blocks: vec!()
            }
        ))
    )
}
/*
#[test]
fn test_parse_menu_depends_on() {
    let input = "menu \"BPF subsystem\" depends on ARC_HAS_ICACHE || ARC_HAS_DCACHE endmenu";
    assert_parsing_eq!(
        parse_menu,
        input,
        Ok((
            "",
            Menu {
                prompt: "BPF subsystem".to_string(),
                depends_on: vec!(Expression::Operation(Operation {
                    operator: Operator::Or,
                    operands: vec!(
                        Expression::Term(Term::Symbol(Symbol::Constant("ARC_HAS_ICACHE".to_string()))),
                        Expression::Term(Term::Symbol(Symbol::Constant("ARC_HAS_DCACHE".to_string())))
                    )})
            ),
                blocks: vec!(),
                visible: None
            }
        ))
    )
}
*/
