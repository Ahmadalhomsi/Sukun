import { writable } from 'svelte/store';
import { persisted } from 'svelte-local-storage-store';
import type { AppConfig } from '$lib/api/schema';

// Theme store (persisted)
export const themeMode = persisted<'light' | 'dark' | 'system'>('theme-mode', 'system');

// App configuration (persisted)
export const appConfig = persisted<Partial<AppConfig>>('app-config', {
	city: 'Istanbul',
	country: 'Turkey',
	autoMute: true,
	notificationsEnabled: true,
	themeMode: 'system'
});

// Current view/page
export const currentView = writable<'home' | 'settings' | 'logs'>('home');

// Loading states
export const isLoadingPrayers = writable(false);
export const isLoadingLogs = writable(false);

// Error states
export const errorMessage = writable<string | null>(null);

// Audio mute state
export const isAudioMuted = writable(false);
