use std::collections::HashMap;

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
            Err(_) => Err(Error),
        }
    }
}

lazy_static! {
    static ref ALIAS_MAP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("MON", 1);
        m.insert("TUE", 2);
        m.insert("WED", 3);
        m.insert("THU", 4);
        m.insert("FRI", 5);
        m.insert("SAT", 6);
        m.insert("SUN", 0);
        m
    };
}

named!(
  number<CompleteStr, u32>,
  map_res!(ws!(digit), |x: CompleteStr| x.0.parse())
);

named!(
  exact<CompleteStr, CronBaseExpression>,
  do_parse!(n: alt!(number | day) >> (CronBaseExpression::Exact(n)))
);

fn map_alias_to_number(alias: String) -> Result<u32, Error> {
    if let Some(value) = ALIAS_MAP.get(&*alias) {
        Ok(value.clone())
    } else {
        Err(Error)
    }
}

named!(
    day<CompleteStr, u32>,
    do_parse!(
        day: alt!(tag!("MON") | tag!("TUE") | tag!("WED") | tag!("THU") | tag!("FRI") | tag!("SAT") | tag!("SUN")) >> (map_alias_to_number(day.0.to_string()).unwrap())
    )
);

named!(
  range<CompleteStr, CronBaseExpression>,
  complete!(do_parse!(
      start: alt!(number | day) >> tag!("-") >> end: alt!(number| day) >> (CronBaseExpression::Range(start, end))
  ))
);

named!(all<CompleteStr, CronBaseExpression>, do_parse!(tag!("*") >> (CronBaseExpression::All)));

named!(
  cron_base_expression<CompleteStr, CronBaseExpression>,
  alt!(all | range | exact )
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
    command<CompleteStr, String>,
    do_parse!(
        multispace0 >>
        command: take_while!(|c: char| c.is_alphanumeric() || c == '\'' || c == '/' || c.is_whitespace()) >> (String::from(command.0))
    )
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
            command: command >>
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
    fn test_valid_day() {
        day(CompleteStr("MON")).unwrap();
    }

    #[test]
    fn test_invalid_alias() {
        assert!(day(CompleteStr("MoN")).is_err());
    }

    #[test]
    fn test_valid_range() {
        range(CompleteStr("2-4")).unwrap();
    }

    #[test]
    fn test_valid_range_of_number_and_day() {
        range(CompleteStr("MON-4")).unwrap();
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
    fn test_valid_alias_list() {
        cron_expression_list(CompleteStr("MON,2")).unwrap();
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
