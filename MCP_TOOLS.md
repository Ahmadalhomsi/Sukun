# Sukun MCP Tools Documentation

This document describes the MCP (Model Context Protocol) tools and helper commands for the Sukun Prayer Times Manager application.

## 📖 Overview

These tools help you understand, navigate, and work with the Sukun codebase more effectively.

## 🛠 Available MCP Tools

### 1. api.describe
**Description**: Summarize API endpoints and data structures

**Usage**:
```bash
api.describe
```

**Output**:
- Prayer Times API structure (placeholder)
- Request/Response formats
- Required parameters
- Expected data types

**Example Output**:
```
Prayer Times API Endpoint (Placeholder):
- URL: TBD (will be provided)
- Method: GET
- Parameters:
  - api_key: string (required)
  - latitude: number (required)
  - longitude: number (required)
  - date: string (YYYY-MM-DD format)
- Response Format:
  {
    date: string,
    prayers: [
      { name: string, time: string }
    ]
  }
```

### 2. ui.componentDocs
**Description**: Pull UI component examples and documentation

**Usage**:
```bash
ui.componentDocs [component_name]
```

**Available Components**:
- Home.svelte
- Settings.svelte
- Logs.svelte

**Example**:
```bash
ui.componentDocs Home
```

**Output**:
- Component structure
- Props/State
- Event handlers
- Styling approach
- Integration points

### 3. code.explain
**Description**: Explain parts of generated code

**Usage**:
```bash
code.explain [file_path]
```

**Examples**:
```bash
code.explain src-tauri/src/audio.rs
code.explain src-tauri/src/scheduler.rs
code.explain src/lib/api/client.ts
```

**Output**:
- Function purposes
- Data flow
- Dependencies
- Key algorithms

### 4. db.schemaDocs
**Description**: Show database schema and relationships

**Usage**:
```bash
db.schemaDocs
```

**Output**:
```
Database: sukun.db
Location: %APPDATA%/com.ahmad.sukun/sukun.db

Tables:

1. prayer_times
   - id: INTEGER PRIMARY KEY AUTOINCREMENT
   - name: TEXT NOT NULL (e.g., "Fajr", "Dhuhr")
   - time: TEXT NOT NULL (HH:MM format)
   - date: TEXT NOT NULL (YYYY-MM-DD)
   - created_at: TEXT DEFAULT CURRENT_TIMESTAMP

2. prayer_logs
   - id: INTEGER PRIMARY KEY AUTOINCREMENT
   - prayer_name: TEXT NOT NULL
   - scheduled_time: TEXT NOT NULL
   - executed_at: TEXT DEFAULT CURRENT_TIMESTAMP
   - action: TEXT NOT NULL (e.g., "mute_audio")
   - success: INTEGER NOT NULL (0 or 1)

3. settings
   - id: INTEGER PRIMARY KEY AUTOINCREMENT
   - key: TEXT UNIQUE NOT NULL
   - value: TEXT NOT NULL
   - updated_at: TEXT DEFAULT CURRENT_TIMESTAMP

Common Queries:
- Get today's prayers: SELECT * FROM prayer_times WHERE date = ?
- Recent logs: SELECT * FROM prayer_logs ORDER BY executed_at DESC LIMIT ?
- Get setting: SELECT value FROM settings WHERE key = ?
```

## 🧩 Component Documentation

### Home Component

**Location**: `src/lib/components/Home.svelte`

**Purpose**: Display today's prayer times and next upcoming prayer

**Features**:
- Real-time countdown to next prayer
- Visual distinction between completed and upcoming prayers
- Quick mute/unmute actions
- Auto-refresh capability

**Key Functions**:
```typescript
loadPrayers(): Promise<void>
refreshPrayers(): Promise<void>
formatTime(time: string): string
getTimeUntil(time: string): string
```

### Settings Component

**Location**: `src/lib/components/Settings.svelte`

**Purpose**: Configure app settings and preferences

**Settings Available**:
- API Key configuration
- Location (latitude/longitude)
- Auto-mute toggle
- Notifications toggle
- Theme selection (light/dark/system)

**Key Functions**:
```typescript
saveSettings(): Promise<void>
fetchPrayersNow(): Promise<void>
applyTheme(): void
```

### Logs Component

**Location**: `src/lib/components/Logs.svelte`

**Purpose**: Display prayer action history

**Features**:
- Tabular view of all logs
- Success/failure indicators
- Statistics summary
- Pagination support

**Key Functions**:
```typescript
loadLogs(): Promise<void>
formatDateTime(dateTime: string): string
```

## 🔧 Rust Backend Documentation

### Audio Module (audio.rs)

**Purpose**: Control Windows system audio

**Key Functions**:
```rust
mute_system_audio() -> Result<()>
unmute_system_audio() -> Result<()>
set_system_volume(level: f32) -> Result<()>
is_muted() -> Result<bool>
```

**Implementation**:
- Uses Windows Core Audio API
- COM initialization required
- Platform-specific (Windows only)

### Scheduler Module (scheduler.rs)

**Purpose**: Background task scheduling for prayer times

**Key Structure**:
```rust
pub struct PrayerScheduler {
    db: Arc<Database>,
    app_handle: AppHandle,
}
```

**Key Functions**:
```rust
start() -> Result<()>                    // Main scheduler loop
check_and_execute_prayers() -> Result<()> // Check current time
execute_prayer_action() -> Result<()>     // Perform prayer action
get_next_prayer() -> Option<PrayerTime>  // Find next prayer
```

**Behavior**:
- Checks every minute
- Runs in background tokio task
- Sends notifications
- Logs all actions

### Database Module (db.rs)

**Purpose**: SQLite database operations

**Key Structure**:
```rust
pub struct Database {
    pool: Pool<Sqlite>,
}
```

**Prayer Times Operations**:
```rust
insert_prayer_times(prayers: Vec<PrayerTime>) -> Result<()>
get_prayer_times_for_date(date: &str) -> Result<Vec<PrayerTime>>
clear_old_prayer_times(before_date: &str) -> Result<()>
```

**Logs Operations**:
```rust
insert_prayer_log(log: PrayerLog) -> Result<()>
get_recent_logs(limit: i64) -> Result<Vec<PrayerLog>>
```

**Settings Operations**:
```rust
set_setting(key: &str, value: &str) -> Result<()>
get_setting(key: &str) -> Result<Option<String>>
get_all_settings() -> Result<Vec<AppSettings>>
```

### API Module (api.rs)

**Purpose**: Prayer times API client (placeholder)

**Key Function**:
```rust
fetch_prayer_times(
    api_key: &str,
    latitude: f64,
    longitude: f64,
    date: &str
) -> Result<PrayerTimesResponse>
```

**Current Status**: Returns mock data. Replace with actual API call.

## 📚 Frontend API Client

**Location**: `src/lib/api/client.ts`

**Class**: `TauriApiClient`

**Methods**:
```typescript
// Prayer operations
fetchAndStorePrayerTimes(apiKey, lat, lng, date): Promise<PrayerTime[]>
getPrayerTimesForDate(date: string): Promise<PrayerTime[]>
getTodaysPrayers(): Promise<PrayerTime[]>

// Logs operations
getRecentLogs(limit: number): Promise<PrayerLog[]>

// Settings operations
setSetting(key: string, value: string): Promise<void>
getSetting(key: string): Promise<string | null>
getAllSettings(): Promise<AppSettings[]>

// Audio operations
muteAudioNow(): Promise<void>
unmuteAudioNow(): Promise<void>
checkAudioMuteStatus(): Promise<boolean>
```

**Usage Example**:
```typescript
import { apiClient } from '$lib/api/client';

// Fetch prayers
const prayers = await apiClient.getTodaysPrayers();

// Mute audio
await apiClient.muteAudioNow();

// Save setting
await apiClient.setSetting('api_key', 'your-key-here');
```

## 🎨 Theme System

**Configuration**: `tailwind.config.ts` + `src/app.css`

**Primary Colors**:
```css
--color-primary: #1ABC9C (Turquoise)
--color-primary-light: #48C9B0
--color-primary-dark: #16A085
--color-primary-darker: #0E7A64
```

**Tailwind Classes**:
```css
primary-50 through primary-900
turquoise, turquoise-light, turquoise-dark
```

**Dark Mode**:
Controlled via `themeMode` store. Applies `dark` class to `<html>` element.

## 🔐 Data Flow

### Prayer Times Fetch Flow:
1. User clicks "Fetch Prayer Times Now" in Settings
2. Frontend calls `apiClient.fetchAndStorePrayerTimes()`
3. Tauri command invokes Rust `fetch_and_store_prayer_times()`
4. Rust calls `api::fetch_prayer_times()` (currently mock)
5. Results stored in SQLite via `db.insert_prayer_times()`
6. Frontend receives data and updates UI

### Prayer Time Automation Flow:
1. Scheduler runs in background (tokio task)
2. Checks current time every minute
3. Compares with stored prayer times from DB
4. On match: executes prayer action (mute audio)
5. Sends notification
6. Logs action to database
7. Emits event to frontend

### Settings Management:
1. User modifies settings in Settings page
2. Frontend updates local store + localStorage
3. Calls `apiClient.setSetting()` for persistence
4. Rust stores in SQLite settings table
5. Settings retrieved on app start

## 🔍 Code Navigation Tips

### Finding Specific Functionality:

**Audio Control**:
- Rust: `src-tauri/src/audio.rs`
- Frontend calls: Search for `muteAudioNow` in components

**Database Queries**:
- All SQL: `src-tauri/src/db.rs`
- Schema init: `init_schema()` method

**Prayer Scheduling**:
- Main logic: `src-tauri/src/scheduler.rs`
- Background task: `lib.rs` in `setup()` function

**UI Theming**:
- Tailwind config: `tailwind.config.ts`
- CSS variables: `src/app.css`
- Theme toggle: Settings.svelte

**Tray Menu**:
- Setup: `src-tauri/src/lib.rs` in `setup_system_tray()`
- Menu items: "show", "mute", "quit"

## 🚀 Quick Start Commands

```powershell
# Install dependencies
bun install

# Development mode
bun run tauri dev

# Build production
bun run tauri build

# Type check
bun run check

# Run Svelte checks
bun run check:watch
```

## 📦 Dependencies

### Frontend:
- @tauri-apps/api: Tauri JavaScript bindings
- @tanstack/svelte-query: Data fetching (for future use)
- zod: Schema validation
- svelte-local-storage-store: Persistent stores
- @skeletonlabs/skeleton: UI components
- tailwindcss: Styling

### Backend:
- tauri: Desktop app framework
- tokio: Async runtime
- sqlx: Database operations
- reqwest: HTTP client
- chrono: Date/time handling
- windows: Windows API bindings
- serde: Serialization

## 🎯 Next Steps

When API endpoint is provided:

1. Update `src-tauri/src/api.rs`
2. Replace mock data with real API call
3. Adjust response schema if needed
4. Update Zod schemas in `src/lib/api/schema.ts`
5. Test with real data
6. Update README with actual API documentation

---

**Note**: This is a living document. Update as features are added or modified.
