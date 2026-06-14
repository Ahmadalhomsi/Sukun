<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import type { PrayerTime } from '$lib/api/schema';
	import { formatTime, getTimeUntil, isTimeInPast, getTodayDate, formatDate } from '$lib/utils/time';
	import { isLoadingPrayers, errorMessage } from '$lib/stores';
	import { t, currentLanguage, translatePrayerName } from '$lib/i18n';

	let prayers = $state<PrayerTime[]>([]);
	let nextPrayer = $state<PrayerTime | null>(null);
	let currentTime = $state(new Date());

	// Update time every second
	$effect(() => {
		const interval = setInterval(() => {
			currentTime = new Date();
		}, 1000);
		return () => clearInterval(interval);
	});

	async function loadPrayers() {
		try {
			$isLoadingPrayers = true;
			prayers = await apiClient.getTodaysPrayers();

			// Find next prayer
			const upcoming = prayers.find((p) => !isTimeInPast(p.time));
			nextPrayer = upcoming || null;
			
			// Clear error if prayers were loaded successfully
			if (prayers.length > 0) {
				$errorMessage = null;
			}
		} catch (error) {
			$errorMessage = error instanceof Error ? error.message : 'Failed to load prayers';
			console.error('Error loading prayers:', error);
		} finally {
			$isLoadingPrayers = false;
		}
	}

	async function refreshPrayers() {
		const today = getTodayDate();
		try {
			$isLoadingPrayers = true;
			// This would fetch from API - for now just reload from DB
			await loadPrayers();
		} catch (error) {
			$errorMessage = error instanceof Error ? error.message : 'Failed to refresh prayers';
		}
	}

	onMount(() => {
		const init = async () => {
			try {
				// Load existing prayers first
				await loadPrayers();

				// If prayers exist, clear any previous error messages and we're done
				if (prayers.length > 0) {
					$errorMessage = null;
					console.log('Prayer times already loaded from database');
					return;
				}

			// Auto-fetch prayers on first load if location is configured but no prayers exist
				try {
					const settings = await apiClient.getAllSettings();
					const setupDone = settings.find((s) => s.key === 'setup_completed');
					const citySettings = settings.find((s) => s.key === 'city');
					const countrySettings = settings.find((s) => s.key === 'country');
					const useAutoLocationSetting = settings.find((s) => s.key === 'use_auto_location');
					const latitudeSetting = settings.find((s) => s.key === 'latitude');
					const longitudeSetting = settings.find((s) => s.key === 'longitude');

					const useAutoLocation = useAutoLocationSetting?.value === 'true';
					const latitude = parseFloat(latitudeSetting?.value || '0');
					const longitude = parseFloat(longitudeSetting?.value || '0');

					const hasCoords = useAutoLocation && latitude !== 0 && longitude !== 0;
					const hasCityCountry = citySettings?.value && countrySettings?.value && citySettings.value !== '' && countrySettings.value !== '';

					if (hasCoords || hasCityCountry) {
						$isLoadingPrayers = true;

						const tryFetch = async () => {
							const today = getTodayDate();
							if (hasCoords) {
								await apiClient.calculateAndStorePrayerTimesFromCoordinates(latitude, longitude, today);
							} else {
								await apiClient.calculateAndStorePrayerTimes(citySettings!.value, countrySettings!.value, today);
							}
							await loadPrayers();
						};

						try {
							await new Promise(resolve => setTimeout(resolve, 1500));
							await tryFetch();
						} catch (firstError) {
							console.error('Auto-fetch failed, retrying...', firstError);
							await new Promise(resolve => setTimeout(resolve, 3000));
							try {
								await tryFetch();
							} catch (retryError) {
								console.error('Auto-fetch retry failed:', retryError);
								if (prayers.length === 0) {
									$errorMessage = $t.autoFetchFailed;
								}
							}
						}
					} else if (!setupDone || setupDone.value !== 'true') {
						$errorMessage = $t.pleaseConfigure;
					}
				} catch (error) {
					console.error('Failed to load settings:', error);
					// Don't show error on settings load failure - just wait for user to configure
				}
			} catch (error) {
				console.error('Failed to initialize home:', error);
			}
		};

		init();

		// Listen for prayer updates from Settings page
		const handlePrayersUpdated = () => {
			loadPrayers();
		};
		window.addEventListener('prayers-updated', handlePrayersUpdated);

		return () => {
			window.removeEventListener('prayers-updated', handlePrayersUpdated);
		};
	});

	// Keep nextPrayer in sync as time moves without reload
	$effect(() => {
		currentTime;
		if (prayers.length === 0) return;
		const upcoming = prayers.find((p) => !isTimeInPast(p.time));
		nextPrayer = upcoming || null;
	});
</script>

<div class="container mx-auto px-4 py-8 max-w-4xl">
	<div class="mb-8">
		<h1 class="text-4xl font-bold text-gray-800 dark:text-white mb-2">
			{$t.prayerTimes}
		</h1>
		<p class="text-gray-600 dark:text-gray-400">
			{formatDate(new Date().toISOString(), $currentLanguage === 'tr' ? 'tr-TR' : 'en-US')}
		</p>
	</div>

	{#if $errorMessage}
		<div class="bg-red-100 dark:bg-red-900 border border-red-400 dark:border-red-700 text-red-700 dark:text-red-200 px-4 py-3 rounded-lg mb-6">
			<p class="font-medium">{$t.error}</p>
			<p class="text-sm">{$errorMessage}</p>
		</div>
	{/if}

	{#if $isLoadingPrayers}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-4 border-primary-500 border-t-transparent"></div>
		</div>
	{:else if nextPrayer}
		<!-- Next Prayer Card -->
		<div class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-primary-500 via-primary-600 to-primary-700 text-white p-8 mb-8 shadow-2xl transform transition-all duration-300 hover:scale-[1.02]">
			<!-- Decorative background pattern -->
			<div class="absolute inset-0 opacity-10">
				<div class="absolute top-0 right-0 w-64 h-64 bg-white rounded-full -translate-y-1/2 translate-x-1/2"></div>
				<div class="absolute bottom-0 left-0 w-48 h-48 bg-white rounded-full translate-y-1/2 -translate-x-1/2"></div>
			</div>
			
			<div class="relative flex items-center justify-between">
				<div class="flex-1">
					<p class="text-sm font-medium opacity-90 mb-2 uppercase tracking-wider">{$t.nextPrayer}</p>
					<h2 class="text-5xl font-bold mb-3 drop-shadow-lg">{translatePrayerName(nextPrayer.name, $currentLanguage)}</h2>
					<div class="flex items-center space-x-2">
						<svg class="w-5 h-5 opacity-90" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						<p class="text-2xl font-semibold opacity-95">{formatTime(nextPrayer.time)}</p>
					</div>
				</div>
				<div class="text-right">
					<p class="text-sm font-medium opacity-90 mb-2 uppercase tracking-wider">{$t.timeRemaining}</p>
					<div class="bg-white/20 backdrop-blur-sm rounded-xl px-6 py-4 border border-white/30">
						<p class="text-5xl font-bold tabular-nums drop-shadow-lg">{currentTime && getTimeUntil(nextPrayer.time, $currentLanguage)}</p>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- All Prayers List -->
	<div class="card mb-6">
		<div class="flex items-center justify-between mb-4">
			<h3 class="text-2xl font-semibold text-gray-800 dark:text-white">{$t.todaySchedule}</h3>
			<button
				onclick={refreshPrayers}
				class="btn-secondary text-sm"
				disabled={$isLoadingPrayers}
			>
				<svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
				</svg>
				{$t.refresh}
			</button>
		</div>

		{#if prayers.length === 0 && !$isLoadingPrayers}
			<div class="text-center py-12">
				<svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<p class="text-gray-600 dark:text-gray-400 mb-4">{$t.noPrayerTimes}</p>
	{#if $errorMessage && prayers.length === 0 && !$isLoadingPrayers}
					<p class="text-sm text-gray-500 dark:text-gray-500 mb-4">
						{$errorMessage}
					</p>
				{/if}
			</div>
		{:else if prayers.length === 0 && $isLoadingPrayers}
			<div class="flex items-center justify-center gap-3 py-12">
				<svg class="animate-spin w-6 h-6 text-primary-500" fill="none" viewBox="0 0 24 24">
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
					<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
				</svg>
				<p class="text-gray-500 dark:text-gray-400">{$t.fetchingPrayerTimes}</p>
			</div>
		{:else}
			<div class="space-y-3">
				{#each prayers as prayer (prayer.id || prayer.name)}
					<div
						class="group relative bg-white dark:bg-gray-800 rounded-xl p-5 border-2 transition-all duration-300 hover:shadow-xl hover:-translate-y-1"
						class:border-gray-200={isTimeInPast(prayer.time)}
						class:dark:border-gray-700={isTimeInPast(prayer.time)}
						class:opacity-60={isTimeInPast(prayer.time)}
						class:border-primary-200={!isTimeInPast(prayer.time)}
						class:dark:border-primary-800={!isTimeInPast(prayer.time)}
						class:hover:border-primary-400={!isTimeInPast(prayer.time)}
						class:dark:hover:border-primary-600={!isTimeInPast(prayer.time)}
					>
						<!-- Accent bar for active prayers -->
						{#if !isTimeInPast(prayer.time)}
							<div class="absolute left-0 top-0 bottom-0 w-1 bg-gradient-to-b from-primary-400 to-primary-600 rounded-l-xl"></div>
						{/if}
						
						<div class="flex items-center justify-between">
							<div class="flex items-center space-x-4">
								<div
									class="w-14 h-14 rounded-xl flex items-center justify-center font-bold text-xl shadow-lg transition-transform duration-300 group-hover:scale-110"
									class:bg-gradient-to-br={!isTimeInPast(prayer.time)}
									class:from-primary-500={!isTimeInPast(prayer.time)}
									class:to-primary-600={!isTimeInPast(prayer.time)}
									class:text-white={!isTimeInPast(prayer.time)}
									class:bg-gray-300={isTimeInPast(prayer.time)}
									class:dark:bg-gray-600={isTimeInPast(prayer.time)}
									class:text-gray-600={isTimeInPast(prayer.time)}
									class:dark:text-gray-400={isTimeInPast(prayer.time)}
								>
									{translatePrayerName(prayer.name, $currentLanguage).charAt(0)}
								</div>
								<div>
									<h4 class="text-lg font-semibold text-gray-800 dark:text-gray-200">{translatePrayerName(prayer.name, $currentLanguage)}</h4>
									<div class="flex items-center space-x-2 mt-1">
										{#if isTimeInPast(prayer.time)}
											<span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400">
												<svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
													<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
												</svg>
												{$t.completed}
											</span>
										{:else}
											<span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300">
												<svg class="w-3 h-3 mr-1 animate-pulse" fill="currentColor" viewBox="0 0 20 20">
													<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
												</svg>
												{$t.upcoming}
											</span>
										{/if}
									</div>
								</div>
							</div>
							<div class="text-right">
								<p class="text-3xl font-bold text-gray-800 dark:text-gray-200 tabular-nums">{formatTime(prayer.time)}</p>
								{#if !isTimeInPast(prayer.time)}
									<p class="text-sm font-medium text-primary-600 dark:text-primary-400 mt-1 tabular-nums">
										{currentTime && getTimeUntil(prayer.time, $currentLanguage)}
									</p>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Quick Actions -->
	<div class="grid grid-cols-2 gap-4">
		<button
			onclick={() => apiClient.muteAudioNow()}
			class="group relative overflow-hidden bg-gradient-to-r from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 text-white font-semibold py-4 px-6 rounded-xl shadow-lg transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
		>
			<div class="absolute inset-0 bg-white opacity-0 group-hover:opacity-10 transition-opacity duration-300"></div>
			<div class="relative flex items-center justify-center">
				<svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2" />
				</svg>
				<span>{$t.muteNow}</span>
			</div>
		</button>
		<button
			onclick={() => apiClient.unmuteAudioNow()}
			class="group relative overflow-hidden bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white font-semibold py-4 px-6 rounded-xl shadow-lg transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
		>
			<div class="absolute inset-0 bg-white opacity-0 group-hover:opacity-10 transition-opacity duration-300"></div>
			<div class="relative flex items-center justify-center">
				<svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
				</svg>
				<span>{$t.unmute}</span>
			</div>
		</button>
	</div>
</div>
