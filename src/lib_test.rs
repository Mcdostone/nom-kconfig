#[macro_export]
macro_rules! assert_parsing_eq {
    ($fn:ident, $input:expr, $expected:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra($input, Default::default()))
            .map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}

#[macro_export]
macro_rules! assert_parsing_source_eq {
    ($fn:ident, $input:expr, $silent_fail: expr,  $expected:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra(
            $input,
            KconfigFile {
                fail_on_missing_source: $silent_fail,
                ..Default::default()
            },
        ))
        .map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}
