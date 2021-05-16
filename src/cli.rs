use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write a task to the journal file.
    Ajouter {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Finie {
        /// Position of a task in the list.
        #[structopt()]
        task_position: usize,
    },
    /// List all tasks in the journal file.
    Lister,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty journal", about = "Application CLI pour lister des tâches")]
pub struct CommandLineArgs {
    /// A cli sub-command
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}