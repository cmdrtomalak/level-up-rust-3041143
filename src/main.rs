use chrono::NaiveDate;
use std::collections::HashMap;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`. 
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let months = HashMap::from([("Jan", "01+"),
                                                    ("Feb", "02+"),
                                                    ("Mar", "03+"),
                                                    ("Apr", "04+"),
                                                    ("May", "05+"),
                                                    ("Jun", "06+"),
                                                    ("Jul", "07+"),
                                                    ("Aug", "08+"),
                                                    ("Sep", "09+"),
                                                    ("Oct", "10+"),
                                                    ("Nov", "11+"),
                                                    ("Dec", "12+")]);
    
    let mut input = text.replace('.', "-");
    input = input.replace('/', "-");

    for (month_s, month_n) in months {
        input = input.replace(month_s, month_n);
    }

    if let Ok(date) = NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
        return Some(date);
    }

    if let Ok(date) = NaiveDate::parse_from_str(&input, "%Y-%m+-%d") {
        return Some(date);
    }

    if let Ok(date) = NaiveDate::parse_from_str(&input, "%m+-%d-%Y") {
        return Some(date);
    }

    if let Ok(date) = NaiveDate::parse_from_str(&input, "%d-%m+-%Y") {
        return Some(date);
    }

    None
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
}

#[test]
fn dmy_dot() {
    assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
}

#[test]
fn mdy_dot() {
    assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}


