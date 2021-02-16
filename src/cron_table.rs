use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use linked_hash_set::LinkedHashSet;
use std::str::FromStr;

use crate::{
    error::Error,
    expression::{CronBaseExpression, CronExpression},
    parser::Parser,
};

#[derive(Debug, PartialEq)]
pub struct CronTab {
    pub minute: Vec<u32>,
    pub hour: Vec<u32>,
    pub day_of_month: Vec<u32>,
    pub month: Vec<u32>,
    pub day_of_week: Vec<u32>,
    pub command: String,
}

impl FromStr for CronTab {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Parser::parse(expression)
    }
}

impl CronTab {
    pub fn next(&self, after: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        let base_year = after.year();
        let base_month = after.month();
        let base_day = after.day();
        let base_hour = after.hour();
        let base_minute = after.minute();

        let mut year = base_year;
        let mut month = base_month;
        let mut day = base_day;
        let mut hour = base_hour;
        let mut minute = base_minute;

        let minute_range: LinkedHashSet<u32> = (minute..=59).collect();
        let minutes: LinkedHashSet<u32> = self.minute.clone().into_iter().collect();
        let minutes = minute_range.intersection(&minutes).collect::<Vec<&u32>>();

        if let Some(&&first_minute) = minutes.first() {
            minute = first_minute;
        } else {
            minute = self.minute[0].clone();
            hour += 1;
        }

        let hour_range: LinkedHashSet<u32> = (hour..=23).collect();
        let hours: LinkedHashSet<u32> = self.hour.clone().into_iter().collect();
        let hours = hour_range.intersection(&hours).collect::<Vec<&u32>>();

        if let Some(&&first_hour) = hours.first() {
            if first_hour > base_hour {
                minute = self.minute[0].clone();
            } else {
                hour = first_hour;
            }
        } else {
            minute = self.minute[0].clone();
            hour = self.hour[0].clone();
            day += 1;
        }

        let day_range: LinkedHashSet<u32> = (day..=31).collect();
        let days: LinkedHashSet<u32> = self.day_of_month.clone().into_iter().collect();
        let days = day_range.intersection(&days).collect::<Vec<&u32>>();

        if let Some(&&first_day) = days.first() {
            println!("{:?}", first_day);
            if first_day > base_day {
                minute = self.minute[0].clone();
                hour = self.hour[0].clone();
            } else {
                day = first_day;
            }
        } else {
            minute = self.minute[0].clone();
            hour = self.hour[0].clone();
            day = self.day[0].clone();
            month += 1;
        }

        let month_range: LinkedHashSet<u32> = (month..=12).collect();
        let months: LinkedHashSet<u32> = self.month.clone().into_iter().collect();
        let months = month_range.intersection(&months).collect::<Vec<&u32>>();

        if let Some(&&first_month) = months.first() {
            if first_month > base_month {
                minute = self.minute[0].clone();
                hour = self.hour[0].clone();
                day = self.day[0].clone();
            } else {
                month = first_month;
            }
        } else {
            minute = self.minute[0].clone();
            hour = self.hour[0].clone();
            day = self.day[0].clone();
            month = self.month[0].clone();
            year += 1;
        }

        let next_time = Utc.ymd(year, month, day).and_hms(hour, minute, 0);

        let day_of_week = next_time.weekday().number_from_sunday() - 1;

        if self.day.contains(&day_of_week) {
            Some(next_time)
        } else {
            None
        }
    }

    pub fn from_cron_expression_list(
        minute_list: Vec<CronExpression>,
        hour_list: Vec<CronExpression>,
        day_of_month_list: Vec<CronExpression>,
        month_list: Vec<CronExpression>,
        day_list: Vec<CronExpression>,
        command: String,
    ) -> Result<CronTab, Error> {
        let minute = Self::calculate_time(minute_list, 0, 59)?;
        let hour = Self::calculate_time(hour_list, 0, 23)?;
        let day_of_month = Self::calculate_time(day_of_month_list, 1, 31)?;
        let month = Self::calculate_time(month_list, 1, 12)?;
        let day_of_week = Self::calculate_time(day_list, 0, 6)?;

        if command.len() == 0 {
            return Err(Error);
        }

        Ok(CronTab {
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
            command,
        })
    }

    fn calculate_time(
        expressions: Vec<CronExpression>,
        min: u32,
        max: u32,
    ) -> Result<Vec<u32>, Error> {
        let mut set = LinkedHashSet::<u32>::new();
        for expression in expressions {
            set.extend(Self::from_cron_expression(expression, min, max)?);
        }
        let mut items: Vec<u32> = set.into_iter().collect();
        items.sort();
        Ok(items)
    }

    fn from_cron_expression(
        expression: CronExpression,
        min: u32,
        max: u32,
    ) -> Result<LinkedHashSet<u32>, Error> {
        match expression {
            CronExpression::Simple(expression) => {
                Self::from_cron_base_expression(expression, min, max)
            }
            CronExpression::Frequency(start, freq) => {
                let set = match start {
                    CronBaseExpression::Exact(start) => Ok((start..=max).collect()),
                    expression => Self::from_cron_base_expression(expression, min, max),
                }?;
                Ok(set
                    .into_iter()
                    .step_by(freq as usize)
                    .collect::<LinkedHashSet<u32>>())
            }
        }
    }

    fn from_cron_base_expression(
        expression: CronBaseExpression,
        min: u32,
        max: u32,
    ) -> Result<LinkedHashSet<u32>, Error> {
        match expression {
            CronBaseExpression::All => Ok((min..=max).collect()),
            CronBaseExpression::Exact(number) => {
                Ok(vec![number].into_iter().collect::<LinkedHashSet<u32>>())
            }
            CronBaseExpression::Range(start, end) => {
                if start > max || end > max {
                    Err(Error)
                } else {
                    Ok((start..=end).collect())
                }
            }
        }
    }
}
