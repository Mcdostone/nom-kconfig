use crate::{
    assert_parsing_eq, assert_parsing_fail,
    entry::{parse_choice, Choice},
    Attribute,
};

#[test]
fn test_parse_choice_optional() {
    assert_parsing_eq!(
        parse_choice,
        "choice optional endchoice",
        Ok((
            "",
            Choice {
                options: vec!(Attribute::Optional),
                configs: vec!()
            }
        ))
    )
}

#[test]
fn test_parse_choice_string_not_allowed() {
    assert_parsing_fail!(
        parse_choice,
        r#"choice 
        config TESTCHOICE1
    string "Choice 1"
        endchoice"#
    )
}
