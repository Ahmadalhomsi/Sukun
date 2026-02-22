use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Sqlite, SqlitePool};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrayerTime {
    pub id: Option<i64>,
    pub name: String,
    pub time: String,
    pub date: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrayerLog {
    pub id: Option<i64>,
    pub prayer_name: String,
    pub scheduled_time: String,
    pub executed_at: String,
    pub action: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppSettings {
    pub id: Option<i64>,
    pub key: String,
    pub value: String,
    pub updated_at: Option<String>,
}

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        let db_url = format!("sqlite://{}", db_path.display());

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await?;
        }

        let pool = SqlitePool::connect(&db_url).await?;

        let db = Database { pool };
        db.init_schema().await?;
        Ok(db)
    }

    async fn init_schema(&self) -> Result<()> {
        // Create prayer_times table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS prayer_times (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                time TEXT NOT NULL,
                date TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create prayer_logs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS prayer_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                prayer_name TEXT NOT NULL,
                scheduled_time TEXT NOT NULL,
                executed_at TEXT DEFAULT CURRENT_TIMESTAMP,
                action TEXT NOT NULL,
                success INTEGER NOT NULL DEFAULT 1
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create settings table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT UNIQUE NOT NULL,
                value TEXT NOT NULL,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Prayer Times operations
    #[allow(dead_code)]
    pub async fn delete_prayer_times_for_date(&self, date: &str) -> Result<()> {
        sqlx::query("DELETE FROM prayer_times WHERE date = ?")
            .bind(date)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_prayer_times(&self, prayers: Vec<PrayerTime>) -> Result<()> {
        for prayer in prayers {
            sqlx::query(
                r#"
                INSERT INTO prayer_times (name, time, date)
                VALUES (?, ?, ?)
                "#,
            )
            .bind(&prayer.name)
            .bind(&prayer.time)
            .bind(&prayer.date)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn get_prayer_times_for_date(&self, date: &str) -> Result<Vec<PrayerTime>> {
        let prayers = sqlx::query_as::<_, PrayerTime>(
            r#"
            SELECT id, name, time, date, created_at
            FROM prayer_times
            WHERE date = ?
            ORDER BY time ASC
            "#,
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        Ok(prayers)
    }

    #[allow(dead_code)]
    pub async fn clear_old_prayer_times(&self, before_date: &str) -> Result<()> {
        sqlx::query("DELETE FROM prayer_times WHERE date < ?")
            .bind(before_date)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Prayer Logs operations
    pub async fn insert_prayer_log(&self, log: PrayerLog) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO prayer_logs (prayer_name, scheduled_time, action, success)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&log.prayer_name)
        .bind(&log.scheduled_time)
        .bind(&log.action)
        .bind(log.success)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_recent_logs(&self, limit: i64) -> Result<Vec<PrayerLog>> {
        let logs = sqlx::query_as::<_, PrayerLog>(
            r#"
            SELECT id, prayer_name, scheduled_time, executed_at, action, success
            FROM prayer_logs
            ORDER BY executed_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    pub async fn clear_all_logs(&self) -> Result<()> {
        sqlx::query("DELETE FROM prayer_logs")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_old_logs(&self, days: i64) -> Result<()> {
        let date_threshold = chrono::Local::now()
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap_or_else(chrono::Local::now)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        sqlx::query("DELETE FROM prayer_logs WHERE executed_at < ?")
            .bind(date_threshold)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Settings operations
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO settings (key, value)
            VALUES (?, ?)
            ON CONFLICT(key) DO UPDATE SET value = ?, updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(key)
        .bind(value)
        .bind(value)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let result = sqlx::query_as::<_, AppSettings>(
            r#"
            SELECT id, key, value, updated_at
            FROM settings
            WHERE key = ?
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|s| s.value))
    }

    pub async fn get_all_settings(&self) -> Result<Vec<AppSettings>> {
        let settings = sqlx::query_as::<_, AppSettings>(
            r#"
            SELECT id, key, value, updated_at
            FROM settings
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(settings)
    }
}
