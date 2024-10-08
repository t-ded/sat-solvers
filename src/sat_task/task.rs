use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Clause {
    pub literals: HashMap<isize, bool>,
    pub n_literals: usize,
}

impl Clause {
    pub fn empty() -> Clause {
        Clause { literals: HashMap::new(), n_literals: 0 }
    }

    pub fn from_set(literal_set: HashSet<isize>) -> Clause {
        Clause {
            n_literals: literal_set.len(),
            literals: literal_set.into_iter().map(|lit| (lit, true)).collect(),
        }
    }

    pub fn from_literal_iter<I>(iter: I) -> Clause
    where
        I: IntoIterator<Item = isize>,
    {
        let mut clause = Clause {
            literals: iter.into_iter().map(|lit| (lit, true)).collect(),
            n_literals: 0,
        };
        clause.n_literals = clause.literals.len();
        clause
    }

    pub fn is_satisfied(&self, assignment: &Vec<Option<bool>>) -> bool {
        if self.n_literals == 0 { return false }
        for (literal, literal_mask) in self.literals.iter() {
            if *literal_mask {
                let literal_idx = (literal.abs() - 1) as usize;
                if assignment[literal_idx].is_none() { return false }
                else {
                    if *literal > 0 && assignment[literal_idx].unwrap() == false { return false }
                    if *literal < 0 && assignment[literal_idx].unwrap() == true { return false }
                }
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct Task {
    pub n_variables: usize,
    pub n_clauses: usize,
    pub clauses: Vec<Clause>,
    mask: Vec<bool>,
}

impl Task {
    pub fn empty(n_variables: usize, n_clauses: usize) -> Task {
        Task { n_variables, n_clauses, clauses: Vec::with_capacity(n_clauses), mask: vec![true; n_clauses] }
    }

    pub fn remove_nth_clause(&mut self, n: usize) {
        self.mask[n] = false;
        self.n_clauses -= 1;
    }
}
