use crate::sat_task::task::Clause;
use crate::sat_task::task::Task;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;


pub struct Parser {}

impl Parser {

    pub fn parse_file<P: AsRef<Path>>(&self, file_path: P) -> io::Result<Task> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let task = self.parse_str(&contents);
        Ok(task)
    }

    pub fn parse_str(&self, input: &str) -> Task {
        let mut input_lines = input.lines().peekable();
        while let Some(nxt) = input_lines.peek() {
            if (*nxt).starts_with("p") { break }
            input_lines.next();
        }
        let mut header = input_lines.next().unwrap().split_whitespace();
        let n_variables = header.nth(2).unwrap().parse().unwrap();
        let n_clauses = header.next().unwrap().parse().unwrap();

        let mut task = Task::empty(n_variables, n_clauses);
        for line in input_lines {
            if line.trim().starts_with("%") { break }
            task.clauses.push(
                Clause::from_literal_iter(
                    line.rsplit_once('0')
                        .unwrap()
                        .0
                        .split_whitespace()
                        .map(|x| x.parse::<isize>().unwrap())
                )
            );
        }
        task
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::zip;

    #[test]
    fn test_parse_basic_str() {
        let parser = Parser {};
        let task = parser.parse_str(
            "p cnf 5 3
            1 -5 4 0
            -1 5 3 4 0
            -3 -4 0
            %6 10 -11"
        );
        let expected = [
            HashSet::from_iter([1, -5, 4]),
            HashSet::from_iter([-1, 5, 3, 4]),
            HashSet::from_iter([-3, -4])
        ];
        for (clause, expected_literals) in zip(&task.clauses, expected.iter()) {
            assert_eq!(&clause.literals.clone().into_iter().collect::<HashSet<isize>>(), expected_literals)
        }
    }
}