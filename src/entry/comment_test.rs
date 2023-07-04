use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
        Attribute,
    },
    entry::comment::{parse_comment, Comment},
    symbol::Symbol,
};

#[test]
fn test_parse_comment() {
    let input = "comment \"Default contiguous memory area size:\"";
    assert_parsing_eq!(
        parse_comment,
        input,
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
