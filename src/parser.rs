use nom::{types::CompleteStr, *};

use crate::{
    cron_table::CronTab,
    error::Error,
    expression::{CronBaseExpression, CronExpression},
};

pub struct Parser;

impl Parser {
    pub fn parse(expression: &str) -> Result<CronTab, Error> {
        match cron_table(CompleteStr(expression)) {
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
  freq<CompleteStr, CronExpression>,
  complete!(do_parse!(
      start: cron_base_expression >> tag!("/") >> freq: number >> (CronExpression::Frequency(start, freq))
  ))
);

named!(
  cron_expression<CompleteStr, CronExpression>,
  alt!(freq | map!(cron_base_expression, |x| CronExpression::Simple(x)))
);

named!(
  cron_expression_list<CompleteStr, Vec<CronExpression>>,
  ws!(complete!(alt!(
      separated_nonempty_list!(tag!(","), cron_expression)
          | do_parse!(spec: cron_expression >> (vec![spec])))
  ))
);

named!(
    rest<CompleteStr, String>,
    parse_to!(String)
);

named!(
  cron_table<CompleteStr, CronTab>,
  map_res!(
    complete!(
        do_parse!(
            minute: cron_expression_list >>
            hour: cron_expression_list >>
            day_of_month: cron_expression_list >>
            month: cron_expression_list >>
            day_of_week: cron_expression_list >>
            command: rest >>
            eof!() >> (minute, hour, day_of_month, month, day_of_week, command)
        )
    ),
    |(minute, hour, day_of_month, month, day_of_week, command)| CronTab::from_cron_expression_list(minute, hour, day_of_month, month, day_of_week, command)
  )
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
    fn test_valid_freq() {
        freq(CompleteStr("4/2")).unwrap();
    }

    #[test]
    fn test_invalid_freq() {
        assert!(freq(CompleteStr("cron/1")).is_err());
    }

    #[test]
    fn test_valid_all_freq() {
        freq(CompleteStr("*/1")).unwrap();
    }

    #[test]
    fn test_invalid_all_freq() {
        assert!(freq(CompleteStr("1/*")).is_err());
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
    fn test_valid_freq_list() {
        cron_expression_list(CompleteStr("4,2/1")).unwrap();
    }

    #[test]
    fn test_all_cron_table() {
        let expected = CronTab {
            minute: (0..=59).collect(),
            hour: (0..=23).collect(),
            day_of_month: (1..=31).collect(),
            month: (1..=12).collect(),
            day_of_week: (0..=6).collect(),
            command: String::from("echo 'hello world'"),
        };

        let (_, cron_table) = cron_table(CompleteStr("* * * * * echo 'hello world'")).unwrap();
        assert_eq!(cron_table, expected);
    }

    #[test]
    fn test_monday_only_cron_table() {
        let expected = CronTab {
            minute: (0..=59).collect(),
            hour: (0..=23).collect(),
            day_of_month: (1..=31).collect(),
            month: (1..=12).collect(),
            day_of_week: vec![1],
            command: String::from("echo"),
        };

        let (_, cron_table) = cron_table(CompleteStr("* * * * 1 echo")).unwrap();
        assert_eq!(cron_table, expected);
    }

    #[test]
    fn test_every_two_hours_between_two_and_eight_cron_table() {
        let expected = CronTab {
            minute: (0..=59).collect(),
            hour: (2..=8).step_by(2).collect(),
            day_of_month: (1..=31).collect(),
            month: (1..=12).collect(),
            day_of_week: (0..=6).collect(),
            command: String::from("echo"),
        };

        let (_, cron_table) = cron_table(CompleteStr("* 2-8/2 * * * echo")).unwrap();
        assert_eq!(cron_table, expected);
    }

    #[test]
    fn test_first_day_of_month_cron_table() {
        let expected = CronTab {
            minute: (0..=59).collect(),
            hour: (0..=23).collect(),
            day_of_month: vec![1],
            month: (1..=12).collect(),
            day_of_week: (0..=6).collect(),
            command: String::from("echo"),
        };

        let (_, cron_table) = cron_table(CompleteStr("* * 1 * * echo")).unwrap();
        assert_eq!(cron_table, expected);
    }

    #[test]
    fn test_invalid_freq_range_cron_table() {
        assert!(cron_table(CompleteStr("30/30-150 * * * * echo")).is_err());
    }

    #[test]
    fn test_missing_command_cron_table() {
        assert!(cron_table(CompleteStr("* * * * *")).is_err());
    }
}
