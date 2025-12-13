// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod audio;
mod db;
mod scheduler;

use crate::db::{Database, PrayerLog, PrayerTime};
use crate::scheduler::PrayerScheduler;
use anyhow::Result;
use chrono::Local;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, State, WindowEvent,
};

struct AppState {
    db: Arc<Database>,
}

#[tauri::command]
async fn fetch_and_store_prayer_times(
    state: State<'_, AppState>,
    api_key: String,
    city: String,
    country: String,
    date: String,
) -> Result<Vec<PrayerTime>, String> {
    let response = api::fetch_prayer_times(&api_key, &city, &country, &date)
        .await
        .map_err(|e| e.to_string())?;

    let prayer_times: Vec<PrayerTime> = response
        .prayers
        .into_iter()
        .map(|p| PrayerTime {
            id: None,
            name: p.name,
            time: p.time,
            date: response.date.clone(),
            created_at: None,
        })
        .collect();

    state
        .db
        .insert_prayer_times(prayer_times.clone())
        .await
        .map_err(|e| e.to_string())?;

    Ok(prayer_times)
}

#[tauri::command]
async fn get_prayer_times_for_date(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<PrayerTime>, String> {
    state
        .db
        .get_prayer_times_for_date(&date)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_todays_prayers(state: State<'_, AppState>) -> Result<Vec<PrayerTime>, String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    state
        .db
        .get_prayer_times_for_date(&today)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_recent_logs(
    state: State<'_, AppState>,
    limit: i64,
) -> Result<Vec<PrayerLog>, String> {
    state
        .db
        .get_recent_logs(limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    state
        .db
        .set_setting(&key, &value)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_setting(state: State<'_, AppState>, key: String) -> Result<Option<String>, String> {
    state
        .db
        .get_setting(&key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_settings(state: State<'_, AppState>) -> Result<Vec<db::AppSettings>, String> {
    state.db.get_all_settings().await.map_err(|e| e.to_string())
}

#[tauri::command]
fn mute_audio_now() -> Result<(), String> {
    audio::mute_system_audio().map_err(|e| e.to_string())
}

#[tauri::command]
fn unmute_audio_now() -> Result<(), String> {
    audio::unmute_system_audio().map_err(|e| e.to_string())
}

#[tauri::command]
fn check_audio_mute_status() -> Result<bool, String> {
    audio::is_muted().map_err(|e| e.to_string())
}

fn setup_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "Open Sukun", true, None::<&str>)?;
    let mute = MenuItem::with_id(app, "mute", "Mute Now", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &mute, &quit])?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "mute" => {
                let _ = audio::mute_system_audio();
                println!("Audio muted from tray");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Initialize database
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            let db_path = app_data_dir.join("sukun.db");

            let db = tauri::async_runtime::block_on(async {
                Database::new(db_path)
                    .await
                    .expect("Failed to initialize database")
            });

            let db = Arc::new(db);

            app.manage(AppState { db: db.clone() });

            // Setup system tray
            setup_system_tray(app.handle())?;

            // Start prayer scheduler in background
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let scheduler = PrayerScheduler::new(db, app_handle);
                if let Err(e) = scheduler.start().await {
                    eprintln!("Scheduler error: {}", e);
                }
            });

            // Handle window close event - minimize to tray instead of exit
            let window = app.get_webview_window("main").unwrap();
            let app_handle_for_event = app.handle().clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    // Prevent window from closing, hide it instead
                    api.prevent_close();
                    if let Some(window) = app_handle_for_event.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_and_store_prayer_times,
            get_prayer_times_for_date,
            get_todays_prayers,
            get_recent_logs,
            set_setting,
            get_setting,
            get_all_settings,
            mute_audio_now,
            unmute_audio_now,
            check_audio_mute_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
