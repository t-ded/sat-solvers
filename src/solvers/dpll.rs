use crate::sat_task::task::Task;
use crate::solvers::solution_outcomes::TaskResult;
use crate::solvers::solution_outcomes::TaskResult::UNSAT;
use rand::Rng;

pub struct DPLLSolver {}

impl DPLLSolver {
    pub fn new() -> Self {
        DPLLSolver { }
    }

    pub fn solve(&self, input: &mut Task, chosen_lit: usize) -> TaskResult {
        if input.is_solved() {
            self.return_sat_assignment(&input)
        } else if input.is_solvable() && chosen_lit <= input.n_variables {
            input.assign_literal(chosen_lit, Some(true));
            let result = self.solve(input, chosen_lit + 1);
            if let TaskResult::SAT(_) = result { return result }

            input.assign_literal(chosen_lit, Some(false));
            let result = self.solve(input, chosen_lit + 1);
            if let TaskResult::SAT(_) = result { return result }

            input.assign_literal(chosen_lit, None);
            UNSAT("TODO")
        } else {
            println!("Not solvable: {:?}", input.assignment);
            UNSAT("TODO")
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
    //
    // pub fn unit_propagation(&self, mut input: &mut Task) {
    //     loop {
    //         let unit_literal = self.find_unit_literal(input);
    //         if unit_literal == 0 { break }
    //         for i in 0..input.n_clauses {
    //             if let Some(&true) = input.clauses[i].literals.get(&unit_literal) {
    //                 input.mask_nth_clause(i);
    //             }
    //             if let Some(&true) = input.clauses[i].literals.get(&(-unit_literal)) {
    //                 input.clauses[i].mask_literal(-unit_literal);
    //             }
    //         }
    //     }
    // }
    //
    // fn find_unit_literal(&self, input: &Task) -> isize {
    //     for (i, clause) in input.clauses.iter().enumerate() {
    //         if input.is_masked(i) { continue };
    //         if clause.n_literals == 1 {
    //             for (literal, mask) in clause.literals.iter() {
    //                 if *mask {
    //                     return *literal
    //                 }
    //             }
    //         }
    //     }
    //     0
    // }
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