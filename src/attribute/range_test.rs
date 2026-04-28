use crate::{
    assert_parsing_eq,
    attribute::{parse_range, range::RangeBound, AndExpression, Atom, Expression, Range, Term},
    symbol::Symbol,
};

#[test]
fn test_parse_range() {
    assert_parsing_eq!(
        parse_range,
        "range 1 5",
        Ok((
            "",
            Range {
                lower_bound: RangeBound::Number(1),
                upper_bound: RangeBound::Number(5),
                r#if: None
            }
        ))
    )
}

#[test]
fn test_parse_range_to_string() {
    assert_eq!(
        Range {
            lower_bound: RangeBound::Number(1),
            upper_bound: RangeBound::Number(5),
            r#if: None
        }
        .to_string(),
        "1 5"
    );

    assert_eq!(
        Range {
            lower_bound: RangeBound::Number(1),
            upper_bound: RangeBound::Number(5),
            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                Atom::Symbol(Symbol::NonConstant("NET".to_string()))
            ))))
        }
        .to_string(),
        "1 5 if NET"
    )
}

// https://github.com/u-boot/u-boot/blob/e2fa3e570f83ab0f9ce667ddaec9dc738bcf05b9/board/ti/common/Kconfig#L15
#[test]
fn test_parse_range_with_hex() {
    assert_parsing_eq!(
        parse_range,
        "range 0 0xff",
        Ok((
            "",
            Range {
                lower_bound: RangeBound::Number(0),
                upper_bound: RangeBound::Hex("0xff".to_string()),
                r#if: None
            }
        ))
    )
}

// https://github.com/torvalds/linux/blob/master/kernel/power/Kconfig#L276-L280
// https://github.com/u-boot/u-boot/blob/master/common/Kconfig#L370-L389
#[test]
fn test_parse_range_symbol() {
    assert_parsing_eq!(
        parse_range,
        "range 0 LOG_MAX_LEVEL",
        Ok((
            "",
            Range {
                lower_bound: RangeBound::Number(0),
                upper_bound: RangeBound::Symbol("LOG_MAX_LEVEL".to_string()),
                r#if: None
            }
        ))
    )
}

// https://github.com/nrfconnect/sdk-zephyr/blob/d947269643dd51e4e95d35d148cecb40562fe0d2/subsys/net/lib/ptp/Kconfig#L198
#[test]
fn test_parse_range_variable() {
    assert_parsing_eq!(
        parse_range,
        "range 0 $(UINT8_MAX)",
        Ok((
            "",
            Range {
                lower_bound: RangeBound::Number(0),
                upper_bound: RangeBound::Variable("UINT8_MAX".to_string()),
                r#if: None
            }
        ))
    )
}
