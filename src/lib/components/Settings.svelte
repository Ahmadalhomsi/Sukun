<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import { appConfig, themeMode } from '$lib/stores';
	import { getTodayDate } from '$lib/utils/time';
	import { t, currentLanguage, setLanguage, type Language } from '$lib/i18n';
	import LocationSelector from './LocationSelector.svelte';

	let city = 'Istanbul';
	let country = 'Turkey';
	let autoMute = true;
	let notificationsEnabled = true;
	let selectedTheme: 'light' | 'dark' | 'system' = 'system';
	let selectedLang: Language = 'tr';
	let timeAdjustment = 0; // Minutes to adjust prayer times (can be negative)
	let unmuteAfterMinutes = 5; // Auto-unmute after X minutes
	let showLocationDialog = false;

	let isSaving = false;
	let saveMessage = '';

	// Load settings on mount
	onMount(async () => {
		// Set language from store
		selectedLang = $currentLanguage;
		
		try {
			const settings = await apiClient.getAllSettings();
			settings.forEach((setting: any) => {
				switch (setting.key) {
					case 'city':
						city = setting.value || 'Istanbul';
						break;
					case 'country':
						country = setting.value || 'Turkey';
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
					case 'time_adjustment':
						timeAdjustment = parseInt(setting.value) || 0;
						break;
					case 'unmute_after_minutes':
						unmuteAfterMinutes = parseInt(setting.value) || 5;
						break;
					case 'language':
						selectedLang = setting.value as Language;
						setLanguage(selectedLang);
						break;
				}
			});
		} catch (error) {
			console.error('Failed to load settings:', error);
		}
	});
	
	function handleLocationSelect(newCity: string, newCountry: string) {
		city = newCity;
		country = newCountry;
	}
	
	function handleLanguageChange() {
		setLanguage(selectedLang);
	}

	async function saveSettings() {
		try {
			isSaving = true;
			saveMessage = '';

			// Save to backend
			await apiClient.setSetting('city', city);
			await apiClient.setSetting('country', country);
			await apiClient.setSetting('auto_mute', autoMute.toString());
			await apiClient.setSetting('notifications_enabled', notificationsEnabled.toString());
			await apiClient.setSetting('theme_mode', selectedTheme);
			await apiClient.setSetting('time_adjustment', timeAdjustment.toString());
			await apiClient.setSetting('unmute_after_minutes', unmuteAfterMinutes.toString());
			await apiClient.setSetting('language', selectedLang);

			// Update local stores
			$appConfig = {
				city,
				country,
				autoMute,
				notificationsEnabled,
				themeMode: selectedTheme
			};

			$themeMode = selectedTheme;

			saveMessage = $t.settingsSaved;
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			saveMessage = $t.failedToSave;
			console.error('Error saving settings:', error);
		} finally {
			isSaving = false;
		}
	}

	async function fetchPrayersNow() {
		try {
			isSaving = true;
			saveMessage = '';

			if (!city || !country) {
				saveMessage = $t.enterCityCountry;
				return;
			}

			const today = getTodayDate();
			await apiClient.fetchAndStorePrayerTimes(city, country, today);
			
			saveMessage = $t.prayerTimesFetched;
			
			// Dispatch event to refresh home page
			window.dispatchEvent(new CustomEvent('prayers-updated'));
			
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			saveMessage = $t.failedToFetch;
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

<LocationSelector
	bind:show={showLocationDialog}
	bind:selectedCity={city}
	bind:selectedCountry={country}
	{handleLocationSelect}
	onSelect={handleLocationSelect}
/>

<div class="container mx-auto px-4 py-8 max-w-3xl">
	<h1 class="text-4xl font-bold text-gray-800 dark:text-white mb-8">{$t.settings}</h1>

	{#if saveMessage}
		<div
			class="mb-6 px-4 py-3 rounded-lg"
			class:bg-green-100={saveMessage.includes($t.success) || saveMessage === $t.settingsSaved || saveMessage === $t.prayerTimesFetched}
			class:text-green-700={saveMessage.includes($t.success) || saveMessage === $t.settingsSaved || saveMessage === $t.prayerTimesFetched}
			class:dark:bg-green-900={saveMessage.includes($t.success) || saveMessage === $t.settingsSaved || saveMessage === $t.prayerTimesFetched}
			class:dark:text-green-200={saveMessage.includes($t.success) || saveMessage === $t.settingsSaved || saveMessage === $t.prayerTimesFetched}
			class:bg-red-100={saveMessage === $t.failedToSave || saveMessage === $t.failedToFetch || saveMessage === $t.enterCityCountry}
			class:text-red-700={saveMessage === $t.failedToSave || saveMessage === $t.failedToFetch || saveMessage === $t.enterCityCountry}
			class:dark:bg-red-900={saveMessage === $t.failedToSave || saveMessage === $t.failedToFetch || saveMessage === $t.enterCityCountry}
			class:dark:text-red-200={saveMessage === $t.failedToSave || saveMessage === $t.failedToFetch || saveMessage === $t.enterCityCountry}
		>
			{saveMessage}
		</div>
	{/if}

	<div class="space-y-6">
		<!-- Location Settings -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				{$t.location}
			</h2>
			<div class="flex items-center justify-between mb-4">
				<div class="flex-1">
					<p class="text-gray-700 dark:text-gray-300">
						<span class="font-medium">{$t.city}:</span> {city || 'Not set'}
					</p>
					<p class="text-gray-700 dark:text-gray-300">
						<span class="font-medium">{$t.country}:</span> {country || 'Not set'}
					</p>
				</div>
				<button
					onclick={() => showLocationDialog = true}
					class="btn-primary"
				>
					{$t.selectLocation}
				</button>
			</div>
			<p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
				{$t.usingFreeApi}
			</p>
		</div>

		<!-- Audio Behavior -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				{$t.audioBehavior}
			</h2>
			<div class="space-y-4">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={autoMute}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						{$t.autoMuteAudio}
					</span>
				</label>
				{#if autoMute}
					<div class="ml-8">
						<label for="unmuteAfter" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{$t.autoUnmuteAfter}
						</label>
						<input
							id="unmuteAfter"
							type="number"
							bind:value={unmuteAfterMinutes}
							min="1"
							max="60"
							class="input-field max-w-xs"
						/>
						<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
							{$t.autoUnmuteHelp}
						</p>
					</div>
				{/if}
			</div>
		</div>

		<!-- Notifications -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				{$t.notifications}
			</h2>
			<div class="space-y-4">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={notificationsEnabled}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						{$t.showNotifications}
					</span>
				</label>
			</div>
		</div>

		<!-- Language -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				{$t.language}
			</h2>
			<div class="space-y-2">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedLang}
						value="tr"
						onchange={handleLanguageChange}
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">Türkçe</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedLang}
						value="en"
						onchange={handleLanguageChange}
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">English</span>
				</label>
			</div>
		</div>

		<!-- Theme -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				{$t.theme}
			</h2>
			<div class="space-y-2">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="light"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">{$t.themeLight}</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="dark"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">{$t.themeDark}</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="system"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">{$t.themeSystem}</span>
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
				{isSaving ? $t.saving : $t.saveSettings}
			</button>
			<button
				onclick={fetchPrayersNow}
				disabled={isSaving}
				class="btn-outline flex-1"
			>
				{$t.fetchPrayerTimes}
			</button>
		</div>
	</div>
</div>
