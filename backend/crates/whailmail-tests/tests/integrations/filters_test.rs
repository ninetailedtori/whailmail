#[test]
fn test_filter_parsing()
{
    let rule = "from:spam@example.com subject:viagra";
    let filter = whailmail_filters::parse_rule(rule).expect("Parse failed");

    assert!(filter.matches_sender("spam@example.com"));
}

#[test]
fn test_filter_application()
{
    let mail = create_test_mail("spam@example.com", "VIAGRA!!!");
    let rule = whailmail_filters::SCompiledFilter::from_spam();

    assert!(rule.expect("REASON").matches(&mail));
}
