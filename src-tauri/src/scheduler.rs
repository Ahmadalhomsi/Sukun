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
const ADHAN_WAV: &[u8] = include_bytes!("../../static/Smart_UI_Notification_Stylized_Calm_19_Menu_UI_Indie_Chill.wav");

struct AlertSettings {
    pre_alert_enabled: bool,
    pre_alert_minutes: i64,
    pre_alert_mode: String,
    notifications_enabled: bool,
	language: String,
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
            println!("📅 New day detected: {}", today);
            pre_alerted.clear();
            main_triggered.clear();
            *current_day = today.clone();
        }

        let settings = self.load_alert_settings().await?;

        // Get today's prayer times
        let prayers = self.db.get_prayer_times_for_date(&today).await?;
        
        if prayers.is_empty() {
            println!("⚠️  No prayer times found for today: {}", today);
        }

        for prayer in prayers {
            let key_base = format!("{}:{}", today, prayer.name);

            let Some(prayer_dt) = self.combine_date_time(&today, &prayer.time) else {
                continue;
            };

            let diff_secs = (prayer_dt - now).num_seconds();
            let diff_mins = diff_secs / 60;

            // Pre-alert window
            if settings.pre_alert_enabled {
                let pre_window = settings.pre_alert_minutes.max(1) * 60;
                println!(
                    "🔔 Checking pre-alert for {}: diff={}s ({}m), window={}s ({}m), mode={}",
                    prayer.name, diff_secs, diff_mins, pre_window, settings.pre_alert_minutes, settings.pre_alert_mode
                );
                
                if diff_secs <= pre_window && diff_secs >= 0 {
                    let pre_key = format!("{}:pre", key_base);
                    if pre_alerted.insert(pre_key) {
                        println!("🔔 TRIGGERING PRE-ALERT for {} ({} minutes before)", prayer.name, diff_mins);
                        self.fire_pre_alert(&prayer, &settings).await?;
                    } else {
                        println!("🔔 Pre-alert already sent for {}", prayer.name);
                    }
                } else if diff_secs > pre_window {
                    println!("🔔 Too early for pre-alert: {} minutes remaining", diff_mins);
                } else {
                    println!("🔔 Too late for pre-alert: prayer time passed");
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
            "⏰ ========== PRAYER TIME TRIGGERED =========="
        );
        println!(
            "⏰ Prayer: {} at {}",
            prayer.name, prayer.time
        );
        println!(
            "⏰ ==========================================="
        );

        let success = true;
        let action = "mute_audio_play_adhan".to_string();

        // Load settings to check if notifications are enabled
        let settings = self.load_alert_settings().await?;
        println!("Notifications enabled: {}", settings.notifications_enabled);

        // Step 1: Mute system audio FIRST
        if let Err(e) = audio::mute_system_audio() {
            eprintln!("Failed to mute audio: {}", e);
        } else {
            println!("🔇 Audio muted successfully");
        }

        // Step 2: Play adhan sound (it will play even when other apps are muted)
        println!("🕌 Playing adhan sound...");
        if let Err(e) = self.play_adhan_sound() {
            eprintln!("❌ Adhan sound failed: {}", e);
        } else {
            println!("✅ Adhan sound playing");
        }

        // Step 3: Send notification if enabled
        if settings.notifications_enabled {
            let translated_name = self.translate_prayer_name(&prayer.name, &settings.language).await;
            let title = if settings.language == "tr" {
                format!("🕌 {} Vakti", translated_name)
            } else {
                format!("🕌 {} Prayer Time", translated_name)
            };
            let body = if settings.language == "tr" {
                format!("Saat {}'de {} vaktinin zamanı geldi", prayer.time, translated_name)
            } else {
                format!("It's time for {} prayer at {}", translated_name, prayer.time)
            };
            println!("Sending notification: {} - {}", title, body);
            
            match self.send_notification(&title, &body).await {
                Ok(_) => {
                    println!("✅ Notification sent successfully!");
                }
                Err(e) => {
                    eprintln!("❌ Notification failed: {}", e);
                }
            }
        } else {
            println!("Notifications disabled in settings, skipping notification");
        }

        // Step 4: Emit event to frontend (if it's open)
        if let Err(e) = self
            .app_handle
            .emit("prayer-time-triggered", &prayer.name)
        {
            eprintln!("Failed to emit event: {}", e);
        } else {
            println!("Event emitted to frontend");
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
        // Use Tauri notification with sound for Windows toast notification
        println!("Attempting to send notification: {} - {}", title, body);
        
        // Build notification with sound to ensure Windows shows it as a toast
        match self.app_handle
            .notification()
            .builder()
            .title(title)
            .body(body)
            .sound("default")
            .show() {
            Ok(_) => {
                println!("✅ Notification sent successfully: {} - {}", title, body);
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to show notification: {}", e);
                Err(anyhow::anyhow!("Notification error: {}", e))
            }
        }
    }

	async fn translate_prayer_name(&self, english_name: &str, language: &str) -> String {
		if language == "tr" {
			match english_name {
				"Fajr" => "İmsak".to_string(),
				"Sunrise" => "Güneş".to_string(),
				"Dhuhr" => "Öğle".to_string(),
				"Asr" => "İkindi".to_string(),
				"Maghrib" => "Akşam".to_string(),
				"Isha" => "Yatsı".to_string(),
				_ => english_name.to_string(),
			}
		} else {
			english_name.to_string()
		}
	}

    async fn fire_pre_alert(&self, prayer: &PrayerTime, settings: &AlertSettings) -> Result<()> {
		let translated_name = self.translate_prayer_name(&prayer.name, &settings.language).await;
		println!("🔔 Firing pre-alert for {}: mode={}", translated_name, settings.pre_alert_mode);
		
		// Always send notification if enabled
		if settings.notifications_enabled {
			let (title, body) = if settings.language == "tr" {
				(
					"🕌 Yaklaşan Namaz".to_string(),
					format!("{} namaz {} dakika sonra saat {}'de", translated_name, settings.pre_alert_minutes, prayer.time)
				)
			} else {
				(
					"🕌 Upcoming Prayer".to_string(),
					format!("{} prayer in {} minutes at {}", translated_name, settings.pre_alert_minutes, prayer.time)
				)
			};
            match self.send_notification(&title, &body).await {
                Ok(_) => println!("✅ Pre-alert notification sent successfully"),
                Err(e) => eprintln!("❌ Pre-alert notification failed: {}", e),
            }
        } else {
            println!("⚠️ Notifications disabled, skipping pre-alert notification");
        }
        
        // Additionally play sound if mode is sound
        if settings.pre_alert_mode.as_str() == "sound" {
            println!("🔊 Playing pre-alert sound...");
            if let Err(e) = self.play_pre_alert_sound() {
                eprintln!("❌ Pre-alert sound failed: {}", e);
            } else {
                println!("✅ Pre-alert sound played successfully");
            }
        }

        Ok(())
    }

    fn play_pre_alert_sound(&self) -> Result<()> {
        // Clone the audio data to move into thread
        let audio_data = PRE_ALERT_WAV.to_vec();
        
        // Play sound in a separate thread to avoid blocking
        std::thread::spawn(move || {
            let cursor = Cursor::new(audio_data);
            if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&stream_handle) {
                    if let Ok(source) = Decoder::new(cursor) {
                        sink.append(source);
                        sink.sleep_until_end();
                    }
                }
            }
        });
        
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

		let language = self
			.db
			.get_setting("language")
			.await?
			.unwrap_or_else(|| "en".to_string());

		println!(
			"⚙️  Loaded settings: pre_alert_enabled={}, pre_alert_minutes={}, pre_alert_mode={}, notifications_enabled={}, language={}",
			pre_alert_enabled, pre_alert_minutes, pre_alert_mode, notifications_enabled, language
		);

		Ok(AlertSettings {
			pre_alert_enabled,
			pre_alert_minutes,
			pre_alert_mode,
			notifications_enabled,
			language,
		})
	}

    fn play_adhan_sound(&self) -> Result<()> {
        // Clone the audio data to move into thread
        let audio_data = ADHAN_WAV.to_vec();
        
        // Play adhan sound in a separate thread
        // This sound will play even when system audio is muted (rodio bypasses system mute)
        std::thread::spawn(move || {
            println!("🕌 Adhan playback thread started");
            let cursor = Cursor::new(audio_data);
            if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&stream_handle) {
                    if let Ok(source) = Decoder::new(cursor) {
                        sink.append(source);
                        println!("🕌 Adhan audio playing...");
                        sink.sleep_until_end();
                        println!("🕌 Adhan audio finished");
                        // Stream and sink will be dropped here, after playback completes
                    }
                }
            }
        });
        
        Ok(())
    }

    fn combine_date_time(&self, date: &str, time: &str) -> Option<chrono::DateTime<Local>> {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
        let time = NaiveTime::parse_from_str(time, "%H:%M").ok()?;
        let naive = NaiveDateTime::new(date, time);
        Local.from_local_datetime(&naive).single()
    }
}
