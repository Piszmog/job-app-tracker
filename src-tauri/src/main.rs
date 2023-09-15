// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use rusqlite::{Connection, Result};
use tauri::{AppHandle, Manager, State};

use crate::job::{JobApplication, JobApplicationNote, JobApplicationStatusHistory};

pub(crate) mod job;

fn main() -> Result<()> {
    tauri::Builder::default()
        .manage(AppState { conn: Default::default() })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_job_applications,
            create_job_application,
            update_job_application,
            get_job_application,
            add_job_application_note,
            get_job_application_notes,
            get_job_application_status_histories,
            get_all_data,
            import_data,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let app_state: State<AppState> = app_handle.state();

            let app_dir = app_handle.path_resolver().app_data_dir().expect("failed to get app data dir");
            fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let sqlite_path = app_dir.join("JobAppTracker.sqlite");
            let conn = job::init(sqlite_path)?;
            *app_state.conn.lock().unwrap() = Some(conn);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running job application tracker");

    Ok(())
}

pub struct AppState {
    pub conn: std::sync::Mutex<Option<Connection>>,
}

pub trait ServiceAccess {
    fn conn<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult;
    fn conn_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn conn<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult {
        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state.conn.lock().unwrap();
        let db = db_connection_guard.as_ref().unwrap();

        operation(db)
    }

    fn conn_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.conn.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_job_applications(app_handle: AppHandle) -> Vec<JobApplication> {
    app_handle.conn(|conn| job::get_job_applications(conn)).unwrap()
}

#[tauri::command]
fn create_job_application(app_handle: AppHandle, company: &str, title: &str, url: &str) -> JobApplication {
    app_handle.conn_mut(|conn| job::create_job_application(conn, company, title, url)).unwrap()
}

#[tauri::command]
fn update_job_application(app_handle: AppHandle, id: i32, company: &str, title: &str, url: &str, status: job::JobApplicationStatus) -> JobApplicationStatusHistory {
    app_handle.conn_mut(|conn| job::update_job_application(conn, id, company, title, url, status)).unwrap()
}

#[tauri::command]
fn get_job_application(app_handle: AppHandle, id: i32) -> JobApplication {
    app_handle.conn(|conn| job::get_job_application(conn, id)).unwrap()
}

#[tauri::command]
fn add_job_application_note(app_handle: AppHandle, id: i32, note: &str) -> JobApplicationNote {
    app_handle.conn_mut(|conn| job::add_job_application_note(conn, id, note)).unwrap()
}

#[tauri::command]
fn get_job_application_notes(app_handle: AppHandle, id: i32) -> Vec<JobApplicationNote> {
    app_handle.conn(|conn| job::get_job_application_notes(conn, id)).unwrap()
}

#[tauri::command]
fn get_job_application_status_histories(app_handle: AppHandle, id: i32) -> Vec<JobApplicationStatusHistory> {
    app_handle.conn(|conn| job::get_job_application_status_histories(conn, id)).unwrap()
}

#[tauri::command]
fn get_all_data(app_handle: AppHandle) -> Vec<JobApplication> {
    app_handle.conn(|conn| job::get_all_data(conn)).unwrap()
}

#[tauri::command]
fn import_data(app_handle: AppHandle, data: Vec<JobApplication>) {
    app_handle.conn_mut(|conn| job::import_data(conn, data)).unwrap()
}
