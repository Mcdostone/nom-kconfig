use crate::{assert_parsing_eq, attribute::parse_help};

#[test]
fn test_parse_help() {
    let input = "help\n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}

#[test]
fn test_parse_help_space() {
    let input = "help   \n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}

#[test]
fn test_parse_help_no_indent() {
    let input = "help\nhello world";
    assert_parsing_eq!(parse_help, input, Ok(("hello world", "".to_string())))
}

#[test]
fn test_parse_help_no_content() {
    let input = "help\n";
    assert_parsing_eq!(parse_help, input, Ok(("", "".to_string())))
}

// 3.2/drivers/net/ethernet/stmicro/stmmac/Kconfig
#[test]
fn test_parse_help_prefixed_by_hyphen() {
    let input = "-- help\n hello world";
    assert_parsing_eq!(parse_help, input, Ok(("", "hello world".to_string())))
}

// 2.5.45/drivers/mtd/maps/Kconfig
#[test]
fn test_parse_help_encoding() {
    let input = "-- help\n Mapping for the Flaga digital module. If you don�t have one, ignore this setting.";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "Mapping for the Flaga digital module. If you don�t have one, ignore this setting."
                .to_string()
        ))
    )
}

#[test]
fn test_parse_help_indent() {
    let input = "	---help---
	This driver supports all of Adaptec's Fast through Ultra 160 PCI
	based SCSI controllers as well as the aic7770 based EISA and VLB
	SCSI controllers (the 274x and 284x series).";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "This driver supports all of Adaptec's Fast through Ultra 160 PCI\nbased SCSI controllers as well as the aic7770 based EISA and VLB\nSCSI controllers (the 274x and 284x series).".to_string()
        ))
    )
}

#[test]
fn test_parse_help_indent_2() {
    let input = "	help
    The Alpha is a 64-bit general-purpose processor designed and
    marketed by the Digital Equipment Corporation of blessed memory, now
    Compaq.  Alpha Linux dates from 1995-1996 and was the first non-x86
    port. The Alpha Linux project has a home page at
    <http://www.alphalinux.org/>.";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "The Alpha is a 64-bit general-purpose processor designed and\nmarketed by the Digital Equipment Corporation of blessed memory, now\nCompaq.  Alpha Linux dates from 1995-1996 and was the first non-x86\nport. The Alpha Linux project has a home page at\n<http://www.alphalinux.org/>.".to_string()
        ))
    )
}

/*
#[test]
fn test_parse_help_not_indentation() {
    let input = "	help
The Alpha is a 64-bit general-purpose processor.";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "The Alpha is a 64-bit general-purpose processor designed and\nmarketed by the Digital Equipment Corporation of blessed memory, now\nCompaq.  Alpha Linux dates from 1995-1996 and was the first non-x86\nport. The Alpha Linux project has a home page at\n<http://www.alphalinux.org/>.".to_string()
        ))
    )
}
*/

#[test]
fn test_parse_help_indentation_preservation() {
    let input = "help\n    Lorem Ipsum\n        - Lorem Ipsum\n    Lorem Ipsum\n";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "Lorem Ipsum\n    - Lorem Ipsum\nLorem Ipsum".to_string()
        ))
    )
}

#[test]
fn test_parse_help_double_newline() {
    let input = r"help
      bla 1
        bla 2

      bla 3";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok(("", "bla 1\n  bla 2\n\nbla 3".to_string()))
    )
}

// https://github.com/Mcdostone/nom-kconfig/issues/65
// https://github.com/torvalds/linux/blob/92514ef226f511f2ca1fb1b8752966097518edc0/security/Kconfig#L236-L252
#[test]
fn test_parse_help_paragraph() {
    let input = r#"help
	  This choice is there only for converting CONFIG_DEFAULT_SECURITY
	  in old kernel configs to CONFIG_LSM in new kernel configs. Don't
	  change this choice unless you are creating a fresh kernel config,
	  for this choice will be ignored after CONFIG_LSM has been set.

	  Selects the legacy "major security module" that will be
	  initialized first. Overridden by non-default CONFIG_LSM."#;
    assert_parsing_eq!(
        parse_help,
        input,
        Ok(("", "This choice is there only for converting CONFIG_DEFAULT_SECURITY\nin old kernel configs to CONFIG_LSM in new kernel configs. Don't\nchange this choice unless you are creating a fresh kernel config,\nfor this choice will be ignored after CONFIG_LSM has been set.\n\nSelects the legacy \"major security module\" that will be\ninitialized first. Overridden by non-default CONFIG_LSM.".to_string()))
    )
}

// https://github.com/Mcdostone/nom-kconfig/issues/101
#[test]
fn test_parse_help_inconsistent_identation_with_space_and_tab() {
    let input = r#"
       help
         An architecture selects this if it sorts the mcount_loc section
	 at build time."#;

    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "An architecture selects this if it sorts the mcount_loc section\nat build time."
                .to_string()
        ))
    )
}

// https://github.com/torvalds/linux/blob/master/init/Kconfig#L493-L503
#[test]
fn test_parse_help_first_line_is_empty() {
    let input = r#"help

	  This is a general notification queue for the kernel to pass events to
	  userspace by splicing them into pipes.  It can be used in conjunction
	  with watches for key/keyring change notifications and device
	  notifications.

	  See Documentation/core-api/watch_queue.rst
"#;

    assert_parsing_eq!(
        parse_help,
        input,
        Ok(("", "\nThis is a general notification queue for the kernel to pass events to\nuserspace by splicing them into pipes.  It can be used in conjunction\nwith watches for key/keyring change notifications and device\nnotifications.\n\nSee Documentation/core-api/watch_queue.rst".to_string()))
    )
}

// https://github.com/Mcdostone/nom-kconfig/issues/103
// https://github.com/torvalds/linux/blob/master/drivers/crypto/caam/Kconfig
#[test]
fn test_parse_help_pouet() {
    let input = r#"
	help
	  Select size of Job Rings as a power of 2, within the
	  range 2-9 (ring size 4-512).
	  Examples:
		2 => 4
"#;

    assert_parsing_eq!(
        parse_help,
        input,
        Ok(("",
            "Select size of Job Rings as a power of 2, within the\nrange 2-9 (ring size 4-512).\nExamples:\n2 => 4".to_string()))
    )
}
