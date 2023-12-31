use std::path::Path;

use rusqlite::{Connection, Error, params, Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    Cancelled,
    Closed,
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
            "cancelled" => Self::Cancelled,
            "closed" => Self::Closed,
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
            Self::Cancelled => "cancelled".to_string(),
            Self::Closed => "closed".to_string(),
            Self::Declined => "declined".to_string(),
            Self::Interviewing => "interviewing".to_string(),
            Self::Offered => "offered".to_string(),
            Self::Rejected => "rejected".to_string(),
            Self::Watching => "watching".to_string(),
            Self::Withdrawn => "withdrawn".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobApplicationNote {
    pub id: i32,
    pub job_application_id: i32,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobApplicationStatusHistory {
    pub id: i32,
    pub job_application_id: i32,
    pub status: JobApplicationStatus,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_applications: i32,
    pub total_companies: i32,
    pub average_time_to_hear_back: f32,
    pub total_interviewing: i32,
    pub total_rejections: i32,
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

pub fn update_job_application(conn: &mut Connection, id: i32, company: &str, title: &str, url: &str, status: JobApplicationStatus) -> Result<JobApplicationStatusHistory> {
    let tx = conn.transaction()?;

    tx.execute(
        "UPDATE job_applications
        SET company = ?1, title = ?2, url = ?3, status = ?4, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?5",
        params![
            company,
            title,
            url,
            status.to_string(),
            id,
        ],
    )?;
    let job_status = {
        tx.execute(
            "INSERT INTO job_application_status_histories (job_application_id, status)
                SELECT ?1, ?2
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM (
                             SELECT status
                             FROM job_application_status_histories
                             WHERE job_application_id = ?1
                             ORDER BY created_at DESC
                             LIMIT 1
                         )
                    WHERE status = ?2
                )",
            params![
                id,
                status.to_string(),
            ],
        )?;
        let mut statement = tx.prepare(
            "SELECT *
                FROM job_application_status_histories
                WHERE job_application_id = ?1 AND status = ?2
                ORDER BY created_at DESC
                LIMIT 1",
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

pub fn import_data(conn: &mut Connection, jobs: Vec<JobApplication>) -> Result<()> {
    let tx = conn.transaction()?;
    // clear data
    tx.execute(
        "DELETE FROM job_applications",
        params![],
    )?;
    tx.execute(
        "DELETE FROM job_application_notes",
        params![],
    )?;
    tx.execute(
        "DELETE FROM job_application_status_histories",
        params![],
    )?;
    for job in jobs {
        let application = {
            let mut statement = tx.prepare(
                "INSERT INTO
            job_applications (company, title, url, status, applied_at, updated_at, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING id, company, title, url, status, applied_at, updated_at, created_at",
            )?;
            statement.query_row(
                params![
                job.company,
                job.title,
                job.url,
                job.status.to_string(),
                job.applied_at,
                job.updated_at,
                job.created_at,
            ],
                scan_job_application,
            )?
        };
        if let Some(notes) = job.notes {
            for note in notes {
                tx.execute(
                    "INSERT INTO
                    job_application_notes (job_application_id, note, created_at)
                    VALUES (?1, ?2, ?3)",
                    params![
                        application.id,
                        note.note,
                        note.created_at,
                    ],
                )?;
            }
        }
        if let Some(statuses) = job.statuses {
            for status in statuses {
                tx.execute(
                    "INSERT INTO
                    job_application_status_histories (job_application_id, status, created_at)
                    VALUES (?1, ?2, ?3)",
                    params![
                        application.id,
                        status.status.to_string(),
                        status.created_at,
                    ],
                )?;
            }
        }
    }
    tx.commit()?;
    Ok(())
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

pub fn get_stats(conn: &Connection) -> Result<Stats> {
    let mut statement = conn.prepare(
        "WITH first_status AS (
            SELECT job_application_id, min(created_at) as first_status_at
            FROM job_application_status_histories
            WHERE status = 'interviewing' or status = 'rejected' or status = 'cancelled' or status = 'closed'
            GROUP BY job_application_id
        )
        SELECT
            (SELECT COUNT(*) FROM job_applications) as total_applications,
            COUNT(DISTINCT company) as total_companies,
            ROUND(AVG(JULIANDAY(fs.first_status_at) - JULIANDAY(ja.applied_at))) as average_time_to_hear_back,
            SUM(CASE WHEN status = 'interviewing' THEN 1 ELSE 0 END) as total_interviewing,
            SUM(CASE WHEN status = 'rejected' THEN 1 ELSE 0 END) as total_rejections
        FROM job_applications ja
        JOIN first_status fs ON ja.id = fs.job_application_id",
    )?;
    let stats = statement.query_row(params![], scan_stats)?;
    Ok(stats)
}

fn scan_stats(row: &Row) -> Result<Stats> {
    Ok(Stats {
        total_applications: row.get(0)?,
        total_companies: row.get(1)?,
        average_time_to_hear_back: row.get(2)?,
        total_interviewing: row.get(3)?,
        total_rejections: row.get(4)?,
    })
}
