use crate::audio;
use crate::db::{Database, PrayerLog, PrayerTime};
use anyhow::Result;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashSet;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;
use tokio::time::sleep;

const PRE_ALERT_WAV: &[u8] = include_bytes!("../../static/Smart_UI_Notification_Stylized_Calm_19_Menu_UI_Indie_Chill.wav");

struct AlertSettings {
    pre_alert_enabled: bool,
    pre_alert_minutes: i64,
    pre_alert_mode: String,
    notifications_enabled: bool,
}

pub struct PrayerScheduler {
    db: Arc<Database>,
    app_handle: AppHandle,
}

impl PrayerScheduler {
    pub fn new(db: Arc<Database>, app_handle: AppHandle) -> Self {
        Self { db, app_handle }
    }

    pub async fn start(&self) -> Result<()> {
        let mut pre_alerted: HashSet<String> = HashSet::new();
        let mut main_triggered: HashSet<String> = HashSet::new();
        let mut current_day = Local::now().format("%Y-%m-%d").to_string();

        loop {
            match self
                .check_and_execute_prayers(&mut pre_alerted, &mut main_triggered, &mut current_day)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error in prayer scheduler: {}", e);
                }
            }

            // Check every minute
            sleep(Duration::from_secs(60)).await;
        }
    }

    async fn check_and_execute_prayers(
        &self,
        pre_alerted: &mut HashSet<String>,
        main_triggered: &mut HashSet<String>,
        current_day: &mut String,
    ) -> Result<()> {
        let now = Local::now();
        let today = now.format("%Y-%m-%d").to_string();

        if &today != current_day {
            pre_alerted.clear();
            main_triggered.clear();
            *current_day = today.clone();
        }

        let settings = self.load_alert_settings().await?;

        // Get today's prayer times
        let prayers = self.db.get_prayer_times_for_date(&today).await?;

        for prayer in prayers {
            let key_base = format!("{}:{}", today, prayer.name);

            let Some(prayer_dt) = self.combine_date_time(&today, &prayer.time) else {
                continue;
            };

            let diff_secs = (prayer_dt - now).num_seconds();

            // Pre-alert window
            if settings.pre_alert_enabled {
                let pre_window = settings.pre_alert_minutes.max(1) * 60;
                if diff_secs <= pre_window && diff_secs >= 0 {
                    let pre_key = format!("{}:pre", key_base);
                    if pre_alerted.insert(pre_key) {
                        self.fire_pre_alert(&prayer, &settings).await?;
                    }
                }
            }

            // Main prayer trigger within the minute window
            if diff_secs <= 0 && diff_secs > -60 {
                let main_key = format!("{}:main", key_base);
                if main_triggered.insert(main_key) {
                    self.execute_prayer_action(&prayer).await?;
                }
            }
        }

        Ok(())
    }

    async fn execute_prayer_action(&self, prayer: &PrayerTime) -> Result<()> {
        println!(
            "Executing prayer action for {} at {}",
            prayer.name, prayer.time
        );

        let mut success = true;
        let mut action = "mute_audio".to_string();

        // Mute system audio
        if let Err(e) = audio::mute_system_audio() {
            eprintln!("Failed to mute audio: {}", e);
            success = false;
        }

        // Send notification
        if let Err(e) = self.send_notification(&format!("{} time", prayer.name), &prayer.time).await {
            eprintln!("Failed to send notification: {}", e);
        }

        // Emit event to frontend
        if let Err(e) = self
            .app_handle
            .emit("prayer-time-triggered", &prayer.name)
        {
            eprintln!("Failed to emit event: {}", e);
        }

        // Log the action
        let log = PrayerLog {
            id: None,
            prayer_name: prayer.name.clone(),
            scheduled_time: prayer.time.clone(),
            executed_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action,
            success,
        };

        self.db.insert_prayer_log(log).await?;

        Ok(())
    }

    async fn send_notification(&self, title: &str, body: &str) -> Result<()> {
        // Use Tauri notification builder with proper configuration
        match self.app_handle
            .notification()
            .builder()
            .title(title)
            .body(body)
            .show() {
            Ok(_) => {
                println!("Notification sent: {} - {}", title, body);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to show notification: {}", e);
                Err(anyhow::anyhow!("Notification error: {}", e))
            }
        }
    }

    async fn fire_pre_alert(&self, prayer: &PrayerTime, settings: &AlertSettings) -> Result<()> {
        match settings.pre_alert_mode.as_str() {
            "sound" => {
                if let Err(e) = self.play_pre_alert_sound() {
                    eprintln!("Pre-alert sound failed: {}", e);
                }
            }
            _ => {
                if settings.notifications_enabled {
                    let body = format!(
                        "{} in {} minutes at {}",
                        prayer.name, settings.pre_alert_minutes, prayer.time
                    );
                    let _ = self.send_notification("Upcoming prayer", &body).await;
                }
            }
        }

        Ok(())
    }

    async fn load_alert_settings(&self) -> Result<AlertSettings> {
        let pre_alert_enabled = self
            .db
            .get_setting("pre_prayer_alert_enabled")
            .await?
            .map(|v| v != "false")
            .unwrap_or(true);

        let pre_alert_minutes = self
            .db
            .get_setting("pre_prayer_alert_minutes")
            .await?
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(10)
            .max(1);

        let pre_alert_mode = self
            .db
            .get_setting("pre_prayer_alert_mode")
            .await?
            .unwrap_or_else(|| "notification".to_string());

        let notifications_enabled = self
            .db
            .get_setting("notifications_enabled")
            .await?
            .map(|v| v != "false")
            .unwrap_or(true);

        Ok(AlertSettings {
            pre_alert_enabled,
            pre_alert_minutes,
            pre_alert_mode,
            notifications_enabled,
        })
    }

    fn play_pre_alert_sound(&self) -> Result<()> {
        let cursor = Cursor::new(PRE_ALERT_WAV);
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = Decoder::new(cursor)?;
        sink.append(source);
        sink.sleep_until_end();
        Ok(())
    }

    fn combine_date_time(&self, date: &str, time: &str) -> Option<chrono::DateTime<Local>> {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
        let time = NaiveTime::parse_from_str(time, "%H:%M").ok()?;
        let naive = NaiveDateTime::new(date, time);
        Local.from_local_datetime(&naive).single()
    }
}
