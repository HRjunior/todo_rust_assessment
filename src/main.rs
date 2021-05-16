mod cli;
mod tasks;

use anyhow::anyhow;
use cli::{Action::Ajouter, Action::Finie, Action::Lister, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command line args
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack thr journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Il semblerait que le fichier journal n'existe pas... vérifie le nom !"))?;

    //Perform the action.
    match action {
        Ajouter { task } => tasks::add_task(journal_file, Task::new(task)),
        Finie { task_position } => tasks::complete_task(journal_file, task_position),
        Lister => tasks::list_tak(journal_file),
    }?;

    Ok(())
}