use crate::crud::{Status, Task, create_task, get_tasks, update_task};
use chrono::{DateTime, FixedOffset, TimeDelta, Utc};

pub mod crud;

fn number_to_time_format(number: i64) -> String {
    if number < 10 {
        return format!("0{number}");
    }
    number.to_string()
}

fn render_tasks(statuses: &Vec<Status>, show_time: bool) -> Vec<Task> {
    print!("\x1B[2J"); // Clear screen
    println!("[XyToDo]");

    println!("Tasks to accomplish:");

    println!("Id\t|\tName\t\t\t|\tStatus\t\t|\tTime left");

    let tasks = get_tasks(&statuses);

    for (i, task) in tasks.iter().enumerate() {
        let mut time_left = "-".to_owned();
        if task.complete_before.is_some() && show_time {
            let mut time_delta = task.complete_before.unwrap().naive_utc() - Utc::now().naive_utc();

            let days = time_delta.num_days();
            time_delta -= TimeDelta::new(days*60*60*24, 0).unwrap();
            let hours = number_to_time_format(time_delta.num_hours());
            time_delta -= TimeDelta::new(time_delta.num_hours()*60*60, 0).unwrap();
            let minutes = number_to_time_format(time_delta.num_minutes());
            time_delta -= TimeDelta::new(time_delta.num_minutes()*60, 0).unwrap();
            let seconds = number_to_time_format(time_delta.num_seconds());

            if task.complete_before.unwrap().naive_utc() < Utc::now().naive_utc() {
                time_left = format!("- {days}d, {hours}:{minutes}:{seconds}");
            } else {
                time_left = format!("{days}d, {hours}:{minutes}:{seconds}");
            }
        }
        println!(
            "{}",
            format!(
                "{i}.\t|\t{title}\t\t|\t{status}\t|\t{time_left}",
                i = i + 1,
                title = task.title,
                time_left = time_left,
                status = task.status,
            )
        );
    }

    tasks
}

fn select_option() -> i8 {
    let mut buffer = String::new();
    let result: i8;

    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().parse::<i8>() {
            Ok(value) => {
                result = value;
                break;
            }
            Err(_error) => {
                println!("Incorrect option");
                buffer = String::new();
            }
        }
    }

    result
}

fn read_text() -> String {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

fn render_options() {
    println!("1. Change task status");
    println!("2. Remove task");
    println!("3. Change shown tasks");
    println!("4. Add task");
    println!("0. Exit");
}

fn select_task(tasks: Vec<Task>) -> i32 {
    println!("0. Leave");
    println!("Select task number");
    let mut task_num;
    loop {
        task_num = select_option();

        if task_num <= tasks.len() as i8 && task_num >= 0 {
            break;
        } else {
            println!("Invalid task number");
        }
    }

    tasks[(task_num - 1) as usize].id
}

fn main() {
    let mut task_filters = vec![Status::Open, Status::HalfDone];
    let mut show_time = true;

    loop {
        let tasks = render_tasks(&task_filters, show_time); 
        render_options();

        match select_option() {
            0 => break,
            1 => {
                let task_id = select_task(tasks);

                if task_id != 0 {
                    println!("0. Leave");
                    println!("1. Set status to Open");
                    println!("2. Set status to Half-Done");
                    println!("3. Set status to Done");

                    loop {
                        match select_option() {
                            0 => break,
                            1 => {
                                update_task(task_id, Status::Open);
                                break;
                            },
                            2 => {
                                update_task(task_id, Status::HalfDone);
                                break;
                            },
                            3 => {
                                update_task(task_id, Status::Done);
                                break;
                            },
                            _ => println!("Invalid option"),
                        }
                    }
                }
            }
            2 => {
                let task_id = select_task(tasks);

                update_task(task_id, Status::Removed)
            }
            3 => {
                println!("0. Leave");
                println!("1. Show Open or Half-Done");
                println!("2. Show Completed");
                println!("3. Show Removed");
                println!("4. Show All");

                loop {
                    match select_option() {
                        0 => break,
                        1 => {
                            task_filters = vec![Status::Open, Status::HalfDone];
                            show_time = true;
                            break;
                        }
                        2 => {
                            task_filters = vec![Status::Done];
                            show_time = false;
                            break;
                        }
                        3 => {
                            task_filters = vec![Status::Removed];
                            show_time = false;
                            break;
                        }
                        4 => {
                            task_filters = vec![
                                Status::Open,
                                Status::HalfDone,
                                Status::Done,
                                Status::Removed,
                            ];
                            show_time = false;
                            break;
                        }
                        _ => println!("Invalid option"),
                    }
                }
            }
            4 => {
                println!("Set title:");
                let title = read_text();
                println!("Set description");
                let description = read_text();

                let mut time: Option<DateTime<FixedOffset>> = None;
                println!(
                    "Set when it has to be complete. Use standard utc format yyyy-mm-ddThh:mm:ssZ, inputting nothing will be treated as no completion time"
                );
                loop {
                    let time_str = read_text();

                    if time_str.is_empty() {
                        break;
                    }

                    match DateTime::parse_from_rfc3339(time_str.as_str()) {
                        Ok(value) => {
                            time = Option::from(value);
                            break;
                        }
                        Err(_error) => {
                            println!("Incorrect time format")
                        }
                    }
                }

                create_task(title, description, time);
            }
            _ => println!("Invalid option"),
        }
    }
}