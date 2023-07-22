use crate::{assert_parsing_eq, attribute::help::parse_help};

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

// 3.2/drivers/net/ethernet/stmicro/stmmac/Kconfig
#[test]
fn test_parse_help_prefixed_by_hypen() {
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
    <http://www.alphalinux.org/>.
";
    assert_parsing_eq!(
        parse_help,
        input,
        Ok((
            "",
            "The Alpha is a 64-bit general-purpose processor designed and\nmarketed by the Digital Equipment Corporation of blessed memory, now\nCompaq.  Alpha Linux dates from 1995-1996 and was the first non-x86\nport. The Alpha Linux project has a home page at\n<http://www.alphalinux.org/>.".to_string()
        ))
    )
}
