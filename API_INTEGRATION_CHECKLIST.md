# API Integration Checklist

Use this checklist when you receive the actual prayer times API endpoint.

## 📋 Pre-Integration Checklist

- [ ] Obtain API endpoint URL
- [ ] Get API key/authentication method
- [ ] Review API documentation
- [ ] Understand response format
- [ ] Check rate limits and quotas
- [ ] Verify required parameters

## 🔧 Backend Integration Steps

### 1. Update API Client (src-tauri/src/api.rs)

- [ ] Open `src-tauri/src/api.rs`
- [ ] Locate `fetch_prayer_times()` function
- [ ] Replace mock implementation with real API call
- [ ] Update URL construction:
  ```rust
  let url = format!(
      "https://your-api-endpoint.com/path?lat={}&lng={}&date={}&key={}",
      latitude, longitude, date, api_key
  );
  ```
- [ ] Adjust HTTP method if needed (GET/POST)
- [ ] Add required headers
- [ ] Update response parsing

### 2. Verify Response Schema

- [ ] Check if API response matches `PrayerTimesResponse` struct
- [ ] Update struct if needed:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct PrayerTimesResponse {
      pub date: String,
      pub prayers: Vec<Prayer>,
      // Add any additional fields
  }
  ```
- [ ] Add error handling for API failures
- [ ] Handle rate limiting errors
- [ ] Add retry logic if needed

### 3. Test Backend Integration

- [ ] Run `cargo test` in src-tauri/ (if tests exist)
- [ ] Test with invalid API key
- [ ] Test with invalid coordinates
- [ ] Test with past dates
- [ ] Test with future dates
- [ ] Test network failure handling

## 🎨 Frontend Integration Steps

### 1. Update TypeScript Schemas (src/lib/api/schema.ts)

- [ ] Review Zod schemas
- [ ] Update if API response format differs
- [ ] Add validation for new fields
- [ ] Update TypeScript types

### 2. Update Settings Component

- [ ] Ensure API key field is visible
- [ ] Add validation for API key format
- [ ] Add location lookup helper (optional)
- [ ] Test "Fetch Prayer Times Now" button

### 3. Update Home Component

- [ ] Verify prayer times display correctly
- [ ] Check time formatting
- [ ] Test countdown timer accuracy
- [ ] Verify next prayer detection

## 🧪 Testing Checklist

### Unit Tests
- [ ] Test API client with mock responses
- [ ] Test time calculations
- [ ] Test database operations
- [ ] Test audio muting

### Integration Tests
- [ ] Test full fetch-store-display flow
- [ ] Test scheduler with real prayer times
- [ ] Test notification system
- [ ] Test tray menu actions

### Manual Tests
- [ ] Fetch prayer times for your location
- [ ] Verify times are accurate
- [ ] Wait for a prayer time (or manually set close time)
- [ ] Verify audio mutes automatically
- [ ] Check notification appears
- [ ] Verify log entry is created
- [ ] Test across multiple days

## 📝 Documentation Updates

- [ ] Update README.md with:
  - [ ] Real API endpoint information
  - [ ] How to obtain API key
  - [ ] API rate limits
  - [ ] Supported regions/countries
  
- [ ] Update MCP_TOOLS.md with:
  - [ ] Actual API structure
  - [ ] Request/response examples
  - [ ] Error codes and handling
  
- [ ] Update QUICKSTART.md if needed

## 🔐 Security Checklist

- [ ] Never commit API keys to git
- [ ] Store API keys securely in database
- [ ] Add API key validation
- [ ] Implement secure key storage
- [ ] Add option to mask API key in UI
- [ ] Consider encryption for sensitive data

## 🚀 Performance Optimization

- [ ] Implement caching for API responses
- [ ] Add request debouncing if needed
- [ ] Optimize database queries
- [ ] Consider background sync schedule
- [ ] Test with slow network conditions

## 📊 Error Handling

### API Errors to Handle:
- [ ] 401 Unauthorized (invalid API key)
- [ ] 403 Forbidden (rate limit exceeded)
- [ ] 404 Not Found (invalid endpoint)
- [ ] 500 Server Error
- [ ] Network timeout
- [ ] Invalid response format

### User-Facing Error Messages:
- [ ] "Invalid API key. Please check your settings."
- [ ] "Unable to connect to prayer times service."
- [ ] "Location not found. Please verify coordinates."
- [ ] "Rate limit exceeded. Please try again later."

## 🔄 Fallback Strategy

If API is unavailable:
- [ ] Implement local prayer time calculation (optional)
- [ ] Cache last successful fetch
- [ ] Show cached data with warning
- [ ] Add manual time entry option

## 📱 Additional Features (Optional)

- [ ] Add prayer calculation method selection
- [ ] Support multiple cities/timezones
- [ ] Add Qibla direction
- [ ] Import/export prayer times
- [ ] Offline mode with calculations

## ✅ Final Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] Error handling implemented
- [ ] Security measures in place
- [ ] Performance acceptable
- [ ] User feedback collected
- [ ] Edge cases handled
- [ ] Code reviewed
- [ ] Ready for production build

## 🐛 Known Issues to Monitor

Track any issues discovered:

```markdown
1. Issue: [Description]
   Status: [Open/In Progress/Resolved]
   Priority: [High/Medium/Low]
   Notes: [Any additional context]

2. Issue: 
   Status: 
   Priority: 
   Notes: 
```

## 📞 Support Contacts

API Provider:
- Support Email: ___________
- Documentation: ___________
- Status Page: ___________

---

## 🎯 Quick Reference

### Current Mock Data Structure:
```json
{
  "date": "2024-01-01",
  "prayers": [
    { "name": "Fajr", "time": "05:30" },
    { "name": "Dhuhr", "time": "12:45" },
    { "name": "Asr", "time": "15:30" },
    { "name": "Maghrib", "time": "18:15" },
    { "name": "Isha", "time": "19:45" }
  ]
}
```

### Files to Modify:
1. `src-tauri/src/api.rs` - Main API implementation
2. `src/lib/api/schema.ts` - TypeScript types
3. `README.md` - Documentation
4. `MCP_TOOLS.md` - MCP documentation

### Testing Command:
```powershell
bun run tauri dev
```

---

**Note**: Check off items as you complete them. This ensures nothing is missed during integration.
