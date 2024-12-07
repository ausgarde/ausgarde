/// Converts a Readable timespan like `1h5m` to a `chrono::Duration`.
/// It accepts the format `ddhhmmss`
///
/// # Usage
/// ```no_run
/// let dur = timespan!(1h5m);
/// ```
#[macro_export]
macro_rules! timespan {
    ($($val:tt)*) => {{
        let mut duration = chrono::Duration::zero();
        let mut current = String::new();

        for c in stringify!($($val)*).chars() {
            match c {
                'd' => {
                    if let Ok(n) = current.parse::<i64>() {
                        duration += chrono::Duration::days(n)
                    }
                    current.clear();
                }
                'h' => {
                    if let Ok(n) = current.parse::<i64>() {
                        duration += chrono::Duration::hours(n)
                    }
                    current.clear();
                }
                'm' => {
                    if let Ok(n) = current.parse::<i64>() {
                        duration += chrono::Duration::minutes(n)
                    }
                    current.clear();
                }
                's' => {
                    if let Ok(n) = current.parse::<i64>() {
                        duration += chrono::Duration::seconds(n)
                    }
                    current.clear();
                }
                _ if c.is_digit(10) => current.push(c),
                _ => panic!("invalid time interval"),
            }
        }
        duration
    }};
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    #[test]
    fn timespan() {
        assert_eq!(timespan!(5m), Duration::minutes(5));
        assert_eq!(timespan!(1h), Duration::hours(1));

        assert_eq!(timespan!(3h30m), Duration::hours(3) + Duration::minutes(30));
        assert_eq!(
            timespan!(1d5h30m),
            Duration::days(1) + Duration::hours(5) + Duration::minutes(30)
        );
    }
}
