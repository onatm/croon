use linked_hash_set::LinkedHashSet;
use std::{error, fmt::Display, str::FromStr};

use crate::parser::Parser;
#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong!")
    }
}
impl error::Error for Error {}

#[derive(Debug, PartialEq)]
pub enum CronBaseExpression {
    All,
    Exact(u32),
    Range(u32, u32),
}

#[derive(Debug, PartialEq)]
pub enum CronExpression {
    Simple(CronBaseExpression),
    Period(CronBaseExpression, u32),
}

#[derive(Debug)]
pub struct Schedule {
    pub minutes: Vec<u32>,
}

impl FromStr for Schedule {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Parser::parse(expression)
    }
}

impl Schedule {
    pub fn from_cron_expression_list(list: Vec<CronExpression>) -> Schedule {
        let mut set = LinkedHashSet::<u32>::new();
        for expression in list {
            println!("{:?}", expression);
            let inner = Self::from_cron_expression(expression);
            set.extend(inner);
        }
        let mut items: Vec<u32> = set.into_iter().collect();
        items.sort();
        Schedule { minutes: items }
    }

    fn from_cron_expression(expression: CronExpression) -> LinkedHashSet<u32> {
        match expression {
            CronExpression::Simple(expression) => Self::from_cron_base_expression(expression),
            CronExpression::Period(start, step) => {
                let set = match start {
                    CronBaseExpression::Exact(start) => (start..=23).collect(),
                    expression => Self::from_cron_base_expression(expression),
                };
                set.into_iter().step_by(step as usize).collect()
            }
        }
    }

    fn from_cron_base_expression(expression: CronBaseExpression) -> LinkedHashSet<u32> {
        match expression {
            CronBaseExpression::All => (0..=23).collect(),
            CronBaseExpression::Exact(number) => {
                let mut set = LinkedHashSet::new();
                set.insert(number);
                set
            }
            CronBaseExpression::Range(start, end) => (start..=end).collect(),
        }
    }
}
