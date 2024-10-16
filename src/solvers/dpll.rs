use crate::sat_task::task::{Clause, Task};
use crate::solvers::solution_outcomes::TaskResult;
use crate::solvers::solution_outcomes::TaskResult::UNSAT;
use std::collections::HashMap;
use std::ops::Neg;

pub struct DPLLSolver {
    cache_removed_clauses: Vec<Clause>,
    cache_clauses_with_removed_literal: Vec<(usize, isize)>,
    cache_pure_literals: Vec<usize>,
}

impl DPLLSolver {
    pub fn new() -> Self {
        DPLLSolver { cache_removed_clauses: Vec::new(), cache_clauses_with_removed_literal: Vec::new(), cache_pure_literals: Vec::new() }
    }

    pub fn solve(&mut self, mut input: Task, chosen_lit: usize) -> TaskResult {
        if input.is_solved() {
            self.return_sat_assignment(&input)
        } else if input.is_solvable() && chosen_lit <= input.n_variables {
            self.unit_propagation(&mut input);
            if input.contains_empty_clause() {
                return UNSAT("TODO")
            }
            self.pure_literal_removal(&mut input);
            if input.n_clauses == 0 {
                self.fill_remaining_true(&mut input);
                return self.return_sat_assignment(&input)
            }

            if input.assignment.get(&chosen_lit).unwrap().is_none() {
                let mut input_copy = input.clone();
                input_copy.assign_literal(chosen_lit, Some(true));
                input_copy.add_clause(Clause::from_literal_iter(std::iter::once(chosen_lit as isize)));
                if let TaskResult::SAT(assignment) = self.solve(input_copy, chosen_lit + 1) {
                    return TaskResult::SAT(assignment)
                }

                let mut input_copy = input.clone();
                input_copy.assign_literal(chosen_lit, Some(false));
                input_copy.add_clause(Clause::from_literal_iter(std::iter::once(-(chosen_lit as isize))));
                if let TaskResult::SAT(assignment) = self.solve(input_copy, chosen_lit + 1) {
                    return TaskResult::SAT(assignment)
                }
            }

            UNSAT("TODO")
        } else {
            println!("Not solvable: {:?}", input.assignment);
            UNSAT("TODO")
        }
    }

    pub fn unit_propagation(&mut self, input: &mut Task) {
        loop {
            let unit_literal = self.find_unit_literal(input);
            if unit_literal == 0 { break }
            if unit_literal > 0 { input.assignment.insert(unit_literal as usize, Some(true)); }
            else { input.assignment.insert(unit_literal.unsigned_abs(), Some(false)); }
            let mut i = 0;
            while i < input.n_clauses {
                if input.clauses[i].literals.contains(&unit_literal) {
                    input.remove_nth_clause(i);
                } else {
                    if input.clauses[i].literals.contains(&unit_literal.neg()) {
                        input.clauses[i].remove_literal(unit_literal.neg());
                    }
                    i += 1;
                }
            }
        }
    }

    fn find_unit_literal(&self, input: &Task) -> isize {
        for clause in input.clauses.iter() {
            if clause.n_literals == 1 {
                return *clause.literals.iter().next().unwrap()
            }
        }
        0
    }

    pub fn pure_literal_removal(&mut self, input: &mut Task) {
        loop {
            let pure_literal = self.find_pure_literal(input);
            if pure_literal == 0 { break }
            if pure_literal > 0 { input.assignment.insert(pure_literal as usize, Some(true)); }
            else { input.assignment.insert(pure_literal.unsigned_abs(), Some(false)); }
            let mut i = 0;
            while i < input.n_clauses {
                if input.clauses[i].literals.contains(&pure_literal) {
                    input.remove_nth_clause(i);
                } else { i += 1 }
            }
        }
    }

    fn find_pure_literal(&self, input: &Task) -> isize {
        let mut literal_sign_found: HashMap<isize, Option<isize>> = HashMap::from_iter(
            input.assignment
                .iter()
                .filter_map(
                    |(&key, &value)| {
                        if value.is_none() {Some((key as isize, None))} else {None}
                    }
                )
        );
        for (literal, sign_found) in literal_sign_found.iter_mut() {
            for clause in input.clauses.iter() {
                if clause.literals.contains(literal) {
                    if sign_found.is_none() { *sign_found = Some(1); }
                    else if *sign_found == Some(-1) { *sign_found = Some(0); }
                } else if clause.literals.contains(&(-literal)) {
                    if sign_found.is_none() { *sign_found = Some(-1); }
                    else if *sign_found == Some(1) { *sign_found = Some(0); }
                }
            }
            if sign_found.is_none() || *sign_found == Some(1) { return *literal}
            else if *sign_found == Some(-1) { return -*literal}
        }
        0
    }

    // fn revert_pure_literal_removal(&mut self, input: &mut Task) {
    //     // while let Some(clause) = self.cache_removed_clauses.pop() {
    //     //     input.add_clause(clause);
    //     // }
    //     while let Some(literal) = self.cache_pure_literals.pop() {
    //         input.assignment.insert(literal, None);
    //     }
    // }

    fn fill_remaining_true(&self, input: &mut Task) {
        for assigned_value in input.assignment.values_mut() {
            if assigned_value.is_none() { *assigned_value = Some(true); }
        }
    }

    fn return_sat_assignment(&self, input: &Task) -> TaskResult {
        TaskResult::SAT(
            input.assignment
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.expect("SAT assignment should have all literals assigned")))
                .collect()
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Parser;

    #[test]
    fn test_basic_task() {
        let mut solver = DPLLSolver::new();
        let parser = Parser {};
        let mut task = parser.parse_str(
            "p cnf 5 3
            1 -5 4 0
            -1 5 3 4 0
            -3 -4 0"
        );
        let result = solver.solve(task.clone(), 1);
        match result {
            TaskResult::SAT(sat) => {
                task.assignment = sat.into_iter().map(|(k, v)| (k, Some(v))).collect();
                assert_eq!(task.is_solved(), true);
            },
            _ => assert!(false),
        }
    }
}