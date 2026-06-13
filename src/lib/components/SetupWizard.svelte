<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import { t, currentLanguage, setLanguage, type Language } from '$lib/i18n';
	import { countries } from '$lib/data/locations';
	import { getCurrentPosition, getLocationFromIP } from '$lib/services/geolocation';
	import { getTodayDate } from '$lib/utils/time';
	import { themeMode } from '$lib/stores';

	let step = $state(0);
	let selectedLang = $state<Language>('tr');
	let selectedTheme = $state<'light' | 'dark' | 'system'>('system');
	let city = $state('');
	let country = $state('');
	let useAutoLocation = $state(false);
	let currentLatitude = $state(0);
	let currentLongitude = $state(0);
	let selectedCountryData = $state(countries[0]);
	let searchCity = $state('');
	let locationMode = $state<'manual' | 'gps' | 'ip'>('manual');
	let isProcessing = $state(false);
	let errorMessage = $state('');
	let locationPicked = $state(false);

	let filteredCities = $derived(
		selectedCountryData.cities.filter((c) =>
			c.name.toLowerCase().includes(searchCity.toLowerCase())
		)
	);

	onMount(() => {
		selectedLang = $currentLanguage;
	});

	function applyTheme() {
		if (
			selectedTheme === 'dark' ||
			(selectedTheme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
		) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}

	function pickLanguage(lang: Language) {
		selectedLang = lang;
		setLanguage(lang);
	}

	function pickTheme(theme: 'light' | 'dark' | 'system') {
		selectedTheme = theme;
		applyTheme();
	}

	function handleCountryChange(name: string) {
		selectedCountryData = countries.find((c) => c.name === name) || countries[0];
		searchCity = '';
	}

	function pickCity(cityName: string) {
		city = cityName;
		country = selectedCountryData.name;
		locationMode = 'manual';
		useAutoLocation = false;
		locationPicked = true;
	}

	async function useGPS() {
		isProcessing = true;
		errorMessage = '';
		try {
			const pos = await getCurrentPosition();
			currentLatitude = pos.latitude;
			currentLongitude = pos.longitude;
			useAutoLocation = true;
			locationMode = 'gps';
			city = 'GPS Location';
			country = `${pos.latitude.toFixed(4)}, ${pos.longitude.toFixed(4)}`;
			locationPicked = true;
		} catch (error) {
			errorMessage = selectedLang === 'tr'
				? 'Konum alınamadı. Windows Konum Servislerini kontrol edin.'
				: 'Failed to get location. Check Windows Location Services.';
		} finally {
			isProcessing = false;
		}
	}

	async function useIP() {
		isProcessing = true;
		errorMessage = '';
		try {
			const loc = await getLocationFromIP();
			currentLatitude = loc.latitude;
			currentLongitude = loc.longitude;
			useAutoLocation = true;
			locationMode = 'ip';
			city = loc.city;
			country = loc.country;
			locationPicked = true;
		} catch (error) {
			errorMessage = selectedLang === 'tr'
				? 'IP ile konum alınamadı.'
				: 'Failed to get location from IP.';
		} finally {
			isProcessing = false;
		}
	}

	async function finish() {
		isProcessing = true;
		errorMessage = '';

		try {
			await apiClient.setSetting('language', selectedLang);
			await apiClient.setSetting('theme_mode', selectedTheme);
			await apiClient.setSetting('city', city);
			await apiClient.setSetting('country', country);
			await apiClient.setSetting('use_auto_location', useAutoLocation.toString());
			await apiClient.setSetting('latitude', currentLatitude.toString());
			await apiClient.setSetting('longitude', currentLongitude.toString());
			await apiClient.setSetting('calculation_method', 'Turkey');
			await apiClient.setSetting('auto_mute', 'true');
			await apiClient.setSetting('notifications_enabled', 'true');
			await apiClient.setSetting('pre_prayer_alert_enabled', 'true');
			await apiClient.setSetting('pre_prayer_alert_minutes', '10');
			await apiClient.setSetting('pre_prayer_alert_mode', 'both');
			await apiClient.setSetting('setup_completed', 'true');

			$themeMode = selectedTheme;

			const today = getTodayDate();
			if (useAutoLocation && currentLatitude !== 0 && currentLongitude !== 0) {
				await apiClient.calculateAndStorePrayerTimesFromCoordinates(
					currentLatitude,
					currentLongitude,
					today
				);
			} else if (city && country) {
				await apiClient.calculateAndStorePrayerTimes(city, country, today);
			}

			window.dispatchEvent(new CustomEvent('setup-complete'));
		} catch (error) {
			errorMessage = selectedLang === 'tr'
				? 'Bir hata oluştu. Lütfen tekrar deneyin.'
				: 'An error occurred. Please try again.';
			isProcessing = false;
		}
	}

	const steps = [
		{ title: () => selectedLang === 'tr' ? 'Dil Seçimi' : 'Choose Language' },
		{ title: () => selectedLang === 'tr' ? 'Konum' : 'Location' },
		{ title: () => selectedLang === 'tr' ? 'Görünüm' : 'Appearance' }
	];

	function langBtn(lang: Language) {
		return `w-full flex items-center gap-4 p-4 rounded-xl border-2 transition-all duration-200 ${
			selectedLang === lang
				? 'border-primary-500 bg-primary-50 dark:bg-gray-700'
				: 'border-gray-200 dark:border-gray-600 hover:border-primary-300'
		}`;
	}

	function themeBtn(theme: string) {
		return `flex flex-col items-center gap-3 p-5 rounded-xl border-2 transition-all duration-200 ${
			selectedTheme === theme
				? 'border-primary-500 bg-primary-50 dark:bg-gray-700'
				: 'border-gray-200 dark:border-gray-600 hover:border-primary-300'
		}`;
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-gradient-to-br from-primary-500/10 to-primary-600/5 dark:from-gray-900 dark:to-gray-800 p-4">
	<div class="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl w-full max-w-lg max-h-[90vh] flex flex-col overflow-hidden">
		<!-- Header with Logo -->
		<div class="flex flex-col items-center pt-10 pb-6 bg-gradient-to-b from-primary-50 to-transparent dark:from-gray-700/50">
			<div class="w-16 h-16 bg-gradient-to-br from-primary-500 to-primary-600 rounded-2xl flex items-center justify-center shadow-lg mb-4">
				<svg class="w-9 h-9 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
			</div>
			<h1 class="text-2xl font-bold text-gray-800 dark:text-white">{$t.appTitle}</h1>
			<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
				{selectedLang === 'tr' ? 'Namaz Vakitleri Yöneticisi' : 'Prayer Times Manager'}
			</p>
		</div>

		<!-- Step Progress -->
		<div class="flex items-center justify-center gap-2 px-6 pb-4">
			{#each steps as s, i}
				<div class="flex items-center gap-2">
					<div
						class="flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold transition-all duration-300 {i <= step ? 'bg-primary-500 text-white' : 'bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400'}"
					>
						{i + 1}
					</div>
					{#if i < steps.length - 1}
						<div
							class="w-8 h-0.5 rounded-full transition-all duration-300 {i < step ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-600'}"
						></div>
					{/if}
				</div>
			{/each}
		</div>

		<!-- Content -->
		<div class="flex-1 overflow-y-auto px-8 py-2">
			<!-- Step 0: Language -->
			{#if step === 0}
				<div class="animate-fade-in">
					<h2 class="text-xl font-bold text-gray-800 dark:text-white mb-2 text-center">
						{steps[0].title()}
					</h2>
					<p class="text-sm text-gray-500 dark:text-gray-400 mb-6 text-center">
						{selectedLang === 'tr' ? 'Devam etmek için bir dil seçin' : 'Select a language to continue'}
					</p>
					<div class="space-y-3">
						<button onclick={() => pickLanguage('tr')} class={langBtn('tr')}>
							<span class="text-3xl">🇹🇷</span>
							<div class="text-left">
								<p class="font-semibold text-gray-800 dark:text-white">Türkçe</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">Turkey</p>
							</div>
							{#if selectedLang === 'tr'}
								<svg class="w-5 h-5 text-primary-500 ml-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
							{/if}
						</button>
						<button onclick={() => pickLanguage('en')} class={langBtn('en')}>
							<span class="text-3xl">🇬🇧</span>
							<div class="text-left">
								<p class="font-semibold text-gray-800 dark:text-white">English</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">United States</p>
							</div>
							{#if selectedLang === 'en'}
								<svg class="w-5 h-5 text-primary-500 ml-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
							{/if}
						</button>
					</div>
				</div>
			{/if}

			<!-- Step 1: Location -->
			{#if step === 1}
				<div class="animate-fade-in">
					<h2 class="text-xl font-bold text-gray-800 dark:text-white mb-2 text-center">
						{steps[1].title()}
					</h2>
					<p class="text-sm text-gray-500 dark:text-gray-400 mb-6 text-center">
						{selectedLang === 'tr' ? 'Namaz vakitleri için konumunuzu seçin' : 'Choose your location for prayer times'}
					</p>

					{#if errorMessage}
						<div class="mb-4 px-4 py-3 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800">
							<p class="text-sm text-red-700 dark:text-red-300">{errorMessage}</p>
						</div>
					{/if}

					<!-- Quick Location Methods -->
					<div class="grid grid-cols-2 gap-3 mb-5">
						<button
							onclick={useGPS}
							disabled={isProcessing}
							class="flex flex-col items-center gap-2 p-4 rounded-xl border-2 border-gray-200 dark:border-gray-600 hover:border-primary-400 transition-all duration-200 disabled:opacity-50"
						>
							<svg class="w-7 h-7 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
							</svg>
							<span class="text-sm font-medium text-gray-700 dark:text-gray-300">
								{selectedLang === 'tr' ? 'GPS ile' : 'Use GPS'}
							</span>
						</button>
						<button
							onclick={useIP}
							disabled={isProcessing}
							class="flex flex-col items-center gap-2 p-4 rounded-xl border-2 border-gray-200 dark:border-gray-600 hover:border-primary-400 transition-all duration-200 disabled:opacity-50"
						>
							<svg class="w-7 h-7 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
							</svg>
							<span class="text-sm font-medium text-gray-700 dark:text-gray-300">
								{selectedLang === 'tr' ? 'IP ile' : 'Use IP'}
							</span>
						</button>
					</div>

					{#if isProcessing}
						<div class="flex items-center justify-center gap-2 py-4">
							<svg class="animate-spin w-5 h-5 text-primary-500" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
							</svg>
							<span class="text-sm text-gray-500 dark:text-gray-400">
								{selectedLang === 'tr' ? 'Konum alınıyor...' : 'Getting location...'}
							</span>
						</div>
					{/if}

					<!-- Divider -->
					<div class="flex items-center gap-3 my-5">
						<div class="flex-1 h-px bg-gray-200 dark:bg-gray-700"></div>
						<span class="text-xs text-gray-400 dark:text-gray-500">
							{selectedLang === 'tr' ? 'veya şehir seçin' : 'or pick a city'}
						</span>
						<div class="flex-1 h-px bg-gray-200 dark:bg-gray-700"></div>
					</div>

					<!-- Country Select -->
					<div class="mb-3">
						<label for="wiz-country" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
							{$t.country}
						</label>
						<select
							id="wiz-country"
							value={selectedCountryData.name}
							onchange={(e) => handleCountryChange(e.currentTarget.value)}
							class="input-field"
						>
							{#each countries as c}
								<option value={c.name}>{c.name}</option>
							{/each}
						</select>
					</div>

					<!-- City Search -->
					<div class="mb-3">
						<label for="wiz-city-search" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
							{$t.city}
						</label>
						<input
							id="wiz-city-search"
							type="text"
							bind:value={searchCity}
							placeholder={selectedLang === 'tr' ? 'Şehir ara...' : 'Search cities...'}
							class="input-field"
						/>
					</div>

					<!-- City Grid -->
					<div class="grid grid-cols-2 gap-2 max-h-36 overflow-y-auto">
						{#each filteredCities as c}
							<button
								onclick={() => pickCity(c.name)}
								class="p-2.5 text-left rounded-lg border-2 transition-all duration-150 {city === c.name && locationMode === 'manual' ? 'border-primary-500 bg-primary-50 dark:bg-gray-700 text-primary-700 dark:text-primary-300' : 'border-gray-200 dark:border-gray-600 hover:border-primary-300'}"
							>
								<span class="text-sm font-medium text-gray-700 dark:text-gray-300">{c.name}</span>
							</button>
						{/each}
					</div>

					<!-- Selected Location Display -->
					{#if locationPicked}
						<div class="mt-4 flex items-center gap-2 px-4 py-3 rounded-lg bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800">
							<svg class="w-5 h-5 text-green-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
							</svg>
							<div class="text-sm">
								<p class="font-medium text-green-800 dark:text-green-200">
									{city}{#if locationMode === 'manual'}, {country}{/if}
								</p>
								{#if useAutoLocation && currentLatitude !== 0}
									<p class="text-xs text-green-600 dark:text-green-400">
										{currentLatitude.toFixed(4)}, {currentLongitude.toFixed(4)}
									</p>
								{/if}
							</div>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Step 2: Theme -->
			{#if step === 2}
				<div class="animate-fade-in">
					<h2 class="text-xl font-bold text-gray-800 dark:text-white mb-2 text-center">
						{steps[2].title()}
					</h2>
					<p class="text-sm text-gray-500 dark:text-gray-400 mb-6 text-center">
						{selectedLang === 'tr' ? 'Görünüm tercihinizi seçin' : 'Choose your appearance preference'}
					</p>
					<div class="grid grid-cols-3 gap-3">
						<button onclick={() => pickTheme('light')} class={themeBtn('light')}>
							<div class="w-12 h-12 rounded-lg bg-white border-2 border-gray-200 flex items-center justify-center">
								<svg class="w-6 h-6 text-amber-400" fill="currentColor" viewBox="0 0 24 24">
									<path d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58c-.39-.39-1.03-.39-1.41 0-.39.39-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37c-.39-.39-1.03-.39-1.41 0-.39.39-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0 .39-.39.39-1.03 0-1.41l-1.06-1.06zm1.06-10.96c.39-.39.39-1.03 0-1.41-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06zM7.05 18.36c.39-.39.39-1.03 0-1.41-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06z" />
								</svg>
							</div>
							<span class="text-sm font-medium text-gray-700 dark:text-gray-300">{$t.themeLight}</span>
						</button>
						<button onclick={() => pickTheme('dark')} class={themeBtn('dark')}>
							<div class="w-12 h-12 rounded-lg bg-gray-900 border-2 border-gray-700 flex items-center justify-center">
								<svg class="w-6 h-6 text-indigo-300" fill="currentColor" viewBox="0 0 24 24">
									<path d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9 9-4.03 9-9c0-.46-.04-.92-.1-1.36-.98 1.37-2.58 2.26-4.4 2.26-2.98 0-5.4-2.42-5.4-5.4 0-1.81.89-3.42 2.26-4.4-.44-.06-.9-.1-1.36-.1z" />
								</svg>
							</div>
							<span class="text-sm font-medium text-gray-700 dark:text-gray-300">{$t.themeDark}</span>
						</button>
						<button onclick={() => pickTheme('system')} class={themeBtn('system')}>
							<div class="w-12 h-12 rounded-lg bg-gradient-to-br from-white to-gray-800 border-2 border-gray-300 flex items-center justify-center">
								<svg class="w-6 h-6 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
								</svg>
							</div>
							<span class="text-sm font-medium text-gray-700 dark:text-gray-300">{$t.themeSystem}</span>
						</button>
					</div>

					{#if locationPicked}
						<div class="mt-6 px-4 py-3 rounded-lg bg-primary-50 dark:bg-gray-700 border border-primary-200 dark:border-gray-600">
							<p class="text-sm text-gray-700 dark:text-gray-300">
								{selectedLang === 'tr'
									? 'Hazırsınız! Sukun artık namaz vakitlerinde sistemi otomatik sessize alacaktır.'
									: 'You are all set! Sukun will now automatically mute your system at prayer times.'}
							</p>
						</div>
					{:else}
						<div class="mt-6 px-4 py-3 rounded-lg bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800">
							<p class="text-sm text-amber-700 dark:text-amber-300">
								{selectedLang === 'tr'
									? '⚠️ Konum seçmeden devam ediyorsunuz. Namaz vakitlerini almak için daha sonra Ayarlar\'dan konum seçmeniz gerekir.'
									: '⚠️ You are continuing without a location. You will need to select one in Settings later to get prayer times.'}
							</p>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Footer Navigation -->
		<div class="flex items-center justify-between px-8 py-5 border-t border-gray-200 dark:border-gray-700">
			<button
				onclick={() => (step = Math.max(0, step - 1))}
				disabled={step === 0}
				class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 disabled:opacity-40 hover:text-gray-800 dark:hover:text-gray-200 transition-colors"
			>
				{selectedLang === 'tr' ? '← Geri' : '← Back'}
			</button>

			{#if step < steps.length - 1}
				<button onclick={() => step++} class="btn-primary px-8">
					{selectedLang === 'tr' ? 'İleri' : 'Next'}
				</button>
			{:else}
				<button onclick={finish} disabled={isProcessing} class="btn-primary px-8">
					{#if isProcessing}
						<span class="flex items-center gap-2">
							<svg class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
							</svg>
							{selectedLang === 'tr' ? 'Tamamlanıyor...' : 'Finishing...'}
						</span>
					{:else}
						{selectedLang === 'tr' ? 'Başla ✓' : 'Get Started ✓'}
					{/if}
				</button>
			{/if}
		</div>
	</div>
</div>
