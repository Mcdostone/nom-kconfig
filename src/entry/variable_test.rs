use crate::{
    assert_parsing_eq,
    attribute::function::ExpressionToken,
    entry::variable::{parse_variable_assignment, Value, VariableAssignment, VariableIdentifier},
};

#[test]
fn test_parse_variable() {
    let input = "hello = world";
    assert_parsing_eq!(
        parse_variable_assignment,
        input,
        Ok((
            "",
            VariableAssignment {
                identifier: VariableIdentifier::Identifier("hello".to_string()),
                operator: "=".to_string(),
                right: Value::Literal("world".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_variable_variables_lefthand_side() {
    let input = "$(X)$(Y) := 5";
    assert_parsing_eq!(
        parse_variable_assignment,
        input,
        Ok((
            "",
            VariableAssignment {
                identifier: VariableIdentifier::VariableRef(vec!(
                    ExpressionToken::Variable("X".to_string()),
                    ExpressionToken::Variable("Y".to_string())
                )),
                operator: ":=".to_string(),
                right: Value::Literal("5".to_string())
            }
        ))
    )
}

// 5.4.231/scripts/Kconfig.include
#[test]
fn test_parse_variable_comma() {
    let input = "comma       := ,";
    assert_parsing_eq!(
        parse_variable_assignment,
        input,
        Ok((
            "",
            VariableAssignment {
                identifier: VariableIdentifier::Identifier("comma".to_string()),
                operator: ":=".to_string(),
                right: Value::Literal(",".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_variable_space() {
    let input = "space       := $(empty) $(empty)";
    assert_parsing_eq!(
        parse_variable_assignment,
        input,
        Ok((
            "",
            VariableAssignment {
                identifier: VariableIdentifier::Identifier("space".to_string()),
                operator: ":=".to_string(),
                right: Value::Literal("$(empty) $(empty)".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_variable_if_success() {
    let input = r#"if-success = $(shell,{ $(1); } >/dev/null 2>&1 && echo "$(2)" || echo "$(3)")"#;
    assert_parsing_eq!(
        parse_variable_assignment,
        input,
        Ok((
            "",
            VariableAssignment {
                identifier: VariableIdentifier::Identifier("if-success".to_string()),
                operator: "=".to_string(),
                right: Value::Literal(
                    r#"$(shell,{ $(1); } >/dev/null 2>&1 && echo "$(2)" || echo "$(3)")"#
                        .to_string()
                )
            }
        ))
    )
}
