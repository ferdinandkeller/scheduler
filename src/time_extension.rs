use time::{util::days_in_year_month, OffsetDateTime};

/// extend the OffsetDateTime struct with useful methods
/// for the crate
pub trait TimeExtension {
    /// set the time to the begining of the current minute
    /// (ns, us, ms, and s are set to 0)
    fn to_start_of_minute(&mut self);

    /// set the time to the begining of the next minute
    fn next_minute(&mut self) -> bool;

    /// set the time to the begining of the current hour
    /// (ns, us, ms, s, and min are set to 0)
    fn to_start_of_hour(&mut self);

    /// set the time to the begining of the next hour
    fn next_hour(&mut self) -> bool;

    /// set the time to the begining of the current day
    /// (ns, us, ms, s, min, and hour are set to 0)
    fn to_start_of_day(&mut self);

    /// set the time to the begining of the next day
    fn next_day(&mut self) -> bool;

    /// set the time to the begining of the current month
    /// (ns, us, ms, s, min, hour, and day are set to 0)
    fn to_start_of_month(&mut self);

    /// set the time to the begining of the next month
    fn next_month(&mut self);
}

/// extend the OffsetDateTime struct with useful methods we defined above
impl TimeExtension for OffsetDateTime {
    #[inline(always)]
    fn to_start_of_minute(&mut self) {
        *self = self.replace_nanosecond(0).expect("invalid nanosecond");
        *self = self.replace_microsecond(0).expect("invalid microsecond");
        *self = self.replace_millisecond(0).expect("invalid millisecond");
        *self = self.replace_second(0).expect("invalid second");
    }

    fn next_minute(&mut self) -> bool {
        self.to_start_of_minute();
        let hour = self.hour();
        *self = self
            .checked_add(time::Duration::MINUTE)
            .expect("time overflow");
        return hour != self.hour();
    }

    fn to_start_of_hour(&mut self) {
        *self = self.replace_nanosecond(0).expect("invalid nanosecond");
        *self = self.replace_microsecond(0).expect("invalid microsecond");
        *self = self.replace_millisecond(0).expect("invalid millisecond");
        *self = self.replace_second(0).expect("invalid second");
        *self = self.replace_minute(0).expect("invalid minute");
    }

    fn next_hour(&mut self) -> bool {
        self.to_start_of_hour();
        let day = self.day();
        *self = self
            .checked_add(time::Duration::HOUR)
            .expect("time overflow");
        return day != self.day();
    }

    fn to_start_of_day(&mut self) {
        *self = self.replace_nanosecond(0).expect("invalid nanosecond");
        *self = self.replace_microsecond(0).expect("invalid microsecond");
        *self = self.replace_millisecond(0).expect("invalid millisecond");
        *self = self.replace_second(0).expect("invalid second");
        *self = self.replace_minute(0).expect("invalid minute");
        *self = self.replace_hour(0).expect("invalid hour");
    }

    fn next_day(&mut self) -> bool {
        self.to_start_of_day();
        let month = self.month();
        *self = self
            .checked_add(time::Duration::DAY)
            .expect("time overflow");
        return month != self.month();
    }

    fn to_start_of_month(&mut self) {
        *self = self.replace_nanosecond(0).expect("invalid nanosecond");
        *self = self.replace_microsecond(0).expect("invalid microsecond");
        *self = self.replace_millisecond(0).expect("invalid millisecond");
        *self = self.replace_second(0).expect("invalid second");
        *self = self.replace_minute(0).expect("invalid minute");
        *self = self.replace_hour(0).expect("invalid hour");
        *self = self.replace_day(1).expect("invalid day");
    }

    fn next_month(&mut self) {
        self.to_start_of_month();
        let days_in_month = days_in_year_month(self.year(), self.month()) as i64;
        *self = self
            .checked_add(time::Duration::days(days_in_month))
            .expect("time overflow");
    }
}
