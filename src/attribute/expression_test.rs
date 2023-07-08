use crate::{
    assert_parsing_eq,
    attribute::{
        expression::{
            parse_expression, AndExpression, Atom, CompareExpression, CompareOperator, Expression,
            OrExpression, Term,
        },
        function::{ExpressionToken, FunctionCall, Parameter},
    },
    symbol::Symbol,
};

#[test]
fn test_parse_expression_number() {
    assert_parsing_eq!(
        parse_expression,
        "-412",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                Atom::Number(-412)
            ))))
        ))
    )
}

#[test]
fn test_parse_term() {
    assert_parsing_eq!(
        parse_expression,
        "!KVM",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Term(Term::Not(
                Atom::Symbol(Symbol::Constant("KVM".to_string()))
            ))))
        ))
    )
}

#[test]
fn test_parse_depends_on_and() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA && ALPHA_LX164",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Expression(vec!(
                Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_MIATA".to_string()))),
                Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
            ))))
        ))
    )
}

#[test]
fn test_parse_depends_on_ambigus() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164",
        Ok((
            "",
            Expression(OrExpression::Expression(vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                    "ALPHA_MIATA".to_string()
                )))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                ))
            )))
        ))
    )
}

#[test]
fn test_parse_depends_on_optimization() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164 && (HELLO = world) || ALPHA_SX164 && (HELLO = world)",
        Ok(("", Expression(OrExpression::Expression(
            vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_MIATA".to_string())))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression(OrExpression::Term(AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression { left: Symbol::Constant("HELLO".to_string()), operator: CompareOperator::Equal, right: Symbol::Constant("world".to_string()) }))))))),
                ))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression(OrExpression::Term(AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression { left: Symbol::Constant("HELLO".to_string()), operator: CompareOperator::Equal, right: Symbol::Constant("world".to_string())}))))))))
                )
            )
        ))))))
}

#[test]
fn test_parse_expression_function() {
    assert_parsing_eq!(
        parse_expression,
        "$(success,$(OBJCOPY) --version | head -n1 | grep -qv llvm)",
        Ok((
            "",
            Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                Atom::Function(FunctionCall {
                    name: "success".to_string(),
                    parameters: vec!(Parameter {
                        tokens: vec!(
                            ExpressionToken::Variable("OBJCOPY".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("--version".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("|".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("head".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("-n1".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("|".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("grep".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("-qv".to_string()),
                            ExpressionToken::Space,
                            ExpressionToken::Literal("llvm".to_string())
                        )
                    })
                })
            ))))
        ))
    )
}
