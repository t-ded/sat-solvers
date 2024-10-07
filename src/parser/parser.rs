use crate::sat_task::task::Task;


pub struct Parser {}

impl Parser {
    fn parse_file(&self, path: &str) -> Task {
        Task::empty()
    }

    fn parse_str(&self, input: &str) -> Task {
        Task::empty()
    }
}