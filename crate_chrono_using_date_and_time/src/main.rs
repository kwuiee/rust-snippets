extern crate chrono;
extern crate time;

use chrono::offset::LocalResult;
use chrono::prelude::*;
use time::Duration;

fn main() {
    // get the current date and time in the UTC time zone (Utc::now()) or in the local time zone
    // (Local::now())
    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

    // ** create your own date and time **
    let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
                                                     // July 8 is 188th day of the year 2014 (`o` for "ordinal")
    assert_eq!(dt, Utc.yo(2014, 189).and_hms(9, 10, 11));
    // July 8 is Tuesday in ISO week 28 of the year 2014.
    assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms(9, 10, 11));

    let dt = Utc.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12); // `2014-07-08T09:10:11.012Z`
    assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000));
    assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000));

    // dynamic verification
    assert_eq!(
        Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
        LocalResult::Single(Utc.ymd(2014, 7, 8).and_hms(21, 15, 33))
    );
    assert_eq!(
        Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33),
        LocalResult::None
    );
    assert_eq!(
        Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33),
        LocalResult::None
    );

    // other time zone objects can be used to construct a local datetime.
    // obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
    let local_dt = Local.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12);
    let fixed_dt = FixedOffset::east(9 * 3600)
        .ymd(2014, 7, 8)
        .and_hms_milli(18, 10, 11, 12);
    assert_eq!(dt, fixed_dt);

    // ** Various properties are available to the date and time **
    // assume this returned `2014-11-28T21:45:59.324310806+09:00`:
    let dt = FixedOffset::east(9 * 3600)
        .ymd(2014, 11, 28)
        .and_hms_nano(21, 45, 59, 324310806);

    // property accessors
    assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
    assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
    assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
    assert_eq!(dt.weekday(), Weekday::Fri);
    assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7
    assert_eq!(dt.ordinal(), 332); // the day of year
    assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

    // time zone accessor and manipulation
    assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
    assert_eq!(dt.timezone(), FixedOffset::east(9 * 3600));
    assert_eq!(
        dt.with_timezone(&Utc),
        Utc.ymd(2014, 11, 28).and_hms_nano(12, 45, 59, 324310806)
    );

    // a sample of property manipulations (validates dynamically)
    assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
    assert_eq!(dt.with_day(32), None);
    assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

    // arithmetic operations
    let dt1 = Utc.ymd(2014, 11, 14).and_hms(8, 9, 10);
    let dt2 = Utc.ymd(2014, 11, 14).and_hms(10, 9, 8);
    assert_eq!(
        dt1.signed_duration_since(dt2),
        Duration::seconds(-2 * 3600 + 2)
    );
    assert_eq!(
        dt2.signed_duration_since(dt1),
        Duration::seconds(2 * 3600 - 2)
    );
    assert_eq!(
        Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) + Duration::seconds(1_000_000_000),
        Utc.ymd(2001, 9, 9).and_hms(1, 46, 40)
    );
    assert_eq!(
        Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) - Duration::seconds(1_000_000_000),
        Utc.ymd(1938, 4, 24).and_hms(22, 13, 20)
    );

    // ** formatting **
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2014-11-28 12:00:09"
    );
    assert_eq!(
        dt.format("%a %b %e %T %Y").to_string(),
        "Fri Nov 28 12:00:09 2014"
    );
    assert_eq!(
        dt.format("%a %b %e %T %Y").to_string(),
        dt.format("%c").to_string()
    );

    assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
    assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
    assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
    assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");

    // Note that milli/nanoseconds are only printed if they are non-zero
    let dt_nano = Utc.ymd(2014, 11, 28).and_hms_nano(12, 0, 9, 1);
    assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");

    // ** parseing **
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9 * 3600));

    // method 1
    assert_eq!(
        "2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(),
        Ok(fixed_dt.clone())
    );

    // method 2
    assert_eq!(
        DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
        Ok(fixed_dt.clone())
    );
    assert_eq!(
        DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
        Ok(fixed_dt.clone())
    );
    assert_eq!(
        DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"),
        Ok(fixed_dt.clone())
    );

    // method 3
    assert_eq!(
        Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"),
        Ok(dt.clone())
    );
    assert_eq!(
        Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"),
        Ok(dt.clone())
    );

    // oops, the year is missing!
    assert!(Utc
        .datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y")
        .is_err());
    // oops, the format string does not include the year at all!
    assert!(Utc
        .datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T")
        .is_err());
    // oops, the weekday is incorrect!
    assert!(Utc
        .datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y")
        .is_err());
}
