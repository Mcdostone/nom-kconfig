use crate::{
    assert_parsing_eq,
    attribute::{parse_function_call, ExpressionToken, FunctionCall, Parameter},
};

#[test]
fn test_parse_function_call() {
    assert_parsing_eq!(
        parse_function_call,
        "$(hello)",
        Ok((
            "",
            FunctionCall {
                name: "hello".to_string(),
                parameters: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_function_call_with_parameters() {
    assert_parsing_eq!(
        parse_function_call,
        "$(greeting,Hello,John)",
        Ok((
            "",
            FunctionCall {
                name: "greeting".to_string(),
                parameters: vec!(
                    Parameter {
                        tokens: vec!(ExpressionToken::Literal("Hello".to_string()))
                    },
                    Parameter {
                        tokens: vec!(ExpressionToken::Literal("John".to_string()))
                    }
                )
            }
        ))
    )
}

#[test]
fn test_parse_function_call_expression() {
    assert_parsing_eq!(
        parse_function_call,
        "$(warning,SIMPLE = basic)",
        Ok((
            "",
            FunctionCall {
                name: "warning".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal("SIMPLE".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Literal("=".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Literal("basic".to_string())
                    )
                })
            }
        ))
    )
}

#[test]
fn test_parse_function_call_expanded_variable() {
    let input = "$(warning,SIMPLE = $(SIMPLE))";
    assert_parsing_eq!(
        parse_function_call,
        input,
        Ok((
            "",
            FunctionCall {
                name: "warning".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal("SIMPLE".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Literal("=".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Variable("SIMPLE".to_string())
                    )
                })
            }
        ))
    )
}

// https://github.com/Mcdostone/nom-kconfig/issues/57
#[test]
fn test_parse_function_call_percent_symbol() {
    let input = "$(hey (%rbx))";
    assert_parsing_eq!(
        parse_function_call,
        input,
        Ok((
            "",
            FunctionCall {
                name: "hey".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(ExpressionToken::Literal("(%rbx)".to_string()))
                })
            }
        ))
    )
}

#[test]
fn test_parse_function_call_recursive_function() {
    let input = "$(warning,$(greeting,Hello,Jean-Louis))";
    assert_parsing_eq!(
        parse_function_call,
        input,
        Ok((
            "",
            FunctionCall {
                name: "warning".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(ExpressionToken::Function(Box::new(FunctionCall {
                        name: "greeting".to_string(),
                        parameters: vec!(
                            Parameter {
                                tokens: vec!(ExpressionToken::Literal("Hello".to_string()),)
                            },
                            Parameter {
                                tokens: vec!(ExpressionToken::Literal("Jean-Louis".to_string()),)
                            }
                        )
                    })))
                })
            }
        ))
    )
}

#[test]
fn test_parse_function_call_complex_expression() {
    let input = "$(warning,filename=$(filename))";
    assert_parsing_eq!(
        parse_function_call,
        input,
        Ok((
            "",
            FunctionCall {
                name: "warning".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal("filename".to_string()),
                        ExpressionToken::Literal("=".to_string()),
                        ExpressionToken::Function(Box::new(FunctionCall {
                            name: "filename".to_string(),
                            parameters: vec!()
                        }))
                    )
                })
            }
        ))
    )
}

#[test]
fn test_parse_function_call_special_chars() {
    let input = "$(as-instr,.arch armv8.5-a+memtag)";
    assert_parsing_eq!(
        parse_function_call,
        input,
        Ok((
            "",
            FunctionCall {
                name: "as-instr".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal(".arch".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Literal("armv8.5-a+memtag".to_string()),
                    )
                })
            }
        ))
    )
}

#[test]
fn test_parse_function_call_special_backtick() {
    let input = r#"$(success, test `$(PAHOLE) --version | sed -E 's/v([0-9]+)\.([0-9]+)/\1\2/'` -ge "119")"#;
    assert_parsing_eq!(
        parse_function_call,
        input,
        Ok((
            "",
            FunctionCall {
                name: "success".to_string(),
                parameters: vec!(Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal("test".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Backtick(
                            "$(PAHOLE) --version | sed -E 's/v([0-9]+)\\.([0-9]+)/\\1\\2/'"
                                .to_string()
                        ),
                        ExpressionToken::Space,
                        ExpressionToken::Literal("-ge".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::DoubleQuotes(vec!(ExpressionToken::Literal(
                            "119".to_string()
                        )))
                    )
                })
            }
        ))
    )
}

#[test]
fn test_parameter_to_string() {
    assert_eq!(
        Parameter {
            tokens: vec!(
                ExpressionToken::Literal("ls".to_string()),
                ExpressionToken::Space,
                ExpressionToken::Literal("-la".to_string())
            )
        }
        .to_string(),
        "ls -la"
    )
}

#[test]
fn test_function_to_string() {
    assert_eq!(
        FunctionCall {
            name: "warning".to_string(),
            parameters: vec!(
                Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal("ls".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Literal("-la".to_string())
                    )
                },
                Parameter {
                    tokens: vec!(
                        ExpressionToken::Literal("echo".to_string()),
                        ExpressionToken::Space,
                        ExpressionToken::Variable("HOME".to_string())
                    )
                }
            ),
        }
        .to_string(),
        "$(warning, ls -la, echo $HOME)"
    )
}

#[test]
fn test_expression_token_to_string() {
    assert_eq!(ExpressionToken::Literal("ls".to_string()).to_string(), "ls");
    assert_eq!(ExpressionToken::Space.to_string(), " ");
    assert_eq!(
        ExpressionToken::Variable("PWD".to_string()).to_string(),
        "$PWD"
    );
    assert_eq!(
        ExpressionToken::DoubleQuotes(vec!(
            ExpressionToken::Literal("hello".to_string()),
            ExpressionToken::Space,
            ExpressionToken::Literal("world".to_string())
        ))
        .to_string(),
        r#""hello world""#
    );
    assert_eq!(
        ExpressionToken::SingleQuotes("hello".to_string()).to_string(),
        "'hello'"
    );
    assert_eq!(
        ExpressionToken::Backtick("hello".to_string()).to_string(),
        "`hello`"
    );
    assert_eq!(
        ExpressionToken::Function(Box::new(FunctionCall {
            name: "warning".to_string(),
            parameters: vec!()
        }))
        .to_string(),
        "$(warning)"
    );
}
