use crate::{
    assert_parsing_eq,
    attribute::imply::{parse_imply, Imply},
    symbol::Symbol,
};

#[test]
fn test_parse_imply() {
    assert_parsing_eq!(
        parse_imply,
        "imply PCI",
        Ok((
            "",
            Imply {
                symbol: Symbol::Constant("PCI".to_string()),
                r#if: None
            }
        ))
    )
}
