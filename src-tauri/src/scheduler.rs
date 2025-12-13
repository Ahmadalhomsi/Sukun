use crate::audio;
use crate::db::{Database, PrayerLog, PrayerTime};
use anyhow::Result;
use chrono::{Local, NaiveDateTime, NaiveTime};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;

pub struct PrayerScheduler {
    db: Arc<Database>,
    app_handle: AppHandle,
}

impl PrayerScheduler {
    pub fn new(db: Arc<Database>, app_handle: AppHandle) -> Self {
        Self { db, app_handle }
    }

    pub async fn start(&self) -> Result<()> {
        loop {
            match self.check_and_execute_prayers().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error in prayer scheduler: {}", e);
                }
            }

            // Check every minute
            sleep(Duration::from_secs(60)).await;
        }
    }

    async fn check_and_execute_prayers(&self) -> Result<()> {
        let now = Local::now();
        let current_date = now.format("%Y-%m-%d").to_string();
        let current_time = now.format("%H:%M").to_string();

        // Get today's prayer times
        let prayers = self.db.get_prayer_times_for_date(&current_date).await?;

        for prayer in prayers {
            if prayer.time == current_time {
                self.execute_prayer_action(&prayer).await?;
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
        if let Err(e) = self.send_notification(&prayer.name, &prayer.time).await {
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

    async fn send_notification(&self, prayer_name: &str, time: &str) -> Result<()> {
        // Use Tauri notification plugin
        #[cfg(feature = "notification")]
        {
            use tauri_plugin_notification::NotificationExt;
            self.app_handle
                .notification()
                .builder()
                .title("Prayer Time")
                .body(format!("It's time for {} prayer at {}", prayer_name, time))
                .show()?;
        }

        Ok(())
    }

    pub fn get_next_prayer(&self, prayers: &[PrayerTime]) -> Option<PrayerTime> {
        let now = Local::now();
        let current_time = now.time();

        prayers
            .iter()
            .filter(|p| {
                if let Ok(prayer_time) = NaiveTime::parse_from_str(&p.time, "%H:%M") {
                    prayer_time > current_time
                } else {
                    false
                }
            })
            .min_by_key(|p| {
                NaiveTime::parse_from_str(&p.time, "%H:%M").unwrap_or(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
            })
            .cloned()
    }
}
