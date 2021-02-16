use nom::{types::CompleteStr, *};

use crate::schedule::{CronBaseExpression, CronExpression, Error, Schedule};

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
          minutes: cron_expression_list >>
          hours: cron_expression_list >>
          eof!() >>
          (Schedule::from_cron_expression_list(minutes, hours))))
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_number() {
        let expression = "42";
        number(CompleteStr(expression)).unwrap();
    }

    #[test]
    fn test_invalid_number() {
        let expression = "AAA";
        assert!(number(CompleteStr(expression)).is_err());
    }

    #[test]
    fn test_valid_all() {
        let expression = "*";
        all(CompleteStr(expression)).unwrap();
    }

    #[test]
    fn test_valid_period() {
        let expression = "4/2";
        period(CompleteStr(expression)).unwrap();
    }

    #[test]
    fn test_invalid_period() {
        let expression = "cron/1";
        assert!(period(CompleteStr(expression)).is_err());
    }

    #[test]
    fn test_valid_number_list() {
        let expression = "4,2";
        cron_expression_list(CompleteStr(expression)).unwrap();
    }

    #[test]
    fn test_invalid_number_list() {
        let expression = "A,4,2";
        assert!(cron_expression_list(CompleteStr(expression)).is_err());
    }

    #[test]
    fn test_valid_range() {
        let expression = "2-4";
        range(CompleteStr(expression)).unwrap();
    }

    #[test]
    fn test_valid_all_period() {
        let expression = "*/1";
        period(CompleteStr(expression)).unwrap();
    }

    #[test]
    fn test_invalid_period_range_schedule() {
        let expression = "30/30-150";
        assert!(schedule(CompleteStr(expression)).is_err());
    }
}
