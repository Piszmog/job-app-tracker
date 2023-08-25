// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result};

use crate::job::{JobApplication, JobApplicationNote, JobApplicationStatusHistory};

mod job;

fn main() -> Result<()> {
    job::init("../data.db3")?;

    tauri::Builder::default()
        .manage(Options {
            path: "../data.db3".to_string(),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_job_applications,
            create_job_application,
            update_job_application_status,
            get_job_application,
            add_job_application_note,
            get_job_application_notes,
            get_job_application_status_histories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running job application tracker");

    Ok(())
}

struct Options {
    path: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_job_applications(opts: tauri::State<Options>) -> Vec<JobApplication> {
    let conn = Connection::open(&opts.path).unwrap();
    job::get_job_applications(&conn).unwrap()
}

#[tauri::command]
fn create_job_application(opts: tauri::State<Options>, company: &str, title: &str, url: &str) -> JobApplication {
    let mut conn = Connection::open(&opts.path).unwrap();
    job::create_job_application(&mut conn, company, title, url).unwrap()
}

#[tauri::command]
fn update_job_application_status(opts: tauri::State<Options>, id: i32, status: job::JobApplicationStatus) -> JobApplication {
    let mut conn = Connection::open(&opts.path).unwrap();
    job::update_job_application_status(&mut conn, id, status).unwrap()
}

#[tauri::command]
fn get_job_application(opts: tauri::State<Options>, id: i32) -> JobApplication {
    let conn = Connection::open(&opts.path).unwrap();
    job::get_job_application(&conn, id).unwrap()
}

#[tauri::command]
fn add_job_application_note(opts: tauri::State<Options>, id: i32, note: &str) {
    let conn = Connection::open(&opts.path).unwrap();
    job::add_job_application_note(&conn, id, note).unwrap()
}

#[tauri::command]
fn get_job_application_notes(opts: tauri::State<Options>, id: i32) -> Vec<JobApplicationNote> {
    let conn = Connection::open(&opts.path).unwrap();
    job::get_job_application_notes(&conn, id).unwrap()
}

#[tauri::command]
fn get_job_application_status_histories(opts: tauri::State<Options>, id: i32) -> Vec<JobApplicationStatusHistory> {
    let conn = Connection::open(&opts.path).unwrap();
    job::get_job_application_status_histories(&conn, id).unwrap()
}
