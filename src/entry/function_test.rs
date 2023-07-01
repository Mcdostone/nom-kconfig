use crate::{
    assert_parsing_eq,
    entry::function::{parse_function, Function},
};

#[test]
fn test_parse_function() {
    let input = "greeting = $(1), my name is $(2).\n";
    assert_parsing_eq!(
        parse_function,
        input,
        Ok((
            "",
            Function {
                name: "greeting".to_string(),
                body: "$(1), my name is $(2).".to_string()
            }
        ))
    )
}
