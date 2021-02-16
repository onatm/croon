use linked_hash_set::LinkedHashSet;
use std::str::FromStr;

use crate::{
    error::Error,
    expression::{CronBaseExpression, CronExpression},
    parser::Parser,
};

#[derive(Debug, PartialEq)]
pub struct Schedule {
    pub minute: Vec<u32>,
    pub hour: Vec<u32>,
    pub day_of_month: Vec<u32>,
    pub month: Vec<u32>,
    pub day: Vec<u32>,
}

impl FromStr for Schedule {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Parser::parse(expression)
    }
}

impl Schedule {
    pub fn from_cron_expression_list(
        minute_list: Vec<CronExpression>,
        hour_list: Vec<CronExpression>,
        day_of_month_list: Vec<CronExpression>,
        month_list: Vec<CronExpression>,
        day_list: Vec<CronExpression>,
    ) -> Schedule {
        Schedule {
            minute: Self::calculate_unit(minute_list, 0, 59),
            hour: Self::calculate_unit(hour_list, 0, 23),
            day_of_month: Self::calculate_unit(day_of_month_list, 1, 31),
            month: Self::calculate_unit(month_list, 1, 12),
            day: Self::calculate_unit(day_list, 0, 6),
        }
    }

    fn calculate_unit(expressions: Vec<CronExpression>, min: u32, max: u32) -> Vec<u32> {
        let mut set = LinkedHashSet::<u32>::new();
        for expression in expressions {
            let inner = Self::from_cron_expression(expression, min, max);
            set.extend(inner);
        }
        let mut items: Vec<u32> = set.into_iter().collect();
        items.sort();
        items
    }

    fn from_cron_expression(expression: CronExpression, min: u32, max: u32) -> LinkedHashSet<u32> {
        match expression {
            CronExpression::Simple(expression) => {
                Self::from_cron_base_expression(expression, min, max)
            }
            CronExpression::Period(start, step) => {
                let set = match start {
                    CronBaseExpression::Exact(start) => (start..=max).collect(),
                    expression => Self::from_cron_base_expression(expression, min, max),
                };
                set.into_iter().step_by(step as usize).collect()
            }
        }
    }

    fn from_cron_base_expression(
        expression: CronBaseExpression,
        min: u32,
        max: u32,
    ) -> LinkedHashSet<u32> {
        match expression {
            CronBaseExpression::All => (min..=max).collect(),
            CronBaseExpression::Exact(number) => {
                let mut set = LinkedHashSet::new();
                set.insert(number);
                set
            }
            CronBaseExpression::Range(start, end) => (start..=end).collect(),
        }
    }
}
