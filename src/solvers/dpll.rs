use crate::sat_task::task::{Clause, Task};
use crate::solvers::solution_outcomes::TaskResult;
use crate::solvers::solution_outcomes::TaskResult::UNSAT;
use std::ops::Neg;

pub struct DPLLSolver {
    cache_removed_clauses: Vec<Clause>,
    cache_clauses_with_removed_literal: Vec<usize>,
}

impl DPLLSolver {
    pub fn new() -> Self {
        DPLLSolver { cache_removed_clauses: Vec::new(), cache_clauses_with_removed_literal: Vec::new() }
    }

    pub fn solve(&mut self, input: &mut Task, chosen_lit: usize) -> TaskResult {
        if input.is_solved() {
            self.return_sat_assignment(&input)
        } else if input.is_solvable() && chosen_lit <= input.n_variables {
            input.assign_literal(chosen_lit, Some(true));
            input.add_clause(Clause::from_literal_iter(std::iter::once(chosen_lit as isize)));
            self.unit_propagation(input);
            if let TaskResult::SAT(assignment) = self.solve(input, chosen_lit + 1) {
                return TaskResult::SAT(assignment)
            }
            self.revert_unit_propagation(input, chosen_lit as isize);

            let neg_lit = -(chosen_lit as isize);
            input.assign_literal(chosen_lit, Some(false));
            input.add_clause(Clause::from_literal_iter(std::iter::once(neg_lit)));
            self.unit_propagation(input);
            if let TaskResult::SAT(assignment) = self.solve(input, chosen_lit + 1) {
                return TaskResult::SAT(assignment)
            }
            self.revert_unit_propagation(input, neg_lit);

            input.assign_literal(chosen_lit, None);
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
            let mut i = 0;
            while i < input.n_clauses {
                if input.clauses[i].literals.contains(&unit_literal) {
                    self.cache_removed_clauses.push(input.remove_nth_clause(i));
                } else {
                    if input.clauses[i].literals.contains(&unit_literal.neg()) {
                        input.clauses[i].remove_literal(unit_literal.neg());
                        self.cache_clauses_with_removed_literal.push(i);
                    }
                    i += 1;
                }
            }
        }
    }

    fn find_unit_literal(&self, input: &'_ Task) -> isize {
        for clause in input.clauses.iter() {
            if clause.n_literals == 1 {
                return *clause.literals.iter().next().unwrap()
            }
        }
        0
    }

    fn revert_unit_propagation(&mut self, input: &mut Task, chosen_lit: isize) {
        while let Some(clause_idx) = self.cache_clauses_with_removed_literal.pop() {
            input.clauses[clause_idx].add_literal(chosen_lit);
        }
        while let Some(clause) = self.cache_removed_clauses.pop() {
            input.add_clause(clause);
        }
    }

        // self.unit_propagation(input);
        // for clause in &input.clauses {
        //     if clause.n_literals == 0 { return TaskResult::UNSAT("TODO") }
        // }
        // self.propagate_pure_literals(input);
        // if input.n_clauses == 0 {
        //     return TaskResult::SAT(
        //         input.assignment
        //             .clone()
        //             .into_iter()
        //             .map(|(k, v)| (k, v.expect("SAT assignment should have all literals assigned")))
        //             .collect()
        //     )
        // }

        // if chosen_lit.unsigned_abs() <= input.n_variables {
        //     let next_chosen_lit = if chosen_lit > 0 {
        //         input.assign_literal(chosen_lit.unsigned_abs(), Some(true));
        //         -chosen_lit
        //     } else {
        //         input.assign_literal(chosen_lit.unsigned_abs(), Some(false));
        //         -chosen_lit + 1
        //     };
        //     // input.add_clause(Clause::from_set(HashSet::from([chosen_lit])));
        //     let result = self.solve(input, next_chosen_lit);
        //     // input.remove_nth_clause(input.n_clauses);
        //     input.assign_literal(chosen_lit.unsigned_abs(), None);
        //     if let TaskResult::SAT(_) = result { return result }
        // } else {
        //     return if input.is_solved() { self.return_sat_assignment(input) } else { TaskResult::UNSAT("TODO") }
        // }
        //

    fn return_sat_assignment(&self, input: &Task) -> TaskResult {
        TaskResult::SAT(
            input.assignment
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.expect("SAT assignment should have all literals assigned")))
                .collect()
        )
    }
    //
    //
    // pub fn propagate_pure_literals(&self, mut input: &mut Task) {
    //     loop {
    //         let pure_literal = self.find_pure_literal(input);
    //         if pure_literal == 0 { break }
    //         for i in 0..input.n_clauses {
    //             if let Some(&true) = input.clauses[i].literals.get(&pure_literal) {
    //                 input.mask_nth_clause(i);
    //             }
    //         }
    //     }
    // }
    //
    // fn find_pure_literal(&self, input: &Task) -> isize {
    //     let mut assignment_vec: Vec<Option<i8>> = vec![None; input.n_variables];
    //     for (i, clause) in input.clauses.iter().enumerate() {
    //         if !input.is_masked(i) {
    //             for (literal, mask) in clause.literals.iter() {
    //                 let assignment_idx = (literal.abs() - 1) as usize;
    //                 let assignment_value = assignment_vec[assignment_idx];
    //                 if *mask && assignment_value != Some(0) {
    //                     if *literal > 0 {
    //                         if assignment_value == Some(-1) { assignment_vec[assignment_idx] = Some(0); }
    //                         else if assignment_value == None { assignment_vec[assignment_idx] = Some(1); }
    //                     } else if *literal < 0 {
    //                         if assignment_value == Some(1) { assignment_vec[assignment_idx] = Some(0); }
    //                         else if assignment_value == None { assignment_vec[assignment_idx] = Some(-1); }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     for idx in 0..input.n_variables {
    //         if let Some(literal) = assignment_vec[idx] {
    //             if literal > 0 { return (idx + 1) as isize }
    //             else if literal < 0 { return -((idx + 1) as isize) }
    //         }
    //     }
    //     0
    // }
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
        let result = solver.solve(&mut task, 1);
        match result {
            TaskResult::SAT(sat) => {
                task.assignment = sat.into_iter().map(|(k, v)| (k, Some(v))).collect();
                assert_eq!(task.is_solved(), true);
            },
            _ => assert!(false),
        }
    }
}