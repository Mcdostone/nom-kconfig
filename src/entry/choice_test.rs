use crate::{
    assert_parsing_eq,
    attribute::Attribute,
    entry::choice::{parse_choice, Choice},
};

#[test]
fn test_parse_choice_optioanl() {
    let input = "choice optional endchoice";
    assert_parsing_eq!(
        parse_choice,
        input,
        Ok((
            "",
            Choice {
                options: vec!(Attribute::Optional),
                blocks: vec!()
            }
        ))
    )
}
