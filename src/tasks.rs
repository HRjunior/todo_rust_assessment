use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Rewind the file before reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Consume the file's contents as a vector of tasks.
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Try to remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Cette tache ne semble pas exister..."));
    }
    tasks.remove(task_position - 1);

    // Write the modified task list back into the file.
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tak(journal_path: PathBuf) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Parse the file and collect the tasks.
    let tasks: Vec<Task> = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("IL semblerait qu'il n'y ait aucune tâche... Pense à en ajouter !");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}- {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at_hour = self.created_at.with_timezone(&Local).format("%H");
        let created_at_minute = self.created_at.with_timezone(&Local).format("%M");

        let created_at_date = self.created_at.with_timezone(&Local).format("%F");
        write!(f, "{:<50} Tache crée à {}h{} le [{}]", self.text, created_at_hour,created_at_minute, created_at_date)
    }
}