import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';

export type Language = 'en' | 'tr';

export const currentLanguage: Writable<Language> = writable('tr'); // Default to Turkish

const translations = {
	en: {
		// App Title
		appTitle: 'Sukun',
		appSubtitle: 'Prayer Times',
		
		// Navigation
		navHome: 'Home',
		navSettings: 'Settings',
		navLogs: 'Logs',
		
		// Home Page
		prayerTimes: 'Prayer Times',
		nextPrayer: 'Next Prayer',
		timeRemaining: 'Time Remaining',
		todaySchedule: "Today's Schedule",
		refresh: 'Refresh',
		noPrayerTimes: 'No prayer times available',
		configureLocation: 'Configure your location in Settings to fetch prayer times',
		completed: 'Completed',
		upcoming: 'Upcoming',
		muteNow: 'Mute Now',
		unmute: 'Unmute',
		quickActions: 'Quick Actions',
		error: 'Error',
		fetchingPrayerTimes: 'Fetching prayer times...',
		autoFetchFailed: 'Failed to fetch prayer times. Click "Fetch Prayer Times Now" in Settings.',
		pleaseConfigure: 'Please configure your location in Settings to load prayer times.',
		
		// Settings Page
		settings: 'Settings',
		location: 'Location',
		city: 'City',
		country: 'Country',
		selectLocation: 'Select Location',
		timeAdjustment: 'Time Adjustment (minutes)',
		timeAdjustmentHelp: 'Adjust prayer times by ± minutes. Use negative values to make times earlier.',
		usingFreeApi: 'Using free Aladhan API - no API key required',
		audioBehavior: 'Audio Behavior',
		autoMuteAudio: 'Automatically mute system audio at prayer time',
		autoUnmuteAfter: 'Auto-unmute after (minutes)',
		autoUnmuteHelp: 'System will automatically unmute after this duration',
		notifications: 'Notifications',
		showNotifications: 'Show notifications for prayer times',
		theme: 'Theme',
		themeLight: 'Light',
		themeDark: 'Dark',
		themeSystem: 'System',
		language: 'Language',
		saveSettings: 'Save Settings',
		saving: 'Saving...',
		fetchPrayerTimes: 'Fetch Prayer Times Now',
		settingsSaved: 'Settings saved successfully!',
		failedToSave: 'Failed to save settings',
		prayerTimesFetched: 'Prayer times fetched successfully!',
		failedToFetch: 'Failed to fetch prayer times',
		enterCityCountry: 'Please enter city and country first',
		
		// Prayer Names (in English)
		fajr: 'Fajr',
		sunrise: 'Sunrise',
		dhuhr: 'Dhuhr',
		asr: 'Asr',
		maghrib: 'Maghrib',
		isha: 'Isha',
		
		// Logs Page
		prayerLogs: 'Prayer Logs',
		historyOfActions: 'History of automated prayer actions',
		noLogs: 'No logs available',
		prayer: 'Prayer',
		scheduledTime: 'Scheduled Time',
		executedAt: 'Executed At',
		action: 'Action',
		status: 'Status',
		success: 'Success',
		failed: 'Failed'
	},
	tr: {
		// Uygulama Başlığı
		appTitle: 'Sukun',
		appSubtitle: 'Namaz Vakitleri',
		
		// Navigasyon
		navHome: 'Ana Sayfa',
		navSettings: 'Ayarlar',
		navLogs: 'Kayıtlar',
		
		// Ana Sayfa
		prayerTimes: 'Namaz Vakitleri',
		nextPrayer: 'Sonraki Namaz',
		timeRemaining: 'Kalan Süre',
		todaySchedule: 'Bugünkü Program',
		refresh: 'Yenile',
		noPrayerTimes: 'Namaz vakti bilgisi mevcut değil',
		configureLocation: 'Namaz vakitlerini almak için Ayarlar\'dan konumunuzu yapılandırın',
		completed: 'Tamamlandı',
		upcoming: 'Yaklaşıyor',
		muteNow: 'Şimdi Sessize Al',
		unmute: 'Sesi Aç',
		quickActions: 'Hızlı İşlemler',
		error: 'Hata',
		fetchingPrayerTimes: 'Namaz vakitleri alınıyor...',
		autoFetchFailed: 'Namaz vakitleri alınamadı. Ayarlar\'dan "Namaz Vakitlerini Şimdi Al" düğmesine tıklayın.',
		pleaseConfigure: 'Namaz vakitlerini yüklemek için lütfen Ayarlar\'dan konumunuzu yapılandırın.',
		
		// Ayarlar Sayfası
		settings: 'Ayarlar',
		location: 'Konum',
		city: 'Şehir',
		country: 'Ülke',
		selectLocation: 'Konum Seç',
		timeAdjustment: 'Zaman Ayarlaması (dakika)',
		timeAdjustmentHelp: 'Namaz vakitlerini ± dakika ayarlayın. Vakitleri öne almak için negatif değerler kullanın.',
		usingFreeApi: 'Aladhan API kullanılıyor - API anahtarı gerekmez',
		audioBehavior: 'Ses Davranışı',
		autoMuteAudio: 'Namaz vaktinde sistem sesini otomatik kapat',
		autoUnmuteAfter: 'Otomatik sesi aç (dakika)',
		autoUnmuteHelp: 'Sistem bu süre sonunda otomatik olarak sesi açacak',
		notifications: 'Bildirimler',
		showNotifications: 'Namaz vakitleri için bildirim göster',
		theme: 'Tema',
		themeLight: 'Açık',
		themeDark: 'Koyu',
		themeSystem: 'Sistem',
		language: 'Dil',
		saveSettings: 'Ayarları Kaydet',
		saving: 'Kaydediliyor...',
		fetchPrayerTimes: 'Namaz Vakitlerini Şimdi Al',
		settingsSaved: 'Ayarlar başarıyla kaydedildi!',
		failedToSave: 'Ayarlar kaydedilemedi',
		prayerTimesFetched: 'Namaz vakitleri başarıyla alındı!',
		failedToFetch: 'Namaz vakitleri alınamadı',
		enterCityCountry: 'Lütfen önce şehir ve ülke bilgilerini girin',
		
		// Namaz İsimleri (Türkçe)
		fajr: 'İmsak',
		sunrise: 'Güneş',
		dhuhr: 'Öğle',
		asr: 'İkindi',
		maghrib: 'Akşam',
		isha: 'Yatsı',
		
		// Kayıtlar Sayfası
		prayerLogs: 'Namaz Kayıtları',
		historyOfActions: 'Otomatik namaz işlemlerinin geçmişi',
		noLogs: 'Kayıt yok',
		prayer: 'Namaz',
		scheduledTime: 'Planlanan Saat',
		executedAt: 'Gerçekleşme Zamanı',
		action: 'İşlem',
		status: 'Durum',
		success: 'Başarılı',
		failed: 'Başarısız'
	}
};

export const t = derived(currentLanguage, ($lang) => translations[$lang]);

export function translatePrayerName(englishName: string, lang: Language): string {
	const prayerMap: Record<string, keyof typeof translations.en> = {
		'Fajr': 'fajr',
		'Sunrise': 'sunrise',
		'Dhuhr': 'dhuhr',
		'Asr': 'asr',
		'Maghrib': 'maghrib',
		'Isha': 'isha'
	};
	
	const key = prayerMap[englishName];
	if (key) {
		return translations[lang][key];
	}
	return englishName;
}

export function setLanguage(lang: Language) {
	currentLanguage.set(lang);
	localStorage.setItem('language', lang);
}

// Initialize language from localStorage
if (typeof window !== 'undefined') {
	const stored = localStorage.getItem('language') as Language;
	if (stored && (stored === 'en' || stored === 'tr')) {
		currentLanguage.set(stored);
	}
}
