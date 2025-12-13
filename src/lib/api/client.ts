import { invoke } from '@tauri-apps/api/core';
import type { PrayerTime, PrayerLog, AppSettings } from './schema';
import {
	prayerTimesArraySchema,
	prayerLogsArraySchema,
	appSettingsArraySchema
} from './schema';

export class TauriApiClient {
	/**
	 * Fetch prayer times from API and store in database
	 */
	async fetchAndStorePrayerTimes(
		apiKey: string,
		latitude: number,
		longitude: number,
		date: string
	): Promise<PrayerTime[]> {
		const result = await invoke<PrayerTime[]>('fetch_and_store_prayer_times', {
			apiKey,
			latitude,
			longitude,
			date
		});
		return prayerTimesArraySchema.parse(result);
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
}

// Export singleton instance
export const apiClient = new TauriApiClient();
