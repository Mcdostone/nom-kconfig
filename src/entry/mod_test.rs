use crate::{
    assert_parsing_eq,
    attribute::{
        r#type::{ConfigType, Type},
        AndExpression, Atom, DefaultAttribute, Expression, Term,
    },
    entry::{parse_entries, Comment, Config},
    Attribute, Entry, Symbol,
};

#[test]
fn test_parse_entries() {
    let input = r#"config KVM
        bool
        comment "some configs""#;
    assert_parsing_eq!(
        parse_entries,
        input,
        Ok((
            "",
            vec!(
                Entry::Config(Config {
                    symbol: "KVM".to_string(),
                    attributes: vec!(Attribute::Type(ConfigType {
                        r#type: Type::Bool(None),
                        r#if: None
                    }))
                }),
                Entry::Comment(Comment {
                    prompt: "some configs".to_string(),
                    dependencies: vec!()
                }),
            )
        ))
    )
}

#[test]
fn test_double_indented_entries() {
    let input = r#"mainmenu "MAIN"

    config A
        bool
        help
            - Lorem ipsum dolor sit amet, consetetur sadipscing elitr.
                - Lorem ipsum dolor sit amet, consetetur sadipscing elitr.

    config B
        bool
"#;
    assert_parsing_eq!(
        parse_entries,
        input,
        Ok((
            "",
            vec!(
                Entry::MainMenu(crate::entry::MainMenu {
                    prompt: "MAIN".to_string()
                }),
                Entry::Config(Config {
                    symbol: "A".to_string(),
                    attributes: vec!(
                        Attribute::Type(ConfigType {
                            r#type: Type::Bool(None),
                            r#if: None
                        }),
                        Attribute::Help(
                            "- Lorem ipsum dolor sit amet, consetetur sadipscing elitr.\n    - Lorem ipsum dolor sit amet, consetetur sadipscing elitr.".to_string()
                        )
                    )
                }),
                Entry::Config(Config {
                    symbol: "B".to_string(),
                    attributes: vec![Attribute::Type(ConfigType {
                        r#type: Type::Bool(None),
                        r#if: None
                    })]
                })
            )
        ))
    )
}

#[test]
fn test_issue() {
    let input = r#"    
config COMPACT_UNEVICTABLE_DEFAULT
	int
	depends on COMPACTION
	default 0 if PREEMPT_RT
	default 1
"#;
    assert_parsing_eq!(
        parse_entries,
        input,
        Ok((
            "",
            vec!(Entry::Config(Config {
                symbol: "COMPACT_UNEVICTABLE_DEFAULT".to_string(),
                attributes: vec!(
                    Attribute::Type(ConfigType {
                        r#type: Type::Int(None),
                        r#if: None
                    }),
                    Attribute::DependsOn(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Symbol(Symbol::NonConstant("COMPACTION".to_string()))
                    )))),
                    Attribute::Default(DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Number(0)
                        ))),
                        r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Symbol(Symbol::NonConstant("PREEMPT_RT".to_string()))
                        ))))
                    }),
                    Attribute::Default(DefaultAttribute {
                        expression: Expression::Term(AndExpression::Term(Term::Atom(
                            Atom::Number(1)
                        ))),
                        r#if: None
                    })
                )
            }),)
        ))
    )
}

//#[test]
//fn test_issue_2() {
//    let input = r#"
//config ZRAM_DEF_COMP
//	string
//	depends on ZRAM
//	default "lzo-rle" if ZRAM_DEF_COMP_LZORLE
//	default "lzo" if ZRAM_DEF_COMP_LZO
//	default "lz4" if ZRAM_DEF_COMP_LZ4
//	default "lz4hc" if ZRAM_DEF_COMP_LZ4HC
//	default "zstd" if ZRAM_DEF_COMP_ZSTD
//	default "deflate" if ZRAM_DEF_COMP_DEFLATE
//	default "842" if ZRAM_DEF_COMP_842
//	default "unset-value"
//"#;
//    assert_parsing_eq!(
//        parse_entries,
//        input,
//        Ok((
//            "",
//            vec!(
//                Entry::Config(Config {
//                    symbol: "ZRAM_DEF_COMP".to_string(),
//                    attributes: vec!(
//                        Attribute::Type(ConfigType {
//                            r#type: Type::String(None),
//                            r#if: None
//                        }),
//                        Attribute::DependsOn(
//                            Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(
//                                Symbol::NonConstant("ZRAM".to_string())
//                            ))))
//                        ),
//
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("lzo-rle".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_LZORLE".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("lzo".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_LZO".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("lz4".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_LZ4".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("lz4hc".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_LZ4HC".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("zstd".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_ZSTD".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("deflate".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_DEFLATE".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("842".to_string()))))),
//                            r#if: Some(Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::NonConstant("ZRAM_DEF_COMP_842".to_string()))))))
//                        }),
//                        Attribute::Default(DefaultAttribute {
//                            expression: Expression::Term(AndExpression::Term(Term::Atom(Atom::Symbol(Symbol::Constant("unset-value".to_string()))))),
//                            r#if: None
//                        })
//                    )
//                }),
//            )
//        ))
//    )
//}
//
