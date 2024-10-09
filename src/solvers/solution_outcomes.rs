#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TaskResult<'a> {
    SAT(Vec<bool>),
    UNSAT(&'a str),
    UNKNOWN,
}