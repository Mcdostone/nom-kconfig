use crate::{
    assert_parsing_eq,
    attribute::function::{parse_function_call, ExpressionToken, FunctionCall, Parameter},
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
    let input = "$(greeting,Hello,John)";
    assert_parsing_eq!(
        parse_function_call,
        input,
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
    let input = "$(warning,SIMPLE = basic)";
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
fn test_parse_function_call_special_backtik() {
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
