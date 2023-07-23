use std::path::PathBuf;

use nom_kconfig::{
    attribute::{
        expression::{AndExpression, Atom, Expression, OrExpression, Term},
        r#type::{EntryType, Type},
        select::Select,
    },
    entry::config::Config,
    kconfig::parse_kconfig,
    symbol::Symbol,
    Attribute, Entry, Kconfig, KconfigFile, KconfigInput,
};

#[test]
fn test_parse() {
    let input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("Kconfig");
    let kconfig_file: KconfigFile = KconfigFile::new(
        input_file.parent().unwrap().to_path_buf(),
        input_file.clone(),
    );

    let content = kconfig_file.read_to_string().unwrap();
    let input = KconfigInput::new_extra(&content, kconfig_file);
    let result = parse_kconfig(input);

    assert_parsing!(result, Ok(("", Kconfig { file: input_file.display().to_string(), entries: vec!(Entry::Config(
        Config {
            symbol: "KVM".to_string(),
            attributes: vec!(
                Attribute::Type(EntryType {
                r#type: Type::Tristate,
                prompt: Some("Kernel-based Virtual Machine (KVM) support".to_string()),
                r#if: None
                }),
                Attribute::DependsOn(Expression(OrExpression::Term(AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol(Symbol::Constant("HAVE_KVM".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("HIGH_RES_TIMERS".to_string()))),
                    Term::Atom(Atom::Symbol(Symbol::Constant("X86_LOCAL_APIC".to_string())))
                ))))),
                Attribute::Select(Select { symbol: "PREEMPT_NOTIFIERS".to_string(), r#if: None }),
                Attribute::Select(Select { symbol: "MMU_NOTIFIER".to_string(), r#if: None }),
                Attribute::Select(Select { symbol: "HAVE_KVM_IRQCHIP".to_string(), r#if: None }),
                Attribute::Select(Select { symbol: "HAVE_KVM_PFNCACHE".to_string(), r#if: None }),
                Attribute::Help("Support hosting fully virtualized guest machines using hardware\nvirtualization extensions.  You will need a fairly recent\nprocessor equipped with virtualization extensions. You will also\nneed to select one or more of the processor modules below.".to_string()),
            )
        }))})))
}

#[macro_export]
macro_rules! assert_parsing {
    ($result:expr, $expected:expr) => {{
        let res = $result.map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}
