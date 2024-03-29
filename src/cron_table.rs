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
