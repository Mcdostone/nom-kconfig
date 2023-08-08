use crate::{
    assert_parsing_eq, assert_parsing_fail,
    attribute::{
        expression::{
            parse_expression, parse_string, AndExpression, Atom, CompareExpression,
            CompareOperator, Expression, Term,
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
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Number(-412))))
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
            Expression::Term(AndExpression::Term(Term::Not(Atom::Symbol(
                Symbol::Constant("KVM".to_string())
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
            Expression::Term(AndExpression::Expression(vec!(
                Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_MIATA".to_string()))),
                Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
            )))
        ))
    )
}

#[test]
fn test_parse_number_or_symbol() {
    assert_parsing_eq!(
        parse_expression,
        "64BITS",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                Symbol::Constant("64BITS".to_string()),
            ))))
        ))
    );

    assert_parsing_eq!(
        parse_expression,
        "64",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Number(64))),)
        ))
    );

    assert_parsing_eq!(
        parse_expression,
        "\"64\"",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::String(
                "64".to_string()
            ))))
        ))
    );

    assert_parsing_eq!(
        parse_expression,
        "'64'",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                Symbol::NonConstant("'64'".to_string())
            )),))
        ))
    );
}

#[test]
fn test_parse_depends_on_ambigus() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164",
        Ok((
            "",
            Expression::Expression(vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                    "ALPHA_MIATA".to_string()
                )))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                ))
            ))
        ))
    )
}

// 5.0/scripts/gcc-plugins/Kconfig
#[test]
fn test_parse_string() {
    assert_parsing_eq!(
        parse_string,
        r#""$(shell,$(srctree)/scripts/gcc-plugin.sh "$(preferred-plugin-hostcc)" "$(HOSTCXX)" "$(CC)")""#,
        Ok((
            "",
            r#"$(shell,$(srctree)/scripts/gcc-plugin.sh "$(preferred-plugin-hostcc)" "$(HOSTCXX)" "$(CC)")"#.to_string()),
        ));

    assert_parsing_eq!(
        parse_string,
        r#""hello "world"" if NET"#,
        Ok((" if NET", r#"hello "world""#.to_string()),)
    );

    assert_parsing_fail!(parse_string, r#""hello "world""#);

    assert_parsing_fail!(
        parse_string,
        r#""hello "world"
""#
    )
}

#[test]
fn test_parse_depends_on_optimization() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA || ALPHA_LX164 && ALPHA_SX164 && (HELLO = world) || ALPHA_SX164 && (HELLO = world)",
        Ok(("", Expression::Expression(
            vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_MIATA".to_string())))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_LX164".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression { left: Symbol::Constant("HELLO".to_string()), operator: CompareOperator::Equal, right: Symbol::Constant("world".to_string()) }))))))),
                )),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("ALPHA_SX164".to_string()))),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression { left: Symbol::Constant("HELLO".to_string()), operator: CompareOperator::Equal, right: Symbol::Constant("world".to_string())})))))))
                )
            )
        )))))
}

#[test]
fn test_parse_expression_function() {
    assert_parsing_eq!(
        parse_expression,
        "$(success,$(OBJCOPY) --version | head -n1 | grep -qv llvm)",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Function(
                FunctionCall {
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
                }
            ))))
        ))
    )
}

// v3.6-rc4/drivers/mtd/maps/Kconfig
#[test]
fn test_parse_expression_start_like_number_but_symbol() {
    assert_parsing_eq!(
        parse_expression,
        r#"8xx && MTD_CFI"#,
        Ok((
            "",
            Expression::Term(AndExpression::Expression(vec!(
                Term::Atom(Atom::Symbol(Symbol::Constant("8xx".to_string()))),
                Term::Atom(Atom::Symbol(Symbol::Constant("MTD_CFI".to_string()))),
            )))
        ))
    )
}

#[test]
fn test_parse_expression_number_and() {
    assert_parsing_eq!(
        parse_expression,
        r#"8500 && MTD_CFI"#,
        Ok((
            "",
            Expression::Term(AndExpression::Expression(vec!(
                Term::Atom(Atom::Number(8500)),
                Term::Atom(Atom::Symbol(Symbol::Constant("MTD_CFI".to_string()))),
            )))
        ))
    )
}

#[test]
fn test_expression_to_string() {
    assert_eq!(
        "NUMBER_OF_PROCS = 5",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
            CompareExpression {
                left: Symbol::Constant("NUMBER_OF_PROCS".to_string()),
                operator: CompareOperator::Equal,
                right: Symbol::Constant("5".to_string())
            }
        ))))
        .to_string()
    );
    assert_eq!(
        "NUMBER_OF_PROCS != 5",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
            CompareExpression {
                left: Symbol::Constant("NUMBER_OF_PROCS".to_string()),
                operator: CompareOperator::NotEqual,
                right: Symbol::Constant("5".to_string())
            }
        ))))
        .to_string()
    );
    assert_eq!(
        "NUMBER_OF_PROCS < 5",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
            CompareExpression {
                left: Symbol::Constant("NUMBER_OF_PROCS".to_string()),
                operator: CompareOperator::LowerThan,
                right: Symbol::Constant("5".to_string())
            }
        ))))
        .to_string()
    );
    assert_eq!(
        "NUMBER_OF_PROCS <= 5",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
            CompareExpression {
                left: Symbol::Constant("NUMBER_OF_PROCS".to_string()),
                operator: CompareOperator::LowerOrEqual,
                right: Symbol::Constant("5".to_string())
            }
        ))))
        .to_string()
    );
    assert_eq!(
        "NUMBER_OF_PROCS > 5",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
            CompareExpression {
                left: Symbol::Constant("NUMBER_OF_PROCS".to_string()),
                operator: CompareOperator::GreaterThan,
                right: Symbol::Constant("5".to_string())
            }
        ))))
        .to_string()
    );
    assert_eq!(
        r#""A string with "double quotes"""#,
        Expression::Term(AndExpression::Term(Term::Atom(Atom::String(
            r#"A string with "double quotes""#.to_string()
        ))))
        .to_string()
    );
    assert_eq!(
        "NUMBER_OF_PROCS >= 5",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
            CompareExpression {
                left: Symbol::Constant("NUMBER_OF_PROCS".to_string()),
                operator: CompareOperator::GreaterOrEqual,
                right: Symbol::Constant("5".to_string())
            }
        ))))
        .to_string()
    );
    assert_eq!(
        "KVM && NET",
        Expression::Term(AndExpression::Expression(vec!(
            Term::Atom(Atom::Symbol(Symbol::Constant("KVM".to_string()))),
            Term::Atom(Atom::Symbol(Symbol::Constant("NET".to_string())))
        )))
        .to_string()
    );
    assert_eq!(
        "KVM || NET",
        Expression::Expression(vec!(
            AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                "KVM".to_string()
            )))),
            AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant(
                "NET".to_string()
            ))))
        ))
        .to_string()
    );
    assert_eq!(
        "!KVM",
        Expression::Term(AndExpression::Term(Term::Not(Atom::Symbol(
            Symbol::Constant("KVM".to_string())
        ))))
        .to_string()
    );
    assert_eq!(
        "55",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Number(55)))).to_string()
    );
    assert_eq!(
        r#"(hello)"#,
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Parenthesis(
            Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::Constant("hello".to_string()))
            ))))
        ))))
        .to_string()
    );
    assert_eq!(
        "$(warning)",
        Expression::Term(AndExpression::Term(Term::Atom(Atom::Function(
            FunctionCall {
                name: "warning".to_string(),
                parameters: vec!()
            }
        ))))
        .to_string()
    );
}
