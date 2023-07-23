use crate::{
    assert_parsing_eq,
    attribute::{AndExpression, Atom, Attribute, Expression, OrExpression, Term},
    entry::{parse_comment, Comment},
    symbol::Symbol,
};

#[test]
fn test_parse_comment() {
    assert_parsing_eq!(
        parse_comment,
        r#"comment "Default contiguous memory area size:""#,
        Ok((
            "",
            Comment {
                prompt: "Default contiguous memory area size:".to_string(),
                dependencies: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_comment_with_dependencies() {
    let input = "comment \"Default contiguous memory area size:\" depends on JVM";
    assert_parsing_eq!(
        parse_comment,
        input,
        Ok((
            "",
            Comment {
                prompt: "Default contiguous memory area size:".to_string(),
                dependencies: vec!(Attribute::DependsOn(Expression(OrExpression::Term(
                    AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                        "JVM".to_string()
                    ))))
                ))))
            }
        ))
    )
}
