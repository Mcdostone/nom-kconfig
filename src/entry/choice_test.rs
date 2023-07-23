use crate::{
    assert_parsing_eq,
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
                blocks: vec!()
            }
        ))
    )
}
