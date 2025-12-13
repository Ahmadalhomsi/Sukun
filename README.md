# Sukun - Prayer Times Manager

A modern desktop application built with Tauri v2, Svelte, and Rust for managing Islamic prayer times with automated system audio control on Windows.

## ✨ Features

- 🕌 **Prayer Time Management**: Fetch and display daily prayer times
- 🔇 **Automatic Audio Muting**: Mutes system audio at prayer times using Windows Core Audio API
- 🎯 **Background Tasks**: Runs scheduler without WebView (continues when window is closed)
- 🪟 **System Tray**: Minimize to tray with quick actions
- 📊 **Activity Logs**: Track all automated prayer actions
- 🎨 **Modern UI**: Turquoise-themed interface with dark mode support
- 💾 **SQLite Database**: Persistent storage for prayer times, logs, and settings
- 🚀 **Auto-start**: Optional auto-start on Windows boot
- 🔔 **Notifications**: Desktop notifications for prayer times

## 🏗 Architecture

### Backend (Rust/Tauri)
- **main.rs/lib.rs**: Application entry point, Tauri setup, tray menu
- **db.rs**: SQLite database operations using sqlx
- **api.rs**: Prayer times API client (placeholder for actual endpoint)
- **scheduler.rs**: Background task scheduler for prayer time automation
- **audio.rs**: Windows Core Audio API integration for system muting

### Frontend (Svelte)
- **Home**: Display today's prayer times and next prayer
- **Settings**: Configure API key, location, preferences
- **Logs**: View history of automated actions
- **Stores**: Svelte stores for state management with localStorage persistence
- **API Client**: Type-safe Tauri command invocations with Zod validation

## 🚀 Prerequisites

- **Node.js** 18+ (or Bun)
- **Rust** 1.70+
- **Windows** (for audio control features)
- **Visual Studio Build Tools** (for Windows Rust compilation)

## 📦 Installation

### 1. Clone and Install Dependencies

```powershell
# Navigate to project directory
cd "E:\Code PRJS\My repos\Sukun"

# Install frontend dependencies
bun install
# or
npm install

# Rust dependencies will be installed automatically
```

### 2. Configuration

Configure your settings in the app's Settings page:

- **API Key**: Will be provided (currently using placeholder)
- **Location**: Set your latitude and longitude
- **Auto Mute**: Enable/disable automatic audio muting
- **Notifications**: Enable/disable prayer time notifications

## 🛠 Development

### Run Development Server

```powershell
# Start Tauri in development mode
bun run tauri dev
# or
npm run tauri dev
```

This will:
1. Start Vite dev server on `http://localhost:1420`
2. Launch Tauri window with hot reload
3. Start background scheduler

### Build for Production

```powershell
# Build the application
bun run tauri build
# or
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## 📁 Project Structure

```
Sukun/
├── src/                          # Frontend source
│   ├── app.css                   # Global styles & theme
│   ├── app.html                  # HTML template
│   ├── routes/
│   │   ├── +page.svelte          # Main app layout
│   │   └── +layout.ts            # SvelteKit layout config
│   └── lib/
│       ├── api/
│       │   ├── client.ts         # Tauri API client
│       │   └── schema.ts         # Zod schemas & types
│       ├── stores/
│       │   └── index.ts          # Svelte stores
│       ├── utils/
│       │   └── time.ts           # Time formatting utilities
│       └── components/
│           ├── Home.svelte       # Home page
│           ├── Settings.svelte   # Settings page
│           └── Logs.svelte       # Logs page
│
├── src-tauri/                    # Backend source
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # Tauri app setup
│   │   ├── api.rs                # Prayer API client
│   │   ├── audio.rs              # Windows audio control
│   │   ├── scheduler.rs          # Background scheduler
│   │   └── db.rs                 # SQLite database
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
│
├── package.json                  # Node dependencies
├── tailwind.config.ts            # Tailwind + theme config
├── tsconfig.json                 # TypeScript config
├── vite.config.js                # Vite config
└── README.md                     # This file
```

## 🎨 Theme Customization

The app uses a turquoise-based color scheme. Modify in:

- **tailwind.config.ts**: Tailwind color palette
- **src/app.css**: CSS variables and custom styles

Primary color: `#1ABC9C` (Turquoise)

## 🗄 Database Schema

### prayer_times
- id, name, time, date, created_at

### prayer_logs
- id, prayer_name, scheduled_time, executed_at, action, success

### settings
- id, key, value, updated_at

Database location: `%APPDATA%/com.ahmad.sukun/sukun.db`

## 🔧 Tauri Commands

### Available Commands

```typescript
// Fetch and store prayer times
fetchAndStorePrayerTimes(apiKey: string, latitude: number, longitude: number, date: string): Promise<PrayerTime[]>

// Get prayer times for specific date
getPrayerTimesForDate(date: string): Promise<PrayerTime[]>

// Get today's prayers
getTodaysPrayers(): Promise<PrayerTime[]>

// Get recent logs
getRecentLogs(limit: number): Promise<PrayerLog[]>

// Settings management
setSetting(key: string, value: string): Promise<void>
getSetting(key: string): Promise<string | null>
getAllSettings(): Promise<AppSettings[]>

// Audio control
muteAudioNow(): Promise<void>
unmuteAudioNow(): Promise<void>
checkAudioMuteStatus(): Promise<boolean>
```

## 🌐 API Integration

The prayer times API is currently using a placeholder. When you receive the actual API endpoint:

1. Open `src-tauri/src/api.rs`
2. Replace the mock implementation in `fetch_prayer_times()` function
3. Update the API call structure to match the provided endpoint
4. Rebuild the application

Example:
```rust
// Replace this section in api.rs
let url = format!(
    "https://api.example.com/prayer-times?lat={}&lng={}&date={}&key={}",
    latitude, longitude, date, api_key
);

let response = client
    .get(&url)
    .send()
    .await?
    .json::<PrayerTimesResponse>()
    .await?;
```

## 🪟 System Tray

The app runs in the system tray with these options:
- **Open**: Show the main window
- **Mute Now**: Immediately mute system audio
- **Quit**: Exit the application

Closing the window minimizes to tray. The scheduler continues running in the background.

## 🔔 Auto-start

To enable auto-start on Windows boot:

1. The app includes the auto-start plugin
2. Enable it in Settings (future enhancement)
3. Or manually via Windows Task Scheduler

## 🧪 Testing

### Test Audio Control
```typescript
// From browser console or Settings page
await apiClient.muteAudioNow();    // Mute
await apiClient.unmuteAudioNow();  // Unmute
const isMuted = await apiClient.checkAudioMuteStatus();
```

### Test Prayer Fetch
1. Configure API key and location in Settings
2. Click "Fetch Prayer Times Now"
3. Check Home page for populated prayer times

## 🐛 Troubleshooting

### Audio Muting Not Working
- Ensure you're running on Windows
- Check Windows permissions for audio control
- Verify COM initialization (requires admin for some systems)

### Database Errors
- Check `%APPDATA%/com.ahmad.sukun/` folder exists
- Verify write permissions
- Delete `sukun.db` to reset (will lose data)

### Build Errors
- Update Rust: `rustup update`
- Clean build: `cargo clean` in `src-tauri/`
- Reinstall dependencies: `rm -rf node_modules && bun install`

## 📝 MCP Tools & Documentation

### MCP Command Examples

```bash
# Describe API (when endpoint is provided)
api.describe

# Show UI component docs
ui.componentDocs

# Explain code sections
code.explain

# Show database schema
db.schemaDocs
```

### Documentation References
- Tauri v2: https://v2.tauri.app/
- Svelte 5: https://svelte.dev/docs
- Skeleton UI: https://www.skeleton.dev/
- Zod: https://zod.dev/
- SQLx: https://github.com/launchbadge/sqlx

## 🛣 Roadmap

- [ ] Implement actual prayer API integration
- [ ] Add prayer time calculation methods as fallback
- [ ] Qibla direction compass
- [ ] Multiple prayer calculation methods
- [ ] Export/import settings
- [ ] Multi-language support
- [ ] Custom notification sounds

## 📄 License

MIT License

## 👤 Author

Built for Sukun Prayer Times Manager

## 🙏 Acknowledgments

- Tauri Team for the amazing framework
- Svelte Team for the reactive UI framework
- Skeleton UI for the component library
- Windows Audio API documentation

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
