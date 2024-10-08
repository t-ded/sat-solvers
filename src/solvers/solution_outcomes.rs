pub enum TaskResult {
    SAT(Vec<bool>),
    UNSAT(String),
    UNKNOWN,
}