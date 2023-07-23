use crate::{
    assert_parsing_eq,
    attribute::{
        parse_def_bool, AndExpression, Atom, DefBool, Expression, ExpressionToken, FunctionCall,
        OrExpression, Parameter, Term,
    },
    symbol::Symbol,
};

#[test]
fn test_parse_def_bool() {
    assert_parsing_eq!(
        parse_def_bool,
        "def_bool     !PCI ",
        Ok((
            " ",
            DefBool {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Not(
                    Atom::Symbol(Symbol::Constant("PCI".to_string()))
                )))),
                r#if: None
            }
        ))
    )
}

// 5.19.7/arch/x86/Kconfig.assembler
#[test]
fn test_parse_def_bool_function() {
    assert_parsing_eq!(
        parse_def_bool,
        "def_bool $(as-instr,vpmovm2b %k1$(comma)%zmm5)",
        Ok((
            "",
            DefBool {
                expression: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Function(FunctionCall {
                        name: "as-instr".to_string(),
                        parameters: vec!(Parameter {
                            tokens: vec!(
                                ExpressionToken::Literal("vpmovm2b".to_string()),
                                ExpressionToken::Space,
                                ExpressionToken::Literal("%k1".to_string()),
                                ExpressionToken::Function(Box::new(FunctionCall {
                                    name: "comma".to_string(),
                                    parameters: vec!()
                                })),
                                ExpressionToken::Literal("%zmm5".to_string())
                            )
                        })
                    })
                )))),
                r#if: None
            }
        ))
    )
}
