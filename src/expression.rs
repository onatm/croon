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
