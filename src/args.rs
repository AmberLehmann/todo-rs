use clap::{
    Parser,
    Subcommand,
    Args,
};


/// Task Manager
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct UserArgs {
    #[clap(subcommand)]
    pub command: UserOption
}

#[derive(Debug, Subcommand)]
pub enum UserOption {
    /// Add new Todo item
    Add(AddItem),
    /// Remove Todo item
    Remove(RemoveItem),
    /// Show all Todo items
    Show,
}

#[derive(Debug, Args)]
pub struct AddItem {
    /// Name of task
    pub task: String,
    /// Due Date
    pub due_date: String,
}

#[derive(Debug, Args)]
pub struct RemoveItem {
    /// Number of item to remove
    pub number: u8,
}


