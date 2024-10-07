use std::collections::HashSet;

#[derive(Debug)]
pub struct Clause {
    pub literals: HashSet<isize>,
}

impl Clause {
    pub fn empty() -> Clause {
        Clause { literals: HashSet::new() }
    }

    pub fn from_set(literals: HashSet<isize>) -> Clause {
        Clause { literals }
    }

    pub fn from_iter<I>(iter: I) -> Clause
    where
        I: IntoIterator<Item = isize>,
    {
        Clause { literals: iter.into_iter().collect() }
    }

    pub fn is_satisfied(&self, assignment: &Vec<Option<bool>>) -> bool {
        for literal in self.literals.iter() {
            let literal_idx = (literal.abs() - 1) as usize;
            if assignment[literal_idx].is_none() { return false }
            else {
                if *literal > 0 && assignment[literal_idx].unwrap() == false { return false }
                if *literal < 0 && assignment[literal_idx].unwrap() == true { return false }
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct Task {
    pub n_variables: usize,
    pub clauses: Vec<Clause>,
}

impl Task {
    pub fn empty(n_variables: usize, n_clauses: usize) -> Task {
        Task { n_variables, clauses: Vec::with_capacity(n_clauses) }
    }
}
