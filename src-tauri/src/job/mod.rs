use std::path::Path;

use rusqlite::{Connection, Error, params, Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobApplication {
    pub id: i32,
    pub company: String,
    pub title: String,
    pub url: String,
    pub status: JobApplicationStatus,
    pub applied_at: String,
    pub updated_at: String,
    pub created_at: String,
    pub notes: Option<Vec<JobApplicationNote>>,
    pub statuses: Option<Vec<JobApplicationStatusHistory>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobApplicationStatus {
    Accepted,
    Applied,
    Declined,
    Interviewing,
    Offered,
    Rejected,
    Watching,
    Withdrawn,
}

impl JobApplicationStatus {
    pub fn from_str(s: String) -> Self {
        match s.as_str() {
            "accepted" => Self::Accepted,
            "applied" => Self::Applied,
            "declined" => Self::Declined,
            "interviewing" => Self::Interviewing,
            "offered" => Self::Offered,
            "rejected" => Self::Rejected,
            "watching" => Self::Watching,
            "withdrawn" => Self::Withdrawn,
            _ => Self::Applied,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Accepted => "accepted".to_string(),
            Self::Applied => "applied".to_string(),
            Self::Declined => "declined".to_string(),
            Self::Interviewing => "interviewing".to_string(),
            Self::Offered => "offered".to_string(),
            Self::Rejected => "rejected".to_string(),
            Self::Watching => "watching".to_string(),
            Self::Withdrawn => "withdrawn".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobApplicationNote {
    pub id: i32,
    pub job_application_id: i32,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobApplicationStatusHistory {
    pub id: i32,
    pub job_application_id: i32,
    pub status: JobApplicationStatus,
    pub created_at: String,
}

pub fn init<P: AsRef<Path>>(path: P) -> Result<Connection, Error> {
    // TODO foreign key constraints
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_applications (
            id INTEGER PRIMARY KEY,
            company TEXT NOT NULL,
            title TEXT NOT NULL,
            url TEXT,
            status TEXT NOT NULL DEFAULT 'applied',
            applied_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_application_notes (
            id INTEGER PRIMARY KEY,
            job_application_id INTEGER NOT NULL,
            note TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (job_application_id) REFERENCES job_applications(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_application_status_histories (
            id INTEGER PRIMARY KEY,
            job_application_id INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'applied',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (job_application_id) REFERENCES job_applications(id) ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(conn)
}

pub fn create_job_application(conn: &mut Connection, company: &str, title: &str, url: &str) -> Result<JobApplication> {
    let tx = conn.transaction()?;

    let application = {
        let mut statement = tx.prepare(
            "INSERT INTO
        job_applications (company, title, url)
        VALUES (?1, ?2, ?3)
        RETURNING id, company, title, url, status, applied_at, updated_at, created_at",
        )?;
        statement.query_row(
            params![
            company,
            title,
            url,
        ],
            scan_job_application,
        )?
    };
    tx.execute(
        "INSERT INTO
        job_application_status_histories (job_application_id)
        VALUES (?1)",
        params![
            application.id,
        ],
    )?;
    tx.commit()?;

    Ok(application)
}

pub fn update_job_application_status(conn: &mut Connection, id: i32, status: JobApplicationStatus) -> Result<JobApplicationStatusHistory> {
    let tx = conn.transaction()?;

    tx.execute(
        "UPDATE job_applications
        SET status = ?1, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?2",
        params![
            status.to_string(),
            id,
        ],
    )?;
    let job_status = {
        let mut statement = tx.prepare(
            "INSERT INTO
        job_application_status_histories (job_application_id, status)
        VALUES (?1, ?2)
        RETURNING id, job_application_id, status, created_at",
        )?;
        statement.query_row(
            params![
            id,
            status.to_string(),
        ],
            scan_job_application_status_history,
        )?
    };
    tx.commit()?;

    Ok(job_status)
}

pub fn get_job_applications(conn: &Connection) -> Result<Vec<JobApplication>> {
    let mut statement = conn.prepare(
        "SELECT * FROM job_applications
        ORDER BY updated_at DESC",
    )?;
    let rows = statement.query_map(params![], scan_job_application)?;
    let mut applications = Vec::new();
    for row in rows {
        let application = row?;
        applications.push(application);
    }
    Ok(applications)
}

pub fn get_job_application(conn: &Connection, id: i32) -> Result<JobApplication> {
    let mut statement = conn.prepare(
        "SELECT * FROM job_applications WHERE id = ?1",
    )?;
    let application = statement.query_row(params![id], scan_job_application)?;
    Ok(application)
}

pub fn add_job_application_note(conn: &Connection, id: i32, note: &str) -> Result<JobApplicationNote> {
    let mut statement = conn.prepare(
        "INSERT INTO
        job_application_notes (job_application_id, note)
        VALUES (?1, ?2)
        RETURNING id, job_application_id, note, created_at"
    )?;
    let application_note = statement.query_row(
        params![
            id,
            note,
        ],
        scan_job_application_note,
    )?;
    Ok(application_note)
}

fn scan_job_application(row: &Row) -> Result<JobApplication> {
    Ok(JobApplication {
        id: row.get(0)?,
        company: row.get(1)?,
        title: row.get(2)?,
        url: row.get(3)?,
        status: JobApplicationStatus::from_str(row.get(4)?),
        applied_at: row.get(5)?,
        updated_at: row.get(6)?,
        created_at: row.get(7)?,
        notes: None,
        statuses: None,
    })
}

pub fn get_job_application_notes(conn: &Connection, id: i32) -> Result<Vec<JobApplicationNote>> {
    let mut statement = conn.prepare(
        "SELECT * FROM job_application_notes
        WHERE job_application_id = ?1
        ORDER BY created_at DESC",
    )?;
    let rows = statement.query_map(params![id], scan_job_application_note)?;
    let mut notes = Vec::new();
    for row in rows {
        let note = row?;
        notes.push(note);
    }
    Ok(notes)
}

pub fn get_all_data(conn: &Connection) -> Result<Vec<JobApplication>> {
    let applications = get_job_applications(conn).unwrap();
    let mut data = Vec::new();
    for app in applications {
        let mut app = app;
        app.notes = Some(get_job_application_notes(conn, app.id)?);
        app.statuses = Some(get_job_application_status_histories(conn, app.id)?);
        data.push(app);
    }
    Ok(data)
}

fn scan_job_application_note(row: &Row) -> Result<JobApplicationNote> {
    Ok(JobApplicationNote {
        id: row.get(0)?,
        job_application_id: row.get(1)?,
        note: row.get(2)?,
        created_at: row.get(3)?,
    })
}

pub fn get_job_application_status_histories(conn: &Connection, id: i32) -> Result<Vec<JobApplicationStatusHistory>> {
    let mut statement = conn.prepare(
        "SELECT * FROM job_application_status_histories
        WHERE job_application_id = ?1
        ORDER BY created_at DESC",
    )?;
    let rows = statement.query_map(params![id], scan_job_application_status_history)?;
    let mut histories = Vec::new();
    for row in rows {
        let history = row?;
        histories.push(history);
    }
    Ok(histories)
}

fn scan_job_application_status_history(row: &Row) -> Result<JobApplicationStatusHistory> {
    Ok(JobApplicationStatusHistory {
        id: row.get(0)?,
        job_application_id: row.get(1)?,
        status: JobApplicationStatus::from_str(row.get(2)?),
        created_at: row.get(3)?,
    })
}
