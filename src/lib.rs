//! # Scheduler for the [Rust] programming language.
//! 
//! The goal of this crate is to provide a simple and efficient way to schedule tasks. The need came from a necessity to schedule tasks at a specific time in a project I'm working on at work. This crate allows you to create a schedule using the classic cron syntax, and then provides many useful methods like :
//! - `next_occurrence` : returns the next occurrence of the schedule after a given date
//! - `previous_occurrence` : returns the previous occurrence of the schedule before a given date
//! - `matches` : returns true if the schedule matches a given date
//! and many more.
//! 
//! ## How to use it
//! 
//! ## How the project is structured
//! ### lib.rs
//! ### time_extensions.rs
//! ### error.rs
//! ### test.rs

// use the time crate for timekeeping
use time::OffsetDateTime;

// extend the OffsetDateTime struct with useful methods
mod time_extension;
use time_extension::TimeExtension;

// load custom error types
mod error;
use error::{ParsingError};

/// A schedule is composed of 4 fields:
/// - `months`: a list of months (1-12)
/// - `days`: a list of days (1-31)
/// - `hours`: a list of hours (0-23)
/// - `minutes`: a list of minutes (0-59)
/// 
/// Each field is a bitfield, where each bit represents a month/day/hour/minute.
/// The order is the following : ... 4 3 2 1 0 (example for minutes). This means that increasing the value of the field will increase the time. This technique allows to encode a schedule in a small enough space.
#[derive(Debug, Clone, Copy)]
pub struct Schedule {
    months: u64,        // 1-12
    days_of_month: u64, // 1-31
    hours: u64,         // 0-23
    minutes: u64,       // 0-59
}

impl Schedule {
    /// create a new schedule from a cron string
    /// 
    /// # Arguments
    /// - `expression`: a string representing the schedule
    /// 
    /// # Examples
    /// ```
    /// use scheduler::Schedule;
    /// 
    /// fn main() {
    ///     let schedule = Schedule::new("0 * * * *").unwrap();
    ///     assert_eq!("", schedule.get_next_match());
    /// }
    /// ```
    pub fn new(expression: &str) -> Result<Self, ParsingError> {
        // todo : verify that the expression is valid

        // split the expression into 5 components (minutes, hours, month-days, months, and week-days)
        let component: Vec<&str> = expression.split(" ").collect();

        // if there are not 5 components, the expression is invalid
        if component.len() != 5 {
            return Err(ParsingError::InvalidExpressionLength {
                expression: expression.to_owned(),
                expected: 5,
                got: component.len(),
            });
        }

        // define an inline function that converts a string to a bitfield
        #[inline(always)]
        fn parse_component<const N: usize>(expression: &str, component: &str) -> Result<u64, ParsingError> {
            // if the component is a wildcard, return a bitfield with bits set to 1
            let mut bitfield = 0;

            // for each component, set the corresponding bit to 1
            for sub_component in component.split(",") {
                // if component if empty (e.g. ",,,"), throw an error
                if sub_component.is_empty() {
                    return Err(ParsingError::InvalidList {
                        expression: expression.to_owned(),
                        list: component.to_owned(),
                    });
                }

                // if wildcard, set all bits to 1, and return (no need to continue)
                if sub_component == "*" {
                    for i in 0..N {
                        bitfield |= 1 << i;
                    }
                    break;
                }

                // if the component is a range, set the bits in the range to 1
                else if sub_component.contains("-") {
                    // split the range into 2 components : the start and the end
                    let range: Vec<&str> = sub_component.split("-").collect();

                    // if there are not 2 components, the range is invalid
                    if range.len() != 2 {
                        return Err(ParsingError::InvalidRange {
                            expression: expression.to_owned(),
                            range: sub_component.to_owned(),
                        });
                    }

                    // parse the start and end of the range
                    let start: usize = match range[0].parse() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(ParsingError::InvalidNumber {
                                expression: expression.to_owned(),
                                number: range[0].to_owned(),
                            });
                        }
                    };
                    let end: usize = match range[1].parse() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(ParsingError::InvalidNumber {
                                expression: expression.to_owned(),
                                number: range[1].to_owned(),
                            });
                        }
                    };

                    // if the start is greater than the end, the range is invalid
                    // if the end is greater than the maximum value, the range is invalid
                    if (start > end) || (end >= N) {
                        return Err(ParsingError::InvalidRange {
                            expression: expression.to_owned(),
                            range: sub_component.to_owned(),
                        });
                    }

                    // if the range is valid, set the bits in the range to 1
                    for i in start..=end {
                        bitfield |= 1 << i;
                    }
                }
                
                // if the component is selecting using modulo
                else if sub_component.contains("/") {
                    // split the modulo into 2 components : the start and the end
                    let range: Vec<&str> = sub_component.split("/").collect();

                    // if there are not 2 components, the modulo is invalid
                    if range.len() != 2 {
                        return Err(ParsingError::InvalidModulo {
                            expression: expression.to_owned(),
                            modulo: sub_component.to_owned(),
                        });
                    }

                    // if the start if not a wildcard, the modulo is invalid
                    if range[0] != "*" {
                        return Err(ParsingError::InvalidModulo {
                            expression: expression.to_owned(),
                            modulo: sub_component.to_owned(),
                        });
                    }

                    // parse the modulo
                    let modulo: usize = match range[1].parse() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(ParsingError::InvalidNumber {
                                expression: expression.to_owned(),
                                number: range[1].to_owned(),
                            });
                        }
                    };

                    // if the modulo is correct, set the bits in the range to 1
                    for i in 0..N {
                        if i % modulo == 0 {
                            bitfield |= 1 << i;
                        }
                    }
                }
                
                // if the component is a single number
                else {
                    // try parsing the number
                    let index: usize = match sub_component.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(ParsingError::InvalidNumber {
                                expression: expression.to_owned(),
                                number: sub_component.to_owned(),
                            });
                        }
                    };

                    // if the number is greater than the maximum value, the number is invalid
                    if index >= N {
                        return Err(ParsingError::InvalidNumber {
                            expression: expression.to_owned(),
                            number: sub_component.to_owned(),
                        });
                    }

                    // set the corresponding bit to 1
                    bitfield |= 1 << index;
                }
            }

            // return the bitfield
            Ok(bitfield)
        }

        let months = parse_component::<12>(expression, component[3])?;
        let days_of_month = parse_component::<31>(expression, component[2])?;
        let hours = parse_component::<24>(expression, component[1])?;
        let minutes = parse_component::<60>(expression, component[0])?;

        Ok(Self {
            months,
            days_of_month,
            hours,
            minutes,
        })
    }

    pub fn get_next_match(&self, mut date: OffsetDateTime) -> OffsetDateTime {
        date.next_minute();

        #[inline(always)]
        fn is_month_valid(date: &mut OffsetDateTime, valid_months: u64) -> bool {
            (valid_months & (1 << date.month() as usize)) != 0
        }
        #[inline(always)]
        fn is_day_of_month_valid(date: &mut OffsetDateTime, valid_days_of_month: u64) -> bool {
            (valid_days_of_month & (1 << date.day() as usize)) != 0
        }
        #[inline(always)]
        fn is_hour_valid(date: &mut OffsetDateTime, valid_hours: u64) -> bool {
            (valid_hours & (1 << date.hour() as usize)) != 0
        }
        #[inline(always)]
        fn is_minute_valid(date: &mut OffsetDateTime, valid_minutes: u64) -> bool {
            (valid_minutes & (1 << date.minute() as usize)) != 0
        }

        // returns next valid month, and true if there is a loop
        #[inline(always)]
        fn get_next_month(date: &mut OffsetDateTime, valid_month: u64) {
            loop {
                date.next_month();
                if is_month_valid(date, valid_month) {
                    break;
                }
            }
        }
        // returns next valid day of month, and true if there is a loop
        #[inline(always)]
        fn get_next_day_of_month(date: &mut OffsetDateTime, valid_days_of_month: u64) -> bool {
            let mut looped;

            loop {
                looped = date.next_day();
                if looped || is_day_of_month_valid(date, valid_days_of_month) {
                    break;
                }
            }

            looped
        }
        #[inline(always)]
        fn get_next_hour(date: &mut OffsetDateTime, valid_hours: u64) -> bool {
            let mut looped;

            loop {
                looped = date.next_hour();
                if looped || is_hour_valid(date, valid_hours) {
                    break;
                }
            }

            looped
        }
        #[inline(always)]
        fn get_next_minute(date: &mut OffsetDateTime, valid_minutes: u64) -> bool {
            let mut looped;

            loop {
                looped = date.next_minute();
                if looped || is_minute_valid(date, valid_minutes) {
                    break;
                }
            }

            looped
        }

        // not very pretty, but I couldn't find a way to make the code cleaner
        // the execution is efficient though
        loop {
            if is_month_valid(&mut date, self.months) {
                loop {
                    if is_day_of_month_valid(&mut date, self.days_of_month) {
                        loop {
                            if is_hour_valid(&mut date, self.hours) {
                                loop {
                                    if is_minute_valid(&mut date, self.minutes) {
                                        return date;
                                    }
                                    if get_next_minute(&mut date, self.minutes)
                                        && !is_hour_valid(&mut date, self.hours)
                                    {
                                        break;
                                    }
                                }
                            }
                            if get_next_hour(&mut date, self.hours)
                                && !is_day_of_month_valid(&mut date, self.days_of_month)
                            {
                                break;
                            }
                        }
                    }
                    if get_next_day_of_month(&mut date, self.days_of_month)
                        && !is_month_valid(&mut date, self.months)
                    {
                        break;
                    }
                }
            }
            get_next_month(&mut date, self.months);
        }
    }
}

/// Test if the expression parsing works
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test if the expression parsing works.
    fn parsing() {
        let schedule = Schedule::new("0 15-18 5 2-4 *");
        // let mut now = OffsetDateTime::from();

    }

    #[test]
    /// Test that checks if the next match is correct.
    fn expression_1() {
        let schedule = Schedule::new("0 15-18 5 2-4 *");
        let mut now = OffsetDateTime::now_utc();
    }

    #[test]
    /// Test that checks if the next match is correct.
    fn expression_2() {
    }

    #[test]
    /// Test that checks if multipe matches are correct.
    fn expression_multi_1() {
    }
}
