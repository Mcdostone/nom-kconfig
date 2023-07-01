use crate::{
    assert_parsing_eq,
    attribute::imply::{parse_imply, Imply},
    symbol::Symbol,
};

#[test]
fn test_parse_imply() {
    let input = "imply PCI";
    assert_parsing_eq!(
        parse_imply,
        input,
        Ok((
            "",
            Imply {
                symbol: Symbol::Constant("PCI".to_string()),
                r#if: None
            }
        ))
    )
}
