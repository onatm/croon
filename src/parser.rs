use character::complete::{digit0, multispace0};
use nom::*;

use crate::schedule::{CronBaseExpression, CronExpression, Error, Schedule};

pub struct Parser;

impl Parser {
    pub fn parse(expression: &str) -> Result<Schedule, Error> {
        match schedule(expression) {
            Ok((_, schedule)) => Ok(schedule),
            Err(_) => Err(Error {}),
        }
    }
}

named!(number<&str, u32>, map_res!(delimited!(multispace0, digit0, multispace0), |x: &str| x.parse()));

named!(
    exact<&str, CronBaseExpression>,
    do_parse!(n: number >> (CronBaseExpression::Exact(n)))
);

named!(
    range<&str, CronBaseExpression>,
    complete!(do_parse!(
        start: number >> tag!("-") >> end: number >> (CronBaseExpression::Range(start, end))
    ))
);

named!(
    all<&str, CronBaseExpression>,
    do_parse!(tag!("*") >> (CronBaseExpression::All))
);

named!(
    cron_base_expression<&str, CronBaseExpression>,
    alt!(all | range | exact)
);

named!(
    period<&str, CronExpression>,
    complete!(do_parse!(
        start: cron_base_expression
            >> tag!("/")
            >> step: number
            >> (CronExpression::Period(start, step))
    ))
);

named!(
    cron_expression<&str, CronExpression>,
    alt!(period | map!(cron_base_expression, |x| CronExpression::Simple(x)))
);

named!(
    cron_expression_list<&str, Vec<CronExpression>>,
    separated_list1!(tag!(","), cron_expression)
);

named!(
    schedule<&str, Schedule>,
    do_parse!(minutes: cron_expression_list >> (Schedule::from_cron_expression_list(minutes)))
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_number() {
        let expression = "42";
        number(expression).unwrap();
    }

    #[test]
    fn test_invalid_number() {
        let expression = "AAA";
        assert!(number(expression).is_err());
    }

    #[test]
    fn test_valid_all() {
        let expression = "*";
        all(expression).unwrap();
    }

    #[test]
    fn test_valid_period() {
        let expression = "4/2";
        period(expression).unwrap();
    }

    #[test]
    fn test_invalid_period() {
        let expression = "cron/1";
        assert!(period(expression).is_err());
    }

    #[test]
    fn test_valid_number_list() {
        let expression = "4,2";
        cron_expression_list(expression).unwrap();
    }

    #[test]
    fn test_invalid_number_list() {
        let expression = "A,4,2";
        assert!(cron_expression_list(expression).is_err());
    }

    #[test]
    fn test_valid_range() {
        let expression = "2-4";
        range(expression).unwrap();
    }

    #[test]
    fn test_valid_all_period() {
        let expression = "*/1";
        period(expression).unwrap();
    }

    #[test]
    fn test_invalid_period_range() {
        let expression = "30/30-150";
        assert!(period(expression).is_err());
    }
}
