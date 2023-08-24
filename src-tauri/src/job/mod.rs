use rusqlite::{Connection, params, Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct JobApplication {
    pub id: i32,
    pub company: String,
    pub title: String,
    pub url: String,
    pub status: JobApplicationStatus,
    pub applied_at: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobApplicationStatus {
    Accepted,
    Applied,
    Declined,
    Interviewing,
    Offered,
    Rejected,
    Watching,
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
        }
    }
}

pub fn init(path: &str) -> Result<()> {
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

    Ok(())
}

pub fn create_job_application(conn: &Connection, company: &str, title: &str, url: &str) -> Result<JobApplication> {
    let mut statement = conn.prepare(
        "INSERT INTO job_applications (company, title, url) VALUES (?1, ?2, ?3) RETURNING id, company, title, url, status, applied_at, updated_at, created_at",
    )?;
    let application = statement.query_row(
        params![
            company,
            title,
            url,
        ],
        scan_job_application,
    )?;
    Ok(application)
}

pub fn update_job_application_status(conn: &Connection, id: i32, status: JobApplicationStatus) -> Result<JobApplication> {
    let mut statement = conn.prepare(
        "UPDATE job_applications SET status = ?1 WHERE id = ?2 RETURNING id, company, title, url, status, applied_at, updated_at, created_at",
    )?;
    let application = statement.query_row(
        params![
            status.to_string(),
            id,
        ],
        scan_job_application,
    )?;
    Ok(application)
}

pub fn get_job_applications(conn: &Connection) -> Result<Vec<JobApplication>> {
    let mut statement = conn.prepare(
        "SELECT * FROM job_applications",
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
    })
}
