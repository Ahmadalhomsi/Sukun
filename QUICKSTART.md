# Sukun - Quick Start Guide

Get up and running with the Sukun Prayer Times Manager in 5 minutes.

## ⚡ Fast Setup

### Step 1: Install Dependencies

```powershell
# Using Bun (recommended)
bun install

# OR using npm
npm install
```

### Step 2: Run Development Server

```powershell
bun run tauri dev
```

The application window will open automatically.

## 🎯 First Time Setup

### 1. Configure Settings

Click **Settings** in the sidebar and fill in:

- **Latitude**: Your location's latitude (e.g., `40.7128`)
- **Longitude**: Your location's longitude (e.g., `-74.0060`)
- **API Key**: (Will be provided later - leave empty for now)

Click **Save Settings**.

### 2. Fetch Prayer Times

In Settings, click **Fetch Prayer Times Now**.

Note: This currently uses placeholder data. Real API integration pending.

### 3. View Home Page

Click **Home** to see:
- Today's prayer schedule
- Next upcoming prayer with countdown
- Quick mute/unmute buttons

### 4. Test Audio Control

Click **Mute Now** button to test system audio muting (Windows only).

## 📱 Using the App

### System Tray

The app minimizes to the system tray when you close the window.

**Tray Menu Options:**
- Open: Restore the window
- Mute Now: Immediately mute system audio
- Quit: Exit the application

### Background Mode

The prayer scheduler runs in the background even when the window is closed. It will:
- Monitor prayer times
- Automatically mute audio at prayer time
- Send notifications
- Log all actions

### View Logs

Click **Logs** to see:
- History of automated actions
- Success/failure status
- Statistics summary

## 🎨 Customize Theme

In Settings:
- Choose Light, Dark, or System theme
- App uses turquoise (#1ABC9C) as primary color

## 🚀 Build for Production

```powershell
bun run tauri build
```

Installer will be in: `src-tauri/target/release/bundle/`

## 🔧 Troubleshooting

### App won't start?
```powershell
# Clean and reinstall
rm -rf node_modules
bun install
cargo clean  # in src-tauri/
```

### Database issues?
Delete: `%APPDATA%\com.ahmad.sukun\sukun.db`

### Audio not working?
- Ensure you're on Windows
- Try running as administrator
- Check Windows audio permissions

## 📚 Learn More

- Full docs: See [README.md](README.md)
- MCP tools: See [MCP_TOOLS.md](MCP_TOOLS.md)
- Tauri docs: https://v2.tauri.app/

## 🎁 Next Steps

1. **Get API Endpoint**: Replace placeholder in `src-tauri/src/api.rs`
2. **Configure Auto-start**: Enable in Windows Task Scheduler
3. **Customize Prayers**: Adjust times in Settings after fetching
4. **Enable Notifications**: Check Windows notification permissions

---

**Quick Commands Reference:**

```powershell
# Development
bun run tauri dev

# Build
bun run tauri build

# Type check
bun run check

# Clean build
cd src-tauri && cargo clean

# Update dependencies
bun update
cargo update  # in src-tauri/
```

Enjoy using Sukun! 🕌
