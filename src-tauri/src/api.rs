use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimesResponse {
    pub date: String,
    pub prayers: Vec<Prayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prayer {
    pub name: String,
    pub time: String,
}

// Aladhan API response structures
#[derive(Debug, Deserialize)]
struct AladhanResponse {
    data: AladhanData,
}

#[derive(Debug, Deserialize)]
struct AladhanData {
    timings: AladhanTimings,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct AladhanTimings {
    Fajr: String,
    Sunrise: String,
    Dhuhr: String,
    Asr: String,
    Maghrib: String,
    Isha: String,
}

/// Fetch prayer times from Aladhan API (free, no API key required)
pub async fn fetch_prayer_times(
    _api_key: &str,
    city: &str,
    country: &str,
    date: &str,
) -> Result<PrayerTimesResponse> {
    let client = reqwest::Client::new();
    
    // Parse date to get day, month, year
    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    let day = parsed_date.format("%d").to_string();
    let month = parsed_date.format("%m").to_string();
    let year = parsed_date.format("%Y").to_string();

    // Aladhan API endpoint
    let url = format!(
        "https://api.aladhan.com/v1/timingsByCity/{}-{}-{}?city={}&country={}&method=13",
        day, month, year, city, country
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<AladhanResponse>()
        .await?;

    let timings = response.data.timings;

    // Extract just the time portion (remove timezone info if present)
    let clean_time = |time: &str| -> String {
        time.split_whitespace().next().unwrap_or(time).to_string()
    };

    let prayers = vec![
        Prayer {
            name: "Fajr".to_string(),
            time: clean_time(&timings.Fajr),
        },
        Prayer {
            name: "Sunrise".to_string(),
            time: clean_time(&timings.Sunrise),
        },
        Prayer {
            name: "Dhuhr".to_string(),
            time: clean_time(&timings.Dhuhr),
        },
        Prayer {
            name: "Asr".to_string(),
            time: clean_time(&timings.Asr),
        },
        Prayer {
            name: "Maghrib".to_string(),
            time: clean_time(&timings.Maghrib),
        },
        Prayer {
            name: "Isha".to_string(),
            time: clean_time(&timings.Isha),
        },
    ];

    Ok(PrayerTimesResponse {
        date: date.to_string(),
        prayers,
    })
}

/// Fetch prayer times from Aladhan API using coordinates
pub async fn fetch_prayer_times_by_coords(
    latitude: f64,
    longitude: f64,
    date: &str,
    method: u32,
) -> Result<PrayerTimesResponse> {
    let client = reqwest::Client::new();

    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    let day = parsed_date.format("%d").to_string();
    let month = parsed_date.format("%m").to_string();
    let year = parsed_date.format("%Y").to_string();

    let url = format!(
        "https://api.aladhan.com/v1/timings/{}-{}-{}?latitude={}&longitude={}&method={}",
        day, month, year, latitude, longitude, method
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<AladhanResponse>()
        .await?;

    let timings = response.data.timings;

    let clean_time = |time: &str| -> String {
        time.split_whitespace().next().unwrap_or(time).to_string()
    };

    let prayers = vec![
        Prayer { name: "Fajr".to_string(), time: clean_time(&timings.Fajr) },
        Prayer { name: "Sunrise".to_string(), time: clean_time(&timings.Sunrise) },
        Prayer { name: "Dhuhr".to_string(), time: clean_time(&timings.Dhuhr) },
        Prayer { name: "Asr".to_string(), time: clean_time(&timings.Asr) },
        Prayer { name: "Maghrib".to_string(), time: clean_time(&timings.Maghrib) },
        Prayer { name: "Isha".to_string(), time: clean_time(&timings.Isha) },
    ];

    Ok(PrayerTimesResponse {
        date: date.to_string(),
        prayers,
    })
}

/// Validate prayer time format (HH:MM)
#[allow(dead_code)]
pub fn validate_time_format(time: &str) -> bool {
    NaiveTime::parse_from_str(time, "%H:%M").is_ok()
}

/// Validate date format (YYYY-MM-DD)
#[allow(dead_code)]
pub fn validate_date_format(date: &str) -> bool {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}
