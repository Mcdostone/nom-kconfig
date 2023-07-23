use crate::{
    assert_parsing_eq,
    attribute::{
        parse_default, AndExpression, Atom, DefaultAttribute, Expression, ExpressionToken,
        FunctionCall, OrExpression, Parameter, Term,
    },
    symbol::Symbol,
};

#[test]
fn test_parse_default() {
    assert_parsing_eq!(
        parse_default,
        "default 0x1",
        Ok((
            "",
            DefaultAttribute {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("0x1".to_string()))
                )))),
                r#if: None
            }
        ))
    )
}

// v3.5.0/arch/microblaze/platform/generic/Kconfig.auto
#[test]
fn test_parse_default_constant_symbol_with_numbers() {
    assert_parsing_eq!(
        parse_default,
        "default 7.10.d",
        Ok((
            "",
            DefaultAttribute {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::Constant("7.10.d".to_string()))
                )))),
                r#if: None
            }
        ))
    )
}

// 5.0/scripts/gcc-plugins/Kconfig
#[test]
fn test_parse_default_ambigus() {
    assert_parsing_eq!(
        parse_default,
        r#"default "$(shell,$(srctree)/scripts/gcc-plugin.sh "$(preferred-plugin-hostcc)" "$(HOSTCXX)" "$(CC)")" if CC_IS_GCC"#,
        Ok((
            "",
            DefaultAttribute {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::String(Box::new(Atom::Function(FunctionCall {
                        name: "shell".to_string(),
                        parameters: vec!(Parameter {
                            tokens: vec!(
                                ExpressionToken::Function(Box::new(FunctionCall {
                                    name: "srctree".to_string(),
                                    parameters: vec!()
                                })),
                                ExpressionToken::Literal("/scripts/gcc-plugin.sh".to_string()),
                                ExpressionToken::Space,
                                ExpressionToken::DoubleQuotes(vec!(ExpressionToken::Function(
                                    Box::new(FunctionCall {
                                        name: "preferred-plugin-hostcc".to_string(),
                                        parameters: vec!()
                                    })
                                ))),
                                ExpressionToken::Space,
                                ExpressionToken::DoubleQuotes(vec!(ExpressionToken::Variable(
                                    "HOSTCXX".to_string()
                                ))),
                                ExpressionToken::Space,
                                ExpressionToken::DoubleQuotes(vec!(ExpressionToken::Variable(
                                    "CC".to_string()
                                ))),
                            )
                        },)
                    })))
                )))),
                r#if: Some(Expression(OrExpression::Term(AndExpression::Term(
                    Term::Atom(Atom::Symbol(Symbol::Constant("CC_IS_GCC".to_string())))
                ))))
            }
        ))
    )
}
