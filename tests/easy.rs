extern crate iso8601;
extern crate chrono;
extern crate nom;

use chrono::{FixedOffset,LocalResult,TimeZone};

use nom::IResult::*;

use iso8601::{Date,Time,DateTime};
use iso8601::easy::*;

#[test]
fn easy_parse_date() {
    assert_eq!(Done(&[][..], Date{ year: 2015, month: 6, day: 26 }), date(b"2015-06-26"));
    assert_eq!(Done(&[][..], Date{ year: -333, month: 7, day: 11 }), date(b"-0333-07-11"));

    assert!(date(b"201").is_incomplete());
    assert!(date(b"2015p00p00").is_err());
    assert!(date(b"pppp").is_err());
}

#[test]
fn easy_parse_time() {
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second: 16, tz_offset: 0}), time(b"16:43:16"));
    assert_eq!(Done(&[][..], Time{ hour: 16, minute: 43, second:  0, tz_offset: 0}), time(b"16:43"));

    assert!(time(b"20:").is_incomplete());
    assert!(time(b"20p42p16").is_err());
    assert!(time(b"pppp").is_err());
}

#[test]
fn easy_parse_datetime_correct() {
    fn make_datetime((year, month, day, hour, minute, second, tz_offset): (i32, u32, u32, u32, u32, u32, i32)) -> DateTime {
        DateTime {
            date: Date{ year: year, month: month, day: day },
            time: Time{ hour: hour, minute: minute, second: second, tz_offset: tz_offset*3600 },
        }
    }

    let test_datetimes = vec![
        ("2007-08-31T16:47+00:00",     (2007,  08,  31,  16,  47,  0,   0)),
        ("2007-12-24T18:21Z",          (2007,  12,  24,  18,  21,  0,   0)),
        ("2008-02-01T09:00:22+05",     (2008,  02,  01,  9,   0,   22,  5)),
        ("2009-01-01T12:00:00+01:00",  (2009,  1,   1,   12,  0,   0,   1)),
        ("2009-06-30T18:30:00+02:00",  (2009,  06,  30,  18,  30,  0,   2)),
        ("2015-06-29T23:07+02:00",     (2015,  06,  29,  23,  07,  0,   2)),
        ("2015-06-26T16:43:16",        (2015,  06,  26,  16,  43, 16,   0)),
    ];

    for (iso_string, data) in test_datetimes {
        assert_eq!(Done(&[][..], make_datetime(data)), datetime(iso_string.as_bytes()));
    }
}

#[test]
fn easy_parse_datetime_error() {
    let test_datetimes = vec![
        "ppp",
        "dumd-di-duTmd:iu:m"
    ];

    for iso_string in test_datetimes {
        let res = datetime(iso_string.as_bytes());
        assert!(res.is_err() || res.is_incomplete());
    }
}

#[test]
fn easy_allows_notallowed() {
    assert_eq!(Done(&[][..], Time{ hour: 30, minute: 90, second: 90, tz_offset: 0}), time(b"30:90:90"));
    assert_eq!(Done(&[][..], Date{ year: 0, month: 20, day: 40}), date(b"0000-20-40"));
}

#[test]
fn easy_test_chrono_conversion() {
    let t = datetime(b"0000-20-40T30:90:90Z");
    if let Done(_, dt) = t {
        assert_eq!(LocalResult::None, dt.to_chrono());
    }

    let t = datetime(b"2007-08-31T16:47+00:00");
    if let Done(_, dt) = t {
        let fixed_dt = FixedOffset::east(0).ymd(2007,08,31).and_hms(16,47,0);
        assert_eq!(fixed_dt, dt.to_chrono().unwrap());


    let t = datetime(b"2007-08-31T16:47-09:00");
    if let Done(_, dt) = t {
        let fixed_dt = FixedOffset::east(-9*3600).ymd(2007,08,31).and_hms(16,47,0);
        assert_eq!(fixed_dt, dt.to_chrono().unwrap());
    }   }
}
