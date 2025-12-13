<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import { appConfig, themeMode } from '$lib/stores';
	import { getTodayDate } from '$lib/utils/time';

	let apiKey = '';
	let latitude = 0;
	let longitude = 0;
	let autoMute = true;
	let notificationsEnabled = true;
	let selectedTheme: 'light' | 'dark' | 'system' = 'system';

	let isSaving = false;
	let saveMessage = '';

	// Load settings on mount
	onMount(async () => {
		try {
			const settings = await apiClient.getAllSettings();
			settings.forEach((setting: any) => {
				switch (setting.key) {
					case 'api_key':
						apiKey = setting.value;
						break;
					case 'latitude':
						latitude = parseFloat(setting.value);
						break;
					case 'longitude':
						longitude = parseFloat(setting.value);
						break;
					case 'auto_mute':
						autoMute = setting.value === 'true';
						break;
					case 'notifications_enabled':
						notificationsEnabled = setting.value === 'true';
						break;
					case 'theme_mode':
						selectedTheme = setting.value as 'light' | 'dark' | 'system';
						break;
				}
			});
		} catch (error) {
			console.error('Failed to load settings:', error);
		}
	});

	async function saveSettings() {
		try {
			isSaving = true;
			saveMessage = '';

			// Save to backend
			await apiClient.setSetting('api_key', apiKey);
			await apiClient.setSetting('latitude', latitude.toString());
			await apiClient.setSetting('longitude', longitude.toString());
			await apiClient.setSetting('auto_mute', autoMute.toString());
			await apiClient.setSetting('notifications_enabled', notificationsEnabled.toString());
			await apiClient.setSetting('theme_mode', selectedTheme);

			// Update local stores
			$appConfig = {
				apiKey,
				latitude,
				longitude,
				autoMute,
				notificationsEnabled,
				themeMode: selectedTheme
			};

			$themeMode = selectedTheme;

			saveMessage = 'Settings saved successfully!';
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			saveMessage = 'Failed to save settings';
			console.error('Error saving settings:', error);
		} finally {
			isSaving = false;
		}
	}

	async function fetchPrayersNow() {
		try {
			isSaving = true;
			saveMessage = '';

			if (!apiKey || !latitude || !longitude) {
				saveMessage = 'Please fill in API key and location first';
				return;
			}

			const today = getTodayDate();
			await apiClient.fetchAndStorePrayerTimes(apiKey, latitude, longitude, today);
			
			saveMessage = 'Prayer times fetched successfully!';
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			saveMessage = 'Failed to fetch prayer times';
			console.error('Error fetching prayers:', error);
		} finally {
			isSaving = false;
		}
	}

	function applyTheme() {
		if (selectedTheme === 'dark' || (selectedTheme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}

	$: {
		selectedTheme;
		applyTheme();
	}
</script>

<div class="container mx-auto px-4 py-8 max-w-3xl">
	<h1 class="text-4xl font-bold text-gray-800 dark:text-white mb-8">Settings</h1>

	{#if saveMessage}
		<div
			class="mb-6 px-4 py-3 rounded-lg"
			class:bg-green-100={saveMessage.includes('success')}
			class:text-green-700={saveMessage.includes('success')}
			class:dark:bg-green-900={saveMessage.includes('success')}
			class:dark:text-green-200={saveMessage.includes('success')}
			class:bg-red-100={!saveMessage.includes('success')}
			class:text-red-700={!saveMessage.includes('success')}
			class:dark:bg-red-900={!saveMessage.includes('success')}
			class:dark:text-red-200={!saveMessage.includes('success')}
		>
			{saveMessage}
		</div>
	{/if}

	<div class="space-y-6">
		<!-- API Configuration -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				API Configuration
			</h2>
			<div class="space-y-4">
				<div>
					<label for="apiKey" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						API Key
					</label>
					<input
						id="apiKey"
						type="password"
						bind:value={apiKey}
						placeholder="Enter your prayer times API key"
						class="input-field"
					/>
					<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
						API key will be provided - currently using placeholder
					</p>
				</div>
			</div>
		</div>

		<!-- Location Settings -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Location
			</h2>
			<div class="grid grid-cols-2 gap-4">
				<div>
					<label for="latitude" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						Latitude
					</label>
					<input
						id="latitude"
						type="number"
						step="0.000001"
						bind:value={latitude}
						placeholder="e.g., 40.7128"
						class="input-field"
					/>
				</div>
				<div>
					<label for="longitude" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						Longitude
					</label>
					<input
						id="longitude"
						type="number"
						step="0.000001"
						bind:value={longitude}
						placeholder="e.g., -74.0060"
						class="input-field"
					/>
				</div>
			</div>
		</div>

		<!-- Audio Behavior -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Audio Behavior
			</h2>
			<div class="space-y-4">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={autoMute}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						Automatically mute system audio at prayer time
					</span>
				</label>
			</div>
		</div>

		<!-- Notifications -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Notifications
			</h2>
			<div class="space-y-4">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={notificationsEnabled}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						Show notifications for prayer times
					</span>
				</label>
			</div>
		</div>

		<!-- Theme -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Theme
			</h2>
			<div class="space-y-2">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="light"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">Light</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="dark"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">Dark</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="system"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">System</span>
				</label>
			</div>
		</div>

		<!-- Action Buttons -->
		<div class="flex space-x-4">
			<button
				onclick={saveSettings}
				disabled={isSaving}
				class="btn-primary flex-1"
			>
				{isSaving ? 'Saving...' : 'Save Settings'}
			</button>
			<button
				onclick={fetchPrayersNow}
				disabled={isSaving}
				class="btn-outline flex-1"
			>
				Fetch Prayer Times Now
			</button>
		</div>
	</div>
</div>
