use sat_solvers;
use sat_solvers::parser::parser::Parser;
use sat_solvers::solvers::dpll::DPLLSolver;
use sat_solvers::solvers::solution_outcomes::TaskResult;
use std::fs;
use std::path::Path;

#[test]
fn test_sat_problems() {
    let sat_instances_dir = Path::new("tests/sat_instances");
    assert!(sat_instances_dir.exists(), "Directory `sat_instances` not found in the current working directory");

    let parser = Parser {};

    for entry in fs::read_dir(sat_instances_dir).expect("Failed to read 'sat_instances' directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            let mut solver = DPLLSolver::new();
            let mut task = parser.parse_file(path).expect("Failed to parse");
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
}


#[test]
fn test_unsat_problems() {
    let unsat_instances_dir = Path::new("tests/unsat_instances");
    assert!(unsat_instances_dir.exists(), "Directory `unsat_instances` not found in the current working directory");

    let parser = Parser {};

    for entry in fs::read_dir(unsat_instances_dir).expect("Failed to read 'unsat_instances' directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            let mut solver = DPLLSolver::new();
            let mut task = parser.parse_file(path).expect("Failed to parse");
            let result = solver.solve(&mut task, 1);
            match result {
                TaskResult::UNSAT("TODO") => { assert!(true) },
                _ => assert!(false),
            }
        }
    }
}