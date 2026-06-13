<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import { appConfig, themeMode } from '$lib/stores';
	import { getTodayDate } from '$lib/utils/time';
	import { t, currentLanguage, setLanguage, type Language } from '$lib/i18n';
	import LocationSelector from './LocationSelector.svelte';
	import { getCurrentPosition, getLocationFromIP } from '$lib/services/geolocation';

	let city = 'Istanbul';
	let country = 'Turkey';
	let useAutoLocation = false;
	let currentLatitude = 0;
	let currentLongitude = 0;
	let autoMute = true;
	let notificationsEnabled = true;
	let preAlertEnabled = true;
	let preAlertMinutes = 10;
	let preAlertMode: 'notification' | 'sound' | 'both' = 'both';
	let selectedTheme: 'light' | 'dark' | 'system' = 'system';
	let selectedLang: Language = 'tr';
	let timeAdjustment = 0;
	let unmuteAfterMinutes = 5;
	let showLocationDialog = false;
	let showLocationError = false;
	let autoStartEnabled = false;

	let isSaving = false;
	let saveMessage = '';

	// Load settings on mount
	onMount(async () => {
		// Set language from store
		selectedLang = $currentLanguage;
		
		// Check auto-start status
		try {
			autoStartEnabled = await apiClient.isAutoStartEnabled();
		} catch (error) {
			console.error('Failed to check auto-start status:', error);
		}
		
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
					case 'use_auto_location':
						useAutoLocation = setting.value === 'true';
						break;
					case 'latitude':
						currentLatitude = parseFloat(setting.value) || 0;
						break;
					case 'longitude':
						currentLongitude = parseFloat(setting.value) || 0;
						break;
					case 'auto_mute':
						autoMute = setting.value === 'true';
						break;
					case 'notifications_enabled':
						notificationsEnabled = setting.value === 'true';
						break;
					case 'pre_prayer_alert_enabled':
						preAlertEnabled = setting.value !== 'false';
						break;
					case 'pre_prayer_alert_minutes':
						preAlertMinutes = parseInt(setting.value) || 10;
						break;
					case 'pre_prayer_alert_mode':
							if (setting.value === 'sound') {
								preAlertMode = 'sound';
						} else if (setting.value === 'notification') {
							preAlertMode = 'notification';
						} else {
							// Default to 'both' for new users or if value is 'both'
							preAlertMode = 'both';
							}
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
	


	async function toggleAutoStart() {
		try {
			if (autoStartEnabled) {
				await apiClient.disableAutoStart();
				autoStartEnabled = false;
				saveMessage = '✅ Auto-start disabled';
			} else {
				await apiClient.enableAutoStart();
				autoStartEnabled = true;
				saveMessage = '✅ Auto-start enabled - Sukun will start with Windows';
			}
			setTimeout(() => saveMessage = '', 3000);
		} catch (error) {
			saveMessage = '❌ Failed to toggle auto-start';
			console.error('Auto-start toggle error:', error);
			setTimeout(() => saveMessage = '', 3000);
		}
}

function handleLocationSelect(newCity: string, newCountry: string) {
	city = newCity;
	country = newCountry;
}

function handleLanguageChange() {
	setLanguage(selectedLang);
}

async function useMyLocation() {
		try {
			isSaving = true;
			showLocationError = false;
			saveMessage = 'Getting your location...';

			const position = await getCurrentPosition();
			currentLatitude = position.latitude;
			currentLongitude = position.longitude;
			useAutoLocation = true;

			city = 'GPS Location';
			country = `${position.latitude.toFixed(4)}, ${position.longitude.toFixed(4)}`;

			saveMessage = `✅ Location found: ${position.latitude.toFixed(4)}, ${position.longitude.toFixed(4)}`;
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			showLocationError = true;
			saveMessage = error instanceof Error ? error.message : 'Failed to get location';
			console.error('Geolocation error:', error);
		} finally {
			isSaving = false;
		}
	}

	async function useIPLocation() {
		try {
			isSaving = true;
			showLocationError = false;
			saveMessage = 'Getting location from IP address...';

			const location = await getLocationFromIP();
			currentLatitude = location.latitude;
			currentLongitude = location.longitude;
			useAutoLocation = true;

			city = location.city;
			country = location.country;

			saveMessage = `✅ Location found: ${location.city}, ${location.country}`;
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			saveMessage = error instanceof Error ? error.message : 'Failed to get location from IP';
			console.error('IP Location error:', error);
		} finally {
			isSaving = false;
		}
	}

	async function saveSettings() {
		try {
			isSaving = true;
			saveMessage = '';

			// Save to backend
			await apiClient.setSetting('city', city);
			await apiClient.setSetting('country', country);
			await apiClient.setSetting('use_auto_location', useAutoLocation.toString());
			await apiClient.setSetting('latitude', currentLatitude.toString());
			await apiClient.setSetting('longitude', currentLongitude.toString());
			await apiClient.setSetting('auto_mute', autoMute.toString());
			await apiClient.setSetting('notifications_enabled', notificationsEnabled.toString());
			await apiClient.setSetting('pre_prayer_alert_enabled', preAlertEnabled.toString());
			await apiClient.setSetting('pre_prayer_alert_minutes', preAlertMinutes.toString());
			await apiClient.setSetting('pre_prayer_alert_mode', preAlertMode);
			await apiClient.setSetting('theme_mode', selectedTheme);
			await apiClient.setSetting('time_adjustment', timeAdjustment.toString());
			await apiClient.setSetting('unmute_after_minutes', unmuteAfterMinutes.toString());
			await apiClient.setSetting('language', selectedLang);
			await apiClient.setSetting('calculation_method', 'Turkey');

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

			const today = getTodayDate();
			
			if (useAutoLocation && currentLatitude !== 0 && currentLongitude !== 0) {
				// Use coordinates
				await apiClient.calculateAndStorePrayerTimesFromCoordinates(
					currentLatitude,
					currentLongitude,
					today
				);
			} else {
				// Use city/country
				if (!city || !country) {
					saveMessage = $t.enterCityCountry;
					return;
				}
				await apiClient.calculateAndStorePrayerTimes(city, country, today);
			}
			
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

	function playAlertSoundPreview() {
		try {
			const audio = new Audio('/Smart_UI_Notification_Stylized_Calm_19_Menu_UI_Indie_Chill.wav');
			audio.currentTime = 0;
			audio.play().catch((err) => console.error('Preview sound failed', err));
		} catch (err) {
			console.error('Preview sound error', err);
		}
	}

	async function testPreAlert() {
		if (preAlertMode === 'sound') {
			playAlertSoundPreview();
			return;
		}

		if (preAlertMode === 'both') {
			// Test both sound and notification
			playAlertSoundPreview();
			try {
				const result = await apiClient.testNotification();
				console.log(result);
				saveMessage = '✅ Test notification and sound sent!';
				setTimeout(() => saveMessage = '', 3000);
			} catch (error) {
				console.error('Notification test failed:', error);
				saveMessage = '❌ Notification test failed. Check console for details.';
				setTimeout(() => saveMessage = '', 3000);
			}
			return;
		}

		// Use Tauri notification instead of browser notification
		try {
			const result = await apiClient.testNotification();
			console.log(result);
			saveMessage = '✅ Test notification sent!';
			setTimeout(() => saveMessage = '', 3000);
		} catch (error) {
			console.error('Notification test failed:', error);
			saveMessage = '❌ Notification test failed. Check console for details.';
			setTimeout(() => saveMessage = '', 3000);
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
			
			<div class="mb-4">
				<label class="flex items-center space-x-3 cursor-pointer mb-4">
					<input
						type="checkbox"
						bind:checked={useAutoLocation}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						Use my device location automatically
					</span>
				</label>
			</div>

			{#if useAutoLocation}
				<div class="mb-4 space-y-3">
					<div class="grid grid-cols-2 gap-3">
						<button
							onclick={useMyLocation}
							disabled={isSaving}
							class="btn-primary"
						>
							<svg class="w-5 h-5 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
							</svg>
							{isSaving ? 'Getting...' : 'Use GPS'}
						</button>
						<button
							onclick={useIPLocation}
							disabled={isSaving}
							class="btn-secondary"
						>
							<svg class="w-5 h-5 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
							</svg>
							Use IP
						</button>
					</div>
					
					{#if showLocationError}
						<div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
							<p class="text-sm font-medium text-yellow-800 dark:text-yellow-200 mb-2">
								⚠️ Windows Location Services may be disabled
							</p>
							<ol class="text-xs text-yellow-700 dark:text-yellow-300 list-decimal list-inside space-y-1">
								<li>Open Windows Settings</li>
								<li>Go to Privacy & Security → Location</li>
								<li>Turn ON "Location services"</li>
								<li>Allow this app to access location</li>
							</ol>
							<p class="text-xs text-yellow-700 dark:text-yellow-300 mt-2">
								Or use the "Use IP" button for approximate location.
							</p>
						</div>
					{/if}
					
					{#if currentLatitude !== 0 && currentLongitude !== 0}
						<div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-3">
							<p class="text-sm text-green-700 dark:text-green-300">
								📍 {city}, {country}
							</p>
							<p class="text-xs text-green-600 dark:text-green-400">
								Coordinates: {currentLatitude.toFixed(4)}, {currentLongitude.toFixed(4)}
							</p>
						</div>
					{/if}
				</div>
			{:else}
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
			{/if}
			
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
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={preAlertEnabled}
						disabled={!notificationsEnabled && preAlertMode === 'notification'}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						{$currentLanguage === 'tr' ? `Namazdan ${preAlertMinutes} dk önce uyar` : `Alert ${preAlertMinutes} minutes before prayer`}
					</span>
				</label>
				{#if preAlertEnabled}
					<div class="ml-8 space-y-3">
						<div class="max-w-xs">
							<label for="preAlertMinutes" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
								{$currentLanguage === 'tr' ? 'Uyarı dakikası' : 'Alert minutes before'}
							</label>
							<input
								id="preAlertMinutes"
								type="number"
								min="1"
								max="120"
								bind:value={preAlertMinutes}
								class="input-field"
							/>
							<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
								{$currentLanguage === 'tr' ? 'Namazdan kaç dakika önce bildirim gösterilsin' : 'How many minutes before prayer to notify'}
							</p>
						</div>

						<div>
							<p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">{$currentLanguage === 'tr' ? 'Uyarı türü' : 'Alert type'}</p>
							<div class="space-y-2">
								<label class="flex items-center space-x-3 cursor-pointer">
									<input
										type="radio"
										bind:group={preAlertMode}
										value="notification"
										class="w-4 h-4 text-primary-500"
									/>
									<span class="text-gray-700 dark:text-gray-300">
										{$currentLanguage === 'tr' ? 'Windows bildirimi' : 'Windows notification'}
									</span>
								</label>
								<label class="flex items-center space-x-3 cursor-pointer">
									<input
										type="radio"
										bind:group={preAlertMode}
										value="sound"
										class="w-4 h-4 text-primary-500"
									/>
									<span class="text-gray-700 dark:text-gray-300">
										{$currentLanguage === 'tr' ? 'Sesli uyarı' : 'Sound alert'}
									</span>
								</label>
								<label class="flex items-center space-x-3 cursor-pointer">
									<input
										type="radio"
										bind:group={preAlertMode}
										value="both"
										class="w-4 h-4 text-primary-500"
									/>
									<span class="text-gray-700 dark:text-gray-300">
										{$currentLanguage === 'tr' ? 'Her ikisi (Bildirim + Ses)' : 'Both (Notification + Sound)'}
									</span>
								</label>
							</div>
						</div>

						<button class="btn-secondary text-sm" onclick={testPreAlert}>
							{$currentLanguage === 'tr' ? 'Test et' : 'Test alert'}
						</button>
					</div>
				{/if}
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

		<!-- Auto Start -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				{$currentLanguage === 'tr' ? 'Başlangıç' : 'Startup'}
			</h2>
			<div class="space-y-4">
				<label class="flex items-center justify-between cursor-pointer">
					<div>
						<span class="text-gray-700 dark:text-gray-300 font-medium">
							{$currentLanguage === 'tr' ? 'Windows ile başlat' : 'Start with Windows'}
						</span>
						<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
							{$currentLanguage === 'tr' 
								? 'Sukun bilgisayar açıldığında otomatik olarak başlasın' 
								: 'Automatically start Sukun when your computer boots'}
						</p>
					</div>
					<button
						onclick={toggleAutoStart}
						aria-label={autoStartEnabled ? 'Disable auto-start' : 'Enable auto-start'}
						class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
						class:bg-primary-600={autoStartEnabled}
						class:bg-gray-300={!autoStartEnabled}
					>
						<span
							class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
							class:translate-x-6={autoStartEnabled}
							class:translate-x-1={!autoStartEnabled}
						></span>
					</button>
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
