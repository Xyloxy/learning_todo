use std::fmt::{Display, Formatter};
use chrono::{DateTime, FixedOffset, Utc};
use sqlite::{Connection, Value};
use sqlite::State::Done;

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Open = 1,
    HalfDone = 2,
    Done = 3,
    Removed = 4,
}

impl TryFrom<i64> for Status {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x == Status::Open as i64 => Ok(Status::Open),
            x if x == Status::HalfDone as i64 => Ok(Status::HalfDone),
            x if x == Status::Done as i64 => Ok(Status::Done),
            x if x == Status::Removed as i64 => Ok(Status::Removed),
            _ => Err(()),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<FixedOffset>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub complete_before: Option<DateTime<FixedOffset>>,
}

fn setup_db(connection: &Connection) {
    let query = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, title VARCHAR(64), description TEXT, status INTEGER, created_at TEXT, completed_at TEXT, complete_before TEXT);";
    connection.execute(query).unwrap();
}

fn get_db() -> Connection {
    let connection = sqlite::open("./db.sqlite").unwrap();

    setup_db(&connection);

    connection
}

pub fn get_tasks(statuses: &Vec<Status>) -> Vec<Task> {
    if statuses.is_empty() {
        return Vec::new();
    }

    let placeholders = vec!["?"; statuses.len()].join(", ");
    let query = format!("SELECT * FROM tasks WHERE status IN ({placeholders});");

    let con = get_db();
    let mut statement = con.prepare(query).unwrap();

    for (i, status) in statuses.iter().enumerate() {
        statement.bind((i + 1, *status as i64)).unwrap();
    }

    let mut tasks = statement
        .into_iter()
        .map(|row| {
            let task = row.unwrap();
            Task {
                id: task.read::<i64, _>("id") as i32,
                title: task.read::<&str, _>("title").to_owned(),
                description: task.read::<&str, _>("description").to_owned(),
                status: Status::try_from(task.read::<i64, _>("status")).unwrap(),
                created_at: DateTime::parse_from_rfc3339(task.read::<&str, _>("created_at"))
                    .unwrap(),
                completed_at: task
                    .read::<Option<&str>, _>("completed_at")
                    .map(|s| DateTime::parse_from_rfc3339(s).unwrap()),
                complete_before: task
                    .read::<Option<&str>, _>("complete_before")
                    .map(|s| DateTime::parse_from_rfc3339(s).unwrap()),
            }
        })
        .collect::<Vec<Task>>();

    tasks.sort_by(|a, b| a.complete_before.cmp(&b.complete_before));

    tasks
}

pub fn update_task(task_id: i32, status: Status) {
    let query = "UPDATE tasks SET status = ? WHERE id = ?";

    let con = get_db();
    let mut statement = con.prepare(query).unwrap();

    statement
        .bind_iter(vec![(1, status as i64), (2, task_id as i64)])
        .unwrap();

    loop {
        if statement.next().unwrap() == Done {
            break;
        }
    }
}

pub fn create_task(
    title: String,
    description: String,
    complete_before: Option<DateTime<FixedOffset>>,
) {
    let query = "INSERT INTO tasks (title, description, status, created_at, complete_before) VALUES (?, ?, ?, ?, ?)";

    let con = get_db();
    let mut statement = con.prepare(query).unwrap();

    statement.bind_iter::<_, (_, Value)>(vec![
        (1, title.into()),
        (2, description.into()),
        (4, Utc::now().to_rfc3339().into()),
    ]).unwrap();
    statement.bind((3, Status::Open as i64)).unwrap();

    if complete_before.is_some() {
        statement.bind((5, complete_before.unwrap().to_rfc3339().as_str())).unwrap();
    }
    else {
        statement.bind((5, ())).unwrap();
    }

    loop {
        if statement.next().unwrap() == Done {
            break;
        }
    }
}
