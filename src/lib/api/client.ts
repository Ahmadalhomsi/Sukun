import { invoke } from '@tauri-apps/api/core';
import type { PrayerTime, PrayerLog, AppSettings } from './schema';
import {
	prayerTimesArraySchema,
	prayerLogsArraySchema,
	appSettingsArraySchema
} from './schema';
import { countries, type City } from '$lib/data/locations';
import { calculatePrayerTimes, convertToDbFormat } from '$lib/services/prayerCalculator';

export class TauriApiClient {
	/**
	 * Calculate and store prayer times locally using adhan-js
	 */
	async calculateAndStorePrayerTimes(
		cityName: string,
		countryName: string,
		date: string
	): Promise<PrayerTime[]> {
		// Find the city coordinates
		const country = countries.find(c => c.name === countryName);
		if (!country) {
			throw new Error(`Country not found: ${countryName}`);
		}

		const city = country.cities.find(c => c.name === cityName);
		if (!city) {
			throw new Error(`City not found: ${cityName}`);
		}

		return this.calculateAndStorePrayerTimesFromCoordinates(
			city.latitude,
			city.longitude,
			date
		);
	}

	/**
	 * Calculate and store prayer times from coordinates
	 */
	async calculateAndStorePrayerTimesFromCoordinates(
		latitude: number,
		longitude: number,
		date: string
	): Promise<PrayerTime[]> {
		// Clear existing prayers for this date first
		await this.clearAllPrayerTimes();

		// Calculate prayer times using adhan-js with coordinates
		const dateObj = new Date(date);
		const city = { name: 'Current Location', latitude, longitude };
		const calculatedTimes = calculatePrayerTimes(city, dateObj);
		
		// Convert to database format
		const prayerData = convertToDbFormat(calculatedTimes, date);

		// Store each prayer time in the database
		const storedPrayers: PrayerTime[] = [];
		for (const prayer of prayerData) {
			const stored = await this.storeSinglePrayerTime(prayer.name, prayer.time, prayer.date);
			storedPrayers.push(stored);
		}

		return storedPrayers;
	}

	/**
	 * Store a single prayer time (helper method)
	 */
	private async storeSinglePrayerTime(
		name: string,
		time: string,
		date: string
	): Promise<PrayerTime> {
		const result = await invoke<PrayerTime>('store_prayer_time', {
			name,
			time,
			date
		});
		return result;
	}

	/**
	 * Fetch prayer times from Aladhan API and store in database (deprecated - kept for compatibility)
	 */
	async fetchAndStorePrayerTimes(
		city: string,
		country: string,
		date: string
	): Promise<PrayerTime[]> {
		// Use local calculation instead
		return this.calculateAndStorePrayerTimes(city, country, date);
	}

	/**
	 * Get prayer times for a specific date
	 */
	async getPrayerTimesForDate(date: string): Promise<PrayerTime[]> {
		const result = await invoke<PrayerTime[]>('get_prayer_times_for_date', { date });
		return prayerTimesArraySchema.parse(result);
	}

	/**
	 * Get today's prayer times
	 */
	async getTodaysPrayers(): Promise<PrayerTime[]> {
		const result = await invoke<PrayerTime[]>('get_todays_prayers');
		return prayerTimesArraySchema.parse(result);
	}

	/**
	 * Get recent prayer logs
	 */
	async getRecentLogs(limit: number = 50): Promise<PrayerLog[]> {
		const result = await invoke<PrayerLog[]>('get_recent_logs', { limit });
		return prayerLogsArraySchema.parse(result);
	}

	/**
	 * Clear all prayer logs
	 */
	async clearLogs(): Promise<void> {
		await invoke('clear_logs');
	}

	/**
	 * Set a setting value
	 */
	async setSetting(key: string, value: string): Promise<void> {
		await invoke('set_setting', { key, value });
	}

	/**
	 * Get a setting value
	 */
	async getSetting(key: string): Promise<string | null> {
		const result = await invoke<string | null>('get_setting', { key });
		return result;
	}

	/**
	 * Get all settings
	 */
	async getAllSettings(): Promise<AppSettings[]> {
		const result = await invoke<AppSettings[]>('get_all_settings');
		return appSettingsArraySchema.parse(result);
	}

	/**
	 * Mute system audio immediately
	 */
	async muteAudioNow(): Promise<void> {
		await invoke('mute_audio_now');
	}

	/**
	 * Unmute system audio
	 */
	async unmuteAudioNow(): Promise<void> {
		await invoke('unmute_audio_now');
	}

	/**
	 * Check if audio is currently muted
	 */
	async checkAudioMuteStatus(): Promise<boolean> {
		const result = await invoke<boolean>('check_audio_mute_status');
		return result;
	}

	/**
	 * Test Windows notifications
	 */
	async testNotification(): Promise<string> {
		const result = await invoke<string>('test_notification');
		return result;
	}

	/**
	 * Enable auto-start on system startup
	 */
	async enableAutoStart(): Promise<boolean> {
		const result = await invoke<boolean>('enable_auto_start');
		return result;
	}

	/**
	 * Disable auto-start on system startup
	 */
	async disableAutoStart(): Promise<boolean> {
		const result = await invoke<boolean>('disable_auto_start');
		return result;
	}

	/**
	 * Check if auto-start is enabled
	 */
	async isAutoStartEnabled(): Promise<boolean> {
		const result = await invoke<boolean>('is_auto_start_enabled');
		return result;
	}

	/**
	 * Clear all prayer times from database (for debugging)
	 */
	async clearAllPrayerTimes(): Promise<void> {
		await invoke('clear_all_prayer_times');
	}
}

// Export singleton instance
export const apiClient = new TauriApiClient();
