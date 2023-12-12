use crate::db::db;
use crate::model::list::List;
use crate::model::status::Status;
use crate::model::task::Task;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, Row};

pub struct Interface {
    pub list: List,
    pub conn: Connection,
}

impl Interface {
    pub fn new() -> Self {
        let db_file_path = "./tasks.db";

        if !std::path::Path::new(&db_file_path).exists() {
            let conn = db::connect();
            _ = db::create_tasks_table(&conn.unwrap());
        }

        // TODO: Load tasks from db

        Self {
            list: List::new(),
            conn: Connection::open(&db_file_path).unwrap(),
        }
    }
}

pub fn add_task(name: String, description: String) -> () {
    let mut interface = Interface::new();

    let mut task = Task::new(name, description);

    _ = db::insert_task(&interface.conn, &task);
    let id = interface.conn.last_insert_rowid() as u32;
    task.id = Some(id);
    _ = interface.list.add(task);

    println!("Task has been added!");
    _ = list_tasks();
}

pub fn start_task(id: u32) -> () {
    println!("Starting task");
    // let mut list = TaskList::new();
    // list.init();
    // list.start(id);
    // println!("{}", list);
    // let mut task = list.start(Soid);
    // db::start(conn, id, &task);
}

pub fn complete_task(id: u32) -> () {
    println!("Completing task");
    // let mut list = TaskList::new();
    // list.init();
    // list.complete(id);
    // println!("{}", list);
    // let mut task = list.start(Soid);
    // db::start(conn, id, &task);
}

pub fn delete_task(id: u32) -> () {
    println!("Deleting task");
    // let mut list = TaskList::new();
    // list.init();
    // list.delete(id);
    // println!("{}", list);
    // if let Some(id) = task.id {
    //     db::delete(conn, id);
    //     list.delete(Some(id));
    // }
}

// fn convert_date(date_string: &str) -> Option<DateTime<Utc>> {
//     let parsed_date = DateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S")
//         .expect("Failed to parse date string");
//
//     let utc_date = Utc
//         .from_local_datetime(&parsed_date.naive_local())
//         .earliest()
//         .expect("Failed to convert to UTC");
//
//     return Some(utc_date);
// }

fn parse_task(row: &Row) -> Result<Task> {
    let id = row.get(0)?;
    let name = row.get(1)?;
    let description = row.get(2)?;
    let status: String = row.get(3)?;
    let status = status.parse::<Status>().unwrap();
    let started_at: Option<DateTime<Utc>> = None;
    let completed_at: Option<DateTime<Utc>> = None;

    Ok(Task {
        id,
        name,
        description,
        status,
        started_at,
        completed_at,
    })
}

pub fn list_tasks() -> Result<()> {
    let mut interface = Interface::new();
    let mut stmt = db::list_tasks(&interface.conn)?;

    let tasks_iter = stmt.query_map([], |row| parse_task(row))?;

    for task in tasks_iter {
        _ = interface.list.add(task.unwrap());
    }

    println!("{}", interface.list);
    Ok(())
}
