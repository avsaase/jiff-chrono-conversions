use jiff_chrono_conversions::{ToChrono, ToJiff, TryToChrono, TryToJiff};

fn main() {
    // Date: infallible `jiff` -> `chrono`, fallible `chrono` -> `jiff`.
    let jiff_date = jiff::civil::date(2024, 3, 15);
    let chrono_date = jiff_date.to_chrono();
    println!("date: jiff {jiff_date} -> chrono {chrono_date}");
    let jiff_date: jiff::civil::Date = chrono_date.try_to_jiff().unwrap();
    println!("date: chrono {chrono_date} -> jiff {jiff_date}");

    // Time: both directions are infallible.
    let jiff_time = jiff::civil::time(13, 30, 0, 0);
    let chrono_time = jiff_time.to_chrono();
    println!("time: jiff {jiff_time} -> chrono {chrono_time}");
    let jiff_time: jiff::civil::Time = chrono_time.to_jiff();
    println!("time: chrono {chrono_time} -> jiff {jiff_time}");

    // DateTime: infallible `jiff` -> `chrono`, fallible `chrono` -> `jiff`.
    let jiff_datetime = jiff::civil::datetime(2024, 3, 15, 13, 30, 0, 0);
    let chrono_datetime = jiff_datetime.to_chrono();
    println!("datetime: jiff {jiff_datetime} -> chrono {chrono_datetime}");
    let jiff_datetime: jiff::civil::DateTime = chrono_datetime.try_to_jiff().unwrap();
    println!("datetime: chrono {chrono_datetime} -> jiff {jiff_datetime}");

    // Timestamp: infallible `jiff` -> `chrono`, fallible `chrono` -> `jiff`.
    let jiff_timestamp = jiff::Timestamp::now();
    let chrono_utc_datetime = jiff_timestamp.to_chrono();
    println!("timestamp: jiff {jiff_timestamp} -> chrono {chrono_utc_datetime}");
    let jiff_timestamp: jiff::Timestamp = chrono_utc_datetime.try_to_jiff().unwrap();
    println!("timestamp: chrono {chrono_utc_datetime} -> jiff {jiff_timestamp}");

    // Offset: infallible `chrono` -> `jiff`, fallible `jiff` -> `chrono`.
    let chrono_offset = chrono::FixedOffset::east_opt(3600).unwrap();
    let jiff_offset: jiff::tz::Offset = chrono_offset.to_jiff();
    println!("offset: chrono {chrono_offset} -> jiff {jiff_offset}");
    let chrono_offset: chrono::FixedOffset = jiff_offset.try_to_chrono().unwrap();
    println!("offset: jiff {jiff_offset} -> chrono {chrono_offset}");

    // Zoned DateTime: both directions are fallible.
    let jiff_zoned = jiff_timestamp.to_zoned(jiff::tz::TimeZone::fixed(jiff_offset));
    let chrono_fixed_datetime: chrono::DateTime<chrono::FixedOffset> =
        jiff_zoned.try_to_chrono().unwrap();
    println!("zoned datetime: jiff {jiff_zoned} -> chrono {chrono_fixed_datetime}");
    let jiff_zoned: jiff::Zoned = chrono_fixed_datetime.try_to_jiff().unwrap();
    println!("zoned datetime: chrono {chrono_fixed_datetime} -> jiff {jiff_zoned}");

    // Time zone: both directions are fallible.
    let chrono_tz = chrono_tz::Tz::Europe__Amsterdam;
    let jiff_tz: jiff::tz::TimeZone = chrono_tz.try_to_jiff().unwrap();
    println!("time zone: chrono {chrono_tz} -> jiff {}", jiff_tz.iana_name().unwrap());
    let chrono_tz: chrono_tz::Tz = jiff_tz.try_to_chrono().unwrap();
    println!("time zone: jiff {} -> chrono {chrono_tz}", jiff_tz.iana_name().unwrap());
}
