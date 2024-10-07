use std::collections::HashSet;

pub struct Task {
    clauses: Vec<HashSet<isize>>,
}

impl Task {
    pub fn empty() -> Task {
        Task { clauses: Vec::new() }
    }
}
