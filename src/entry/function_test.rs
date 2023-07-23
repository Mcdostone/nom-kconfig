use crate::{
    assert_parsing_eq,
    entry::{parse_function, Function},
};

#[test]
fn test_parse_function() {
    assert_parsing_eq!(
        parse_function,
        "greeting = $(1), my name is $(2).\n",
        Ok((
            "",
            Function {
                name: "greeting".to_string(),
                body: "$(1), my name is $(2).".to_string()
            }
        ))
    )
}
