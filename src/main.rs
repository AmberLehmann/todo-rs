const PATH: &str = "/home/amber/dev/todo/todo.csv";
// const PATH_SCHOOL: &str = "/home/amber/dev/todo/todo_school.csv";

mod args;

use args::{AddItem, RemoveItem, UserArgs, UserOption};
use clap::{error::Result, Parser};
use csv::{Writer, Reader};
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use tabled::{
    Tabled, Table, 
    settings::{
        Color, 
        style::{BorderColor, Style},
        object::{Columns, Rows},
    },
};


#[derive(Serialize, Deserialize, Debug, Tabled)]
struct TodoItem {
    num: u8,
    task: String,
    due: String,
}

fn add_todo(item: AddItem) -> Result<(), Box<dyn Error>>{
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(PATH)
        .unwrap();
    let mut writer = Writer::from_writer(file);
    let mut reader = Reader::from_path(PATH)?;
    let todo = TodoItem {
        num: reader.records().count() as u8 + 1,
        task: item.task,
        due: item.due,
    };

    writer.write_record(&[todo.num.to_string(), todo.task, todo.due])?;
    writer.flush()?;
    Ok(())
}

fn remove_todo(item: RemoveItem) -> Result<(), Box<dyn Error>>{
    let mut reader = Reader::from_path(PATH)?;
    let mut todo_vec: Vec<TodoItem> = Vec::new();
    let mut iter = reader.deserialize();
    while let Some(result) = iter.next() {
        let record: TodoItem = result?;
        if record.num == item.num {
            continue;
        }
        todo_vec.push(record);
    }
    let mut writer = Writer::from_path(PATH)?;
    writer.write_record(&["number", "task", "due_date"])?;
    for (todo_num, todo) in todo_vec.into_iter().enumerate() {
        writer.serialize((todo_num + 1, todo.task, todo.due))?;
    }
    writer.flush()?;
    Ok(())
}

fn show_todo() -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(PATH)?;
    let mut todo_vec: Vec<TodoItem> = Vec::new();
    let mut iter = reader.deserialize();
    while let Some(result) = iter.next() {
        let record: TodoItem = result?;
        println!("{:?}", record);
        todo_vec.push(record);
    }
    let clr_red = Color::FG_RED;
    let clr_bright_magenta = Color::FG_BRIGHT_MAGENTA;
    let clr_cyan = Color::FG_CYAN;
    let clr_green = Color::FG_GREEN;
    // let clr_magenta = Color::FG_MAGENTA;

    let border = BorderColor::new()
        .bottom(clr_green.clone())
        .corner_bottom_left(clr_green.clone())
        .corner_bottom_right(clr_green);

    let mut table = Table::new(todo_vec);
    table
        .with(Style::rounded())
        .modify(Rows::first(), border)
        .modify(Columns::single(0), clr_cyan)
        .modify(Columns::single(1), clr_bright_magenta)
        .modify(Columns::single(2), clr_red);
    println!("{}", table);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = UserArgs::parse();
    println!("{:?}", args);
    match args.command {
        UserOption::Add(item) => add_todo(item),
        UserOption::Remove(item) => remove_todo(item),
        UserOption::Show => show_todo(),
    } 
}
