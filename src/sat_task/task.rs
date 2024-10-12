use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Clause {
    pub literals: HashSet<isize>,
    pub n_literals: usize,
}

impl Clause {
    pub fn empty() -> Clause {
        Clause { literals: HashSet::new(), n_literals: 0 }
    }

    pub fn from_set(literal_set: HashSet<isize>) -> Clause {
        Clause {
            n_literals: literal_set.len(),
            literals: literal_set,
        }
    }

    pub fn from_literal_iter<I>(iter: I) -> Clause
    where
        I: IntoIterator<Item = isize>,
    {
        let mut clause = Clause {
            literals: iter.into_iter().collect(),
            n_literals: 0,
        };
        clause.n_literals = clause.literals.len();
        clause
    }

    pub fn can_be_satisfied(&self, assignment: &HashMap<usize, Option<bool>>) -> bool {
        self.get_satisfiability(assignment, false)
    }

    pub fn is_satisfied(&self, assignment: &HashMap<usize, Option<bool>>) -> bool {
        self.get_satisfiability(assignment, true)
    }

    fn get_satisfiability(&self, assignment: &HashMap<usize, Option<bool>>, is_final: bool) -> bool {
        if self.n_literals == 0 { return false }
        let mut any_satisfied_found = false;
        for literal in self.literals.iter() {
            if is_final && assignment[&literal.unsigned_abs()].is_none() { return false }
            else if !is_final && assignment[&literal.unsigned_abs()].is_none() { return true }
            else if !any_satisfied_found {
                if *literal > 0 && assignment[&literal.unsigned_abs()].unwrap() == true { any_satisfied_found = true }
                if *literal < 0 && assignment[&literal.unsigned_abs()].unwrap() == false { any_satisfied_found = true }
            }
        }
        any_satisfied_found
    }

    pub fn remove_literal(&mut self, literal: isize) {
        self.n_literals -= 1;
        self.literals.remove(&literal);
    }

    pub fn add_literal(&mut self, literal: isize) {
        self.n_literals += 1;
        self.literals.insert(literal);
    }
}

#[derive(Debug)]
pub struct Task {
    pub n_variables: usize,
    pub n_clauses: usize,
    pub clauses: Vec<Clause>,
    pub assignment: HashMap<usize, Option<bool>>,
}

impl Task {
    pub fn empty(n_variables: usize, n_clauses: usize) -> Task {
        Task {
            n_variables, n_clauses, clauses: Vec::with_capacity(n_clauses + n_variables),
            assignment: (1..=n_variables).map(|i| (i, None)).collect::<HashMap<usize, Option<bool>>>()
        }
    }

    pub fn is_nth_satisfied(&self, n: usize) -> bool {
        self.clauses[n].is_satisfied(&self.assignment)
    }

    pub fn is_solved(&self) -> bool {
        for clause in self.clauses.iter() {
            if !clause.is_satisfied(&self.assignment) { return false }
        }
        for truth_value in self.assignment.values() {
            if truth_value.is_none() { return false }
        }
        true
    }

    pub fn is_solvable(&self) -> bool {
        for clause in self.clauses.iter() {
            if !clause.can_be_satisfied(&self.assignment) { return false }
        }
        true
    }

    pub fn assign_literal(&mut self, literal: usize, value: Option<bool>) {
        self.assignment.insert(literal, value);
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.n_clauses += 1;
        self.clauses.insert(0, clause);
    }

    pub fn remove_nth_clause(&mut self, n: usize) -> Clause {
        self.n_clauses -= 1;
        self.clauses.remove(n)
    }
}
