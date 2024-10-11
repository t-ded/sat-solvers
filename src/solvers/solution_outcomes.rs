use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TaskResult<'a> {
    SAT(HashMap<usize, bool>),
    UNSAT(&'a str),
    UNKNOWN,
}