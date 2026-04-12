use crate::attribute::{parse_expression, ExpressionToken, FunctionCall, Parameter};
use crate::symbol::ConstantSymbol;
use crate::{
    assert_parsing_eq,
    attribute::{parse_default, AndExpression, Atom, DefaultAttribute, Expression, Term},
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
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::Constant(ConstantSymbol::Hex("0x1".to_string()))
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
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::NonConstant("7.10.d".to_string())
                )))),
                r#if: None
            }
        ))
    )
}

// 5.0 /scripts/gcc-plugins/Kconfig
#[test]
fn test_parse_default_ambigus() {
    assert_parsing_eq!(
        parse_default,
        r#"default "$(shell,$(srctree)/scripts/gcc-plugin.sh "$(preferred-plugin-hostcc)" "$(HOSTCXX)" "$(CC)")" if CC_IS_GCC"#,
        Ok((
            "",
            DefaultAttribute {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Function(
                    FunctionCall {
                        name: "shell".to_string(),
                        parameters: vec!(Parameter {
                            tokens: vec!(
                                ExpressionToken::Function(Box::new(FunctionCall {
                                    name: "srctree".to_string(),
                                    parameters: vec![]
                                })),
                                ExpressionToken::Literal("/scripts/gcc-plugin.sh".to_string()),
                                ExpressionToken::Space,
                                ExpressionToken::DoubleQuotes(vec![ExpressionToken::Function(
                                    Box::new(FunctionCall {
                                        name: "preferred-plugin-hostcc".to_string(),
                                        parameters: vec![]
                                    })
                                ),]),
                                ExpressionToken::Space,
                                ExpressionToken::DoubleQuotes(vec![ExpressionToken::Variable(
                                    "HOSTCXX".to_string()
                                )]),
                                ExpressionToken::Space,
                                ExpressionToken::DoubleQuotes(vec![ExpressionToken::Variable(
                                    "CC".to_string()
                                )])
                            )
                        })
                    }
                )))),
                r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Symbol(Symbol::NonConstant("CC_IS_GCC".to_string()))
                ))))
            }
        ))
    )
}

#[test]
fn test_default_attribute_to_string() {
    assert_eq!(
        DefaultAttribute {
            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                Symbol::Constant(ConstantSymbol::Integer(64))
            )))),
            r#if: None
        }
        .to_string(),
        "64"
    );

    assert_eq!(
        DefaultAttribute {
            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                Symbol::Constant(ConstantSymbol::Integer(64))
            )))),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::NonConstant("NET".to_string()))
            ))))
        }
        .to_string(),
        "64 if NET"
    )
}

#[test]
/// https://github.com/torvalds/linux/blob/master/init/Kconfig#L22-L25
fn test_default_attribute_number() {
    assert_parsing_eq!(
        parse_default,
        "default 0",
        Ok((
            "",
            DefaultAttribute {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::Constant(ConstantSymbol::Integer(0))
                )))),
                r#if: None
            }
        ))
    )
}

#[test]
/// https://github.com/torvalds/linux/blob/master/init/Kconfig#L22-L25
fn test_default_attribute_number_2() {
    assert_parsing_eq!(
        parse_default,
        "default \"845\"",
        Ok((
            "",
            DefaultAttribute {
                expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                    Symbol::Constant(ConstantSymbol::String("845".to_string()))
                )))),
                r#if: None
            }
        ))
    )
}

#[test]
fn test_default_attribute_number_3() {
    assert_parsing_eq!(
        parse_expression,
        "'console=ttyS0,19200'",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
                Symbol::Constant(ConstantSymbol::String("console=ttyS0,19200".to_string()))
            ))))
        ))
    )
}
