use crate::model::task::Task;

use prettytable::{row, Cell, Row};

#[derive(Debug)]
pub enum ListError {
    InvalidId,
    IdAlreadyExists,
}

pub struct List {
    pub tasks: Vec<Task>,
}

impl List {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: Task) -> Result<(), ListError> {
        match task.id {
            Some(id) => {
                if self.tasks.contains(&task) {
                    return Err(ListError::IdAlreadyExists);
                }
                self.tasks.push(task);
            }
            None => {
                return Err(ListError::InvalidId);
            }
        }
        Ok(())
    }

    pub fn start(&mut self, task: Task) -> Result<(), ListError> {
        match task.id {
            Some(id) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == Some(id)) {
                    task.start();
                }
            }
            None => {
                return Err(ListError::InvalidId);
            }
        }
        Ok(())
    }

    pub fn complete(&mut self, task: Task) -> Result<(), ListError> {
        match task.id {
            Some(id) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == Some(id)) {
                    task.complete();
                }
            }
            None => {
                return Err(ListError::InvalidId);
            }
        }
        Ok(())
    }

    pub fn delete(&mut self, task: Task) -> Result<(), ListError> {
        match task.id {
            Some(id) => {
                self.tasks.retain(|task| task.id != Some(id));
            }
            None => {
                return Err(ListError::InvalidId);
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = prettytable::Table::new();
        table.add_row(row![
            "ID",
            "Name",
            "Description",
            "Status",
            "Started At",
            "Completed At"
        ]);

        for task in &self.tasks {
            let mut row = Row::empty();
            row.add_cell(Cell::new(&task.id.unwrap().to_string()));
            row.add_cell(Cell::new(&task.name));
            row.add_cell(Cell::new(&task.description));
            row.add_cell(Cell::new(&task.status.to_string()));

            match task.started_at {
                Some(started) => {
                    row.add_cell(Cell::new(&started.format("%Y-%m-%d %H:%M:%S").to_string()))
                }
                None => row.add_cell(Cell::new("")),
            }

            match task.completed_at {
                Some(completed) => row.add_cell(Cell::new(
                    &completed.format("%Y-%m-%d %H:%M:%S").to_string(),
                )),
                None => row.add_cell(Cell::new("")),
            }

            table.add_row(row);
        }

        write!(f, "{}", table)
    }
}
