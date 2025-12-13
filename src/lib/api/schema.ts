import { z } from 'zod';

// Prayer Time Schema
export const prayerTimeSchema = z.object({
	id: z.number().optional().nullable(),
	name: z.string(),
	time: z.string(),
	date: z.string(),
	created_at: z.string().optional().nullable()
});

export type PrayerTime = z.infer<typeof prayerTimeSchema>;

export const prayerTimesArraySchema = z.array(prayerTimeSchema);

// Prayer Log Schema
export const prayerLogSchema = z.object({
	id: z.number().optional().nullable(),
	prayer_name: z.string(),
	scheduled_time: z.string(),
	executed_at: z.string(),
	action: z.string(),
	success: z.boolean()
});

export type PrayerLog = z.infer<typeof prayerLogSchema>;

export const prayerLogsArraySchema = z.array(prayerLogSchema);

// App Settings Schema
export const appSettingsSchema = z.object({
	id: z.number().optional().nullable(),
	key: z.string(),
	value: z.string(),
	updated_at: z.string().optional().nullable()
});

export type AppSettings = z.infer<typeof appSettingsSchema>;

export const appSettingsArraySchema = z.array(appSettingsSchema);

// API Response Schemas
export const fetchPrayerTimesResponseSchema = prayerTimesArraySchema;

// Settings Keys
export const SETTINGS_KEYS = {
	CITY: 'city',
	COUNTRY: 'country',
	AUTO_MUTE: 'auto_mute',
	NOTIFICATIONS_ENABLED: 'notifications_enabled',
	THEME_MODE: 'theme_mode'
} as const;

// Types for Settings
export interface AppConfig {
	city: string;
	country: string;
	autoMute: boolean;
	notificationsEnabled: boolean;
	themeMode: 'light' | 'dark' | 'system';
}
