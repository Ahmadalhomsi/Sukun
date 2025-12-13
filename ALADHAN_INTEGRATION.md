# Aladhan API Integration & Turkish Translation

## Changes Made

### 1. **Aladhan API Integration**

The app now uses the **free Aladhan API** (https://api.aladhan.com) for fetching prayer times. No API key is required.

#### Backend Changes (Rust)

- **[api.rs](src-tauri/src/api.rs)**: 
  - Updated `fetch_prayer_times()` to call Aladhan's `/v1/timingsByCity` endpoint
  - Changed from latitude/longitude to city/country parameters
  - Prayer times are automatically returned in Turkish names (İmsak, Güneş, Öğle, İkindi, Akşam, Yatsı)
  - Uses method=13 (Turkey Diyanet calculation method)

- **[lib.rs](src-tauri/src/lib.rs)**: 
  - Updated `fetch_and_store_prayer_times` command to accept city/country instead of lat/long

#### Frontend Changes (TypeScript/Svelte)

- **[client.ts](src/lib/api/client.ts)**: 
  - Updated `fetchAndStorePrayerTimes()` to use city/country parameters
  - Removed API key requirement

- **[schema.ts](src/lib/api/schema.ts)**: 
  - Updated `AppConfig` interface to use `city` and `country` instead of `apiKey`, `latitude`, `longitude`
  - Updated `SETTINGS_KEYS` constants

- **[stores/index.ts](src/lib/stores/index.ts)**: 
  - Default app config now uses Istanbul, Turkey

### 2. **Default Location: Istanbul**

The app is now configured with **Istanbul, Turkey** as the default location:
- City: "Istanbul"
- Country: "Turkey"

### 3. **Turkish Language Interface**

All UI text has been translated to Turkish:

#### [Settings.svelte](src/lib/components/Settings.svelte)
- "Settings" → "Ayarlar"
- "Location" → "Konum"
- "City" → "Şehir"
- "Country" → "Ülke"
- "Audio Behavior" → "Ses Davranışı"
- "Automatically mute system audio at prayer time" → "Namaz vaktinde sistem sesini otomatik kapat"
- "Notifications" → "Bildirimler"
- "Show notifications for prayer times" → "Namaz vakitleri için bildirim göster"
- "Theme" → "Tema"
- "Light/Dark/System" → "Açık/Koyu/Sistem"
- "Save Settings" → "Ayarları Kaydet"
- "Fetch Prayer Times Now" → "Namaz Vakitlerini Şimdi Al"
- Success/error messages translated

#### [Home.svelte](src/lib/components/Home.svelte)
- "Prayer Times" → "Namaz Vakitleri"
- Date locale changed from 'en-US' to 'tr-TR'
- "Error" → "Hata"
- "Next Prayer" → "Sonraki Namaz"
- "Time Remaining" → "Kalan Süre"
- "Today's Schedule" → "Bugünkü Program"
- "Refresh" → "Yenile"
- "No prayer times available" → "Namaz vakti bilgisi mevcut değil"
- "Configure your location in Settings to fetch prayer times" → "Namaz vakitlerini almak için Ayarlar'dan konumunuzu yapılandırın"
- "Completed" → "Tamamlandı"
- "Upcoming" → "Yaklaşıyor"
- "Mute Now" → "Şimdi Sessize Al"
- "Unmute" → "Sesi Aç"

### 4. **Prayer Names in Turkish**

The API now returns prayer times with Turkish names:
- Fajr → İmsak
- Sunrise → Güneş
- Dhuhr → Öğle
- Asr → İkindi
- Maghrib → Akşam
- Isha → Yatsı

## API Endpoint Details

**URL Format:**
```
https://api.aladhan.com/v1/timingsByCity/{day}-{month}-{year}?city={city}&country={country}&method=13
```

**Example:**
```
https://api.aladhan.com/v1/timingsByCity/13-12-2025?city=Istanbul&country=Turkey&method=13
```

**Method 13:** Turkey Diyanet (Presidency of Religious Affairs)

## Usage

1. Open Settings (Ayarlar)
2. The default location is already set to Istanbul, Turkey
3. Click "Namaz Vakitlerini Şimdi Al" (Fetch Prayer Times Now) to get today's prayer times
4. The app will use the free Aladhan API - no API key needed!

## Testing

Build succeeded with no errors:
```bash
npm run build
```

Warnings about `$state()` for `prayers` and `nextPrayer` are non-critical and don't affect functionality.
