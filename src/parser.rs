use nom::{types::CompleteStr, *};

use crate::{
    error::Error,
    expression::{CronBaseExpression, CronExpression},
    schedule::Schedule,
};

pub struct Parser;

impl Parser {
    pub fn parse(expression: &str) -> Result<Schedule, Error> {
        match schedule(CompleteStr(expression)) {
            Ok((_, schedule)) => Ok(schedule),
            Err(_) => Err(Error {}),
        }
    }
}

named!(
  number<CompleteStr, u32>,
  map_res!(ws!(digit), |x: CompleteStr| x.0.parse())
);

named!(
  exact<CompleteStr, CronBaseExpression>,
  do_parse!(n: number >> (CronBaseExpression::Exact(n)))
);

named!(
  range<CompleteStr, CronBaseExpression>,
  complete!(do_parse!(
      start: number >> tag!("-") >> end: number >> (CronBaseExpression::Range(start, end))
  ))
);

named!(all<CompleteStr, CronBaseExpression>, do_parse!(tag!("*") >> (CronBaseExpression::All)));

named!(
  cron_base_expression<CompleteStr, CronBaseExpression>,
  alt!(all | range | exact)
);

named!(
  period<CompleteStr, CronExpression>,
  complete!(do_parse!(
      start: cron_base_expression >> tag!("/") >> step: number >> (CronExpression::Period(start, step))
  ))
);

named!(
  cron_expression<CompleteStr, CronExpression>,
  alt!(period | map!(cron_base_expression, |x| CronExpression::Simple(x)))
);

named!(
  cron_expression_list<CompleteStr, Vec<CronExpression>>,
  ws!(complete!(alt!(
      separated_nonempty_list!(tag!(","), cron_expression)
          | do_parse!(spec: cron_expression >> (vec![spec])))
  ))
);

named!(
  schedule<CompleteStr, Schedule>,
  complete!(
      do_parse!(
          minute: cron_expression_list >>
          hour: cron_expression_list >>
          day_of_month: cron_expression_list >>
          month: cron_expression_list >>
          day: cron_expression_list >>
          eof!() >>
          (Schedule::from_cron_expression_list(minute, hour, day_of_month, month, day))))
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_number() {
        number(CompleteStr("42")).unwrap();
    }

    #[test]
    fn test_invalid_number() {
        assert!(number(CompleteStr("AAA")).is_err());
    }

    #[test]
    fn test_valid_range() {
        range(CompleteStr("2-4")).unwrap();
    }

    #[test]
    fn test_invalid_range() {
        assert!(range(CompleteStr("2-A")).is_err());
    }

    #[test]
    fn test_valid_all() {
        all(CompleteStr("*")).unwrap();
    }

    #[test]
    fn test_valid_period() {
        period(CompleteStr("4/2")).unwrap();
    }

    #[test]
    fn test_invalid_period() {
        assert!(period(CompleteStr("cron/1")).is_err());
    }

    #[test]
    fn test_valid_all_period() {
        period(CompleteStr("*/1")).unwrap();
    }

    #[test]
    fn test_invalid_all_period() {
        assert!(period(CompleteStr("1/*")).is_err());
    }

    #[test]
    fn test_valid_number_list() {
        cron_expression_list(CompleteStr("4,2")).unwrap();
    }

    #[test]
    fn test_invalid_number_list() {
        assert!(cron_expression_list(CompleteStr("A,4,2")).is_err());
    }

    #[test]
    fn test_valid_range_list() {
        cron_expression_list(CompleteStr("2-4,4-6")).unwrap();
    }

    #[test]
    fn test_valid_period_list() {
        cron_expression_list(CompleteStr("4,2/1")).unwrap();
    }

    #[test]
    fn test_all_schedule() {
        let expected = Schedule {
            minute: (0..=59).collect(),
            hour: (0..=23).collect(),
            day_of_month: (1..=31).collect(),
            month: (1..=12).collect(),
            day: (0..=6).collect(),
        };

        let (_, schedule) = schedule(CompleteStr("* * * * *")).unwrap();
        assert_eq!(schedule, expected);
    }

    #[test]
    fn test_monday_only_schedule() {
        let expected = Schedule {
            minute: (0..=59).collect(),
            hour: (0..=23).collect(),
            day_of_month: (1..=31).collect(),
            month: (1..=12).collect(),
            day: vec![1],
        };

        let (_, schedule) = schedule(CompleteStr("* * * * 1")).unwrap();
        assert_eq!(schedule, expected);
    }

    #[test]
    fn test_every_two_hours_between_two_and_eight_schedule() {
        let expected = Schedule {
            minute: (0..=59).collect(),
            hour: (2..=8).step_by(2).collect(),
            day_of_month: (1..=31).collect(),
            month: (1..=12).collect(),
            day: (0..=6).collect(),
        };

        let (_, schedule) = schedule(CompleteStr("* 2-8/2 * * *")).unwrap();
        assert_eq!(schedule, expected);
    }

    #[test]
    fn test_first_day_of_month_schedule() {
        let expected = Schedule {
            minute: (0..=59).collect(),
            hour: (0..=23).collect(),
            day_of_month: vec![1],
            month: (1..=12).collect(),
            day: (0..=6).collect(),
        };

        let (_, schedule) = schedule(CompleteStr("* * 1 * *")).unwrap();
        assert_eq!(schedule, expected);
    }

    #[test]
    fn test_invalid_period_range_schedule() {
        assert!(schedule(CompleteStr("30/30-150 * * * *")).is_err());
    }
}
