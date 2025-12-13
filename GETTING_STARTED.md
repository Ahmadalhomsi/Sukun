# 🚀 GETTING STARTED - Read This First!

## Welcome to Sukun Prayer Times Manager

This document will get you started in **5 minutes or less**.

---

## 📋 Prerequisites Check

Before you begin, ensure you have:

- [ ] **Node.js 18+** OR **Bun** installed
- [ ] **Rust 1.70+** installed (via rustup)
- [ ] **Visual Studio Build Tools** (Windows)
- [ ] **Windows OS** (for audio muting features)

### Quick Install Links:
- Node.js: https://nodejs.org/
- Bun: https://bun.sh/
- Rust: https://rustup.rs/
- VS Build Tools: https://visualstudio.microsoft.com/downloads/ (scroll to "Build Tools")

---

## 🎯 Quick Setup (Choose One Method)

### Method 1: Automated Setup (Recommended)

```powershell
# Run the setup script
powershell -ExecutionPolicy Bypass -File setup.ps1
```

This will:
✅ Check for required tools
✅ Install all dependencies
✅ Provide next steps

### Method 2: Manual Setup

```powershell
# Install frontend dependencies
bun install
# or
npm install
```

---

## 🏃 Running the Application

### Development Mode (Hot Reload)

```powershell
bun run tauri dev
# or
npm run tauri dev
```

**First run notes:**
- Rust compilation takes 3-5 minutes initially
- Subsequent runs are much faster
- Development window opens automatically

### Production Build

```powershell
bun run tauri build
# or
npm run tauri build
```

**Output location:**
`src-tauri/target/release/bundle/`

---

## 📖 Documentation Overview

| File | Purpose | When to Read |
|------|---------|--------------|
| **README.md** | Complete documentation | Before starting development |
| **QUICKSTART.md** | 5-minute setup guide | Right now! |
| **PROJECT_SUMMARY.md** | What was built | To understand the project |
| **MCP_TOOLS.md** | Code navigation & tools | When exploring code |
| **API_INTEGRATION_CHECKLIST.md** | API integration guide | When API endpoint arrives |
| **GETTING_STARTED.md** | This file | Right now! |

---

## 🎨 Project Structure at a Glance

```
Sukun/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── api/           # API client
│   │   ├── components/    # UI components
│   │   ├── stores/        # State management
│   │   └── utils/         # Utilities
│   └── routes/
│       └── +page.svelte   # Main app
│
├── src-tauri/             # Rust backend
│   └── src/
│       ├── lib.rs         # App setup
│       ├── api.rs         # Prayer API
│       ├── audio.rs       # Audio control
│       ├── scheduler.rs   # Background tasks
│       └── db.rs          # Database
│
└── Documentation files
```

---

## 🔍 First Steps After Installation

### 1. Launch the App

```powershell
bun run tauri dev
```

### 2. Configure Settings

1. Click **Settings** in the sidebar
2. Enter your location:
   - **Latitude**: Your location's latitude (e.g., 40.7128 for New York)
   - **Longitude**: Your location's longitude (e.g., -74.0060 for New York)
3. (Optional) Enter API Key when available
4. Click **Save Settings**

### 3. Fetch Prayer Times

Still in Settings:
1. Click **Fetch Prayer Times Now**
2. Note: Currently uses mock data (5 prayer times)
3. Real API integration pending

### 4. View Home Page

1. Click **Home** in sidebar
2. You'll see:
   - Today's prayer schedule
   - Next prayer with countdown
   - Quick action buttons

### 5. Test Audio Muting (Windows Only)

1. Click **Mute Now** button
2. Check your system volume (should be muted)
3. Click **Unmute** to restore

### 6. View Activity Logs

1. Click **Logs** in sidebar
2. See history of automated actions
3. Currently empty (will populate when scheduler runs)

---

## 🪟 System Tray Features

The app lives in your system tray:

- **Left click** icon → Opens window
- **Right click** → Menu:
  - Open Sukun
  - Mute Now (instant mute)
  - Quit

**Important:** Closing the window **doesn't quit** the app!
- Window hides to tray
- Background scheduler continues
- Prayer times still trigger

---

## 🔧 Troubleshooting

### "Command not found" errors

**Solution:** Install missing tool
```powershell
# For bun
npm install -g bun

# For rust
# Download from https://rustup.rs/
```

### Compilation errors on first run

**This is normal!**
- First Rust compilation takes time
- Subsequent runs are faster
- Wait for completion

### Audio muting doesn't work

**Checklist:**
- [ ] Running on Windows?
- [ ] Try running as Administrator
- [ ] Check Windows audio permissions
- [ ] Verify audio device is default

### Database errors

**Quick fix:**
```powershell
# Delete database (will reset all data)
Remove-Item "$env:APPDATA\com.ahmad.sukun\sukun.db"
```

### TypeScript/Svelte errors

**Solution:**
```powershell
# Reinstall dependencies
Remove-Item -Recurse node_modules
bun install
```

### Build failures

**Solution:**
```powershell
# Clean and rebuild
cd src-tauri
cargo clean
cd ..
bun run tauri build
```

---

## 📚 Learning Resources

### Tauri
- Docs: https://v2.tauri.app/
- Examples: https://github.com/tauri-apps/tauri/tree/dev/examples

### Svelte 5
- Docs: https://svelte.dev/docs
- Tutorial: https://learn.svelte.dev/

### Skeleton UI
- Docs: https://www.skeleton.dev/
- Components: https://www.skeleton.dev/components

---

## 🎯 What Can I Do Now?

### Explore the Code
```powershell
# View Rust backend
code src-tauri/src/

# View Svelte frontend
code src/lib/
```

### Customize the Theme
1. Edit `tailwind.config.ts` for colors
2. Edit `src/app.css` for custom styles
3. Change primary color from turquoise to your preference

### Test Features
- ✅ Navigation between pages
- ✅ Settings persistence
- ✅ Theme switching (light/dark)
- ✅ Audio mute/unmute
- ✅ System tray interaction

---

## 🔜 Next Steps

### When API Endpoint Arrives

1. **Read:** `API_INTEGRATION_CHECKLIST.md`
2. **Edit:** `src-tauri/src/api.rs`
3. **Replace:** Mock data with real API call
4. **Test:** Thoroughly
5. **Update:** Documentation

### Optional Enhancements

Consider adding:
- [ ] Qibla direction indicator
- [ ] Multiple calculation methods
- [ ] Custom notification sounds
- [ ] Multi-language support
- [ ] Prayer adjustments
- [ ] Hijri calendar

---

## 💡 Tips & Best Practices

### Development
- Use `bun run tauri dev` for hot reload
- Check console for errors
- Use browser DevTools (Ctrl+Shift+I)

### Production
- Always test builds before distributing
- Check installer works on clean system
- Verify auto-start functionality

### Database
- Located at: `%APPDATA%\com.ahmad.sukun\sukun.db`
- Can be backed up
- SQLite browser tools work

---

## 🆘 Need Help?

### Check These First:
1. Error messages in terminal
2. Browser console (F12)
3. Tauri documentation
4. This project's README.md

### Common Issues:
- **Slow first build:** Normal, Rust compilation
- **Hot reload not working:** Restart dev server
- **Window won't open:** Check task manager for background process

---

## ✅ Success Checklist

Before considering setup complete:

- [ ] App launches without errors
- [ ] Can navigate all pages (Home/Settings/Logs)
- [ ] Settings save and persist
- [ ] Theme switching works
- [ ] Can minimize to tray
- [ ] Tray menu works
- [ ] Audio mute/unmute works (Windows)
- [ ] Mock prayer times display

---

## 🎉 You're Ready!

**Everything working?**
You're all set to:
- Explore the codebase
- Customize the theme
- Add features
- Integrate the real API

**Questions?**
- Check README.md for detailed docs
- See MCP_TOOLS.md for code navigation
- Review PROJECT_SUMMARY.md for overview

---

## 📞 Quick Command Reference

```powershell
# Development
bun run tauri dev          # Start dev server
bun run check             # Type check
bun run check:watch       # Watch mode type check

# Production
bun run tauri build       # Build application

# Maintenance
bun update                # Update dependencies
cd src-tauri && cargo update  # Update Rust deps
```

---

**Happy coding! 🚀**

Built with Tauri, Svelte, and Rust ❤️
