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
