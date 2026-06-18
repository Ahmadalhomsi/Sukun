# Microsoft Store Listing - Sukun

## App Identity

| Field | Value |
|-------|-------|
| App Name | Sukun |
| Developer | AlhomsiDev |
| App ID | AlhomsiDev.Sukun |
| Category | Lifestyle > Religious |
| Pricing | Free |

---

## English (en-US)

**Short Description:**
Prayer times manager with automatic audio mute so you never miss a prayer.

**Description:**
Sukun is a lightweight prayer times manager for Windows that runs silently in the background from the moment you start your PC. It automatically mutes system audio at prayer times so you can hear the adhan clearly, and sends you a notification before each prayer so you're always prepared.

Features: accurate prayer times via Aladhan API, auto-start on boot, background system tray operation, automatic audio muting, adhan sound playback, pre-prayer notifications with configurable alerts, setup wizard, and activity logs. Supports English, Turkish, and Arabic. Works offline after initial setup.

**Product Features:**
Prayer Times, Auto Mute, Adhan Sound, Pre-Prayer Notifications, Auto-Start, Background Mode, System Tray, Lightweight, Offline, Setup Wizard, Activity Logs, Multi-Language

**Keywords:**
prayer times, adhan, azan, islam, muslim, namaz, prayer, fasting, qibla, athan

---

## Arabic (ar-SA)

**Short Description:**
مدير أوقات الصلاة مع كتم الصوت التلقائي حتى لا تفوتك أي صلاة.

**Description:**
سكين هو تطبيق خفيف لأوقات الصلاة على ويندوس يعمل بهدوء في الخلفية من لحظة تشغيل جهازك. يكتم صوت الجهاز تلقائيًا عند موعد الصلاة حتى تسمع الأذان بوضوح، ويرسل لك إشعارًا قبل كل صلاة لتكون مستعدًا دائمًا.

الميزات: أوقات صلاة دقيقة من Aladhan API، بدء تلقائي مع تشغيل الويندوس، عمل في خلفية صينية النظام، كتم صوت تلقائي، تشغيل صوت الأذان، إشعار قبل الصلاة قابل للتعديل، معيار إعداد، وسجلات نشاط. يدعم الإنجليزية والتركية والعربية. يعمل دون اتصال.

**Product Features:**
أوقات الصلاة, كتم الصوت, صوت الأذان, إشعار قبل الصلاة, بدء تلقائي, وضع الخلفية, صينية النظام, خفيف, بدون إنترنت, معالج الإعداد, سجلات النشاط, تعدد اللغات

**Keywords:**
أوقات الصلاة, أذان, صلاة, إسلام, مسلم, تقويم, fasting, دين, عبادة, سكين

---

## Turkish (tr-TR)

**Short Description:**
Namaz vakitleri yöneticisi, otomatik ses susturma ile hiçbir namazı kaçırmazsınız.

**Description:**
Sukun, bilgisayarınız açıldığından itibaren arka planda sessizce çalışan hafif bir Windows namaz vakitleri yöneticisidir. Namaz vakitlerinde sistem sesini otomatik susturarak ezanı net duymanızı sağlar ve her namazdan önce bildirim göndererek her zaman hazır olmanızı sağlar.

Özellikler: Aladhan API ile doğru namaz vakitleri, otomatik başlatma, arka plan sistem tepsisi çalışması, otomatik ses susturma, ezan sesli çalma, ayarlanabilir uyarılarla namaz öncesi bildirimler, kurulum sihirbazı ve aktivite kayıtları. İngilizce, Türkçe ve Arapça destekler. Çevrimdışı çalışır.

**Product Features:**
Namaz Vakitleri, Otomatik Susturma, Ezan Sesi, Namaz Öncesi Bildirim, Otomatik Başlatma, Arka Plan Modu, Sistem Tepsisi, Hafif, Çevrimdışı, Kurulum Sihirbazı, Aktivite Kayıtları, Çoklu Dil

**Keywords:**
namaz vakitleri, ezan, islam, muslim, namaz, oruç, prayer, dini, ibadet, sukun

---

## Release Notes (v1.0.0 - All Languages)

Initial release. Accurate prayer times, auto audio muting, adhan playback, notifications, system tray mode, auto-start, setup wizard, activity logs, EN/TR/AR support.

---

## Privacy Policy

**PRIVACY POLICY — Sukun**
**Last Updated:** June 14, 2026

Sukun is a local desktop application that helps you manage prayer times. We respect your privacy.

**What We Collect:**
- City and country name you enter — used only to fetch prayer times from the Aladhan API.

**What We Do NOT Collect:**
- No personal data (name, email, GPS coordinates)
- No usage analytics or crash reports
- No tracking, cookies, or advertising

**How Data Is Used:**
- Your city/country is sent to the Aladhan API to get prayer times
- All data is stored locally on your device (SQLite database)
- No data is sent to any server we operate

**Third Parties:**
- We use the Aladhan API (api.aladhan.com) to fetch prayer times. Only city and country names are sent.

**Children:**
- We do not collect data from children under 13.

**Changes:**
- We may update this policy. Check the "Last Updated" date for changes.

**Contact:**
- GitHub: https://github.com/AlhomsiDev/Sukun
- Email: [your email]

By using Sukun, you agree to this policy.

---

## Microsoft Store — runFullTrust Justification

**Question:** Why do you need the `runFullTrust` capability, and how will it be used in your product?

**Answer:**
Sukun requires full trust to: mute/unmute system audio at prayer times via Windows Core Audio API, play adhan sound bypassing system mute, run in the background via system tray, and auto-start on boot. These are all local operations with no elevated permissions or internet access beyond fetching prayer times.
