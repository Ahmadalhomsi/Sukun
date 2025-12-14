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
			$errorMessage = null;
			prayers = await apiClient.getTodaysPrayers();

			// Find next prayer
			const upcoming = prayers.find((p) => !isTimeInPast(p.time));
			nextPrayer = upcoming || null;
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

				// Auto-fetch prayers on first load if location is configured but no prayers exist
				if (prayers.length === 0) {
					const settings = await apiClient.getAllSettings();
					const citySettings = settings.find((s) => s.key === 'city');
					const countrySettings = settings.find((s) => s.key === 'country');

					if (citySettings?.value && countrySettings?.value) {
						try {
							console.log('Auto-fetching prayer times for:', citySettings.value, countrySettings.value);
							const today = getTodayDate();
							await apiClient.fetchAndStorePrayerTimes(citySettings.value, countrySettings.value, today);
							await loadPrayers();
						} catch (error) {
							console.error('Auto-fetch failed:', error);
							$errorMessage = 'Auto-fetch failed. Please configure location in Settings.';
						}
					} else {
						console.log('Location not configured, skipping auto-fetch');
						$errorMessage = 'Please configure your location in Settings to load prayer times.';
					}
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
		<div class="prayer-card mb-8 animate-fade-in animate-pulse-glow">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm opacity-90 mb-1">{$t.nextPrayer}</p>
					<h2 class="text-3xl font-bold mb-2">{translatePrayerName(nextPrayer.name, $currentLanguage)}</h2>
					<p class="text-xl opacity-95">{formatTime(nextPrayer.time)}</p>
				</div>
				<div class="text-right">
					<p class="text-sm opacity-90 mb-1">{$t.timeRemaining}</p>
					<p class="text-4xl font-bold">{currentTime && getTimeUntil(nextPrayer.time, $currentLanguage)}</p>
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

		{#if prayers.length === 0}
			<div class="text-center py-12">
				<svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<p class="text-gray-600 dark:text-gray-400 mb-4">{$t.noPrayerTimes}</p>
				<p class="text-sm text-gray-500 dark:text-gray-500 mb-4">
					{$t.configureLocation}
				</p>
			</div>
		{:else}
			<div class="space-y-3">
				{#each prayers as prayer (prayer.id || prayer.name)}
					<div
						class="prayer-card-inactive hover:shadow-lg transition-all duration-200"
						class:opacity-50={isTimeInPast(prayer.time)}
					>
						<div class="flex items-center justify-between">
							<div class="flex items-center space-x-4">
								<div
									class="w-12 h-12 rounded-full bg-primary-500 text-white flex items-center justify-center font-bold text-lg"
									class:bg-gray-400={isTimeInPast(prayer.time)}
								>
									{translatePrayerName(prayer.name, $currentLanguage).charAt(0)}
								</div>
								<div>
									<h4 class="text-lg font-semibold">{translatePrayerName(prayer.name, $currentLanguage)}</h4>
									<p class="text-sm text-gray-500 dark:text-gray-400">
										{isTimeInPast(prayer.time) ? $t.completed : $t.upcoming}
									</p>
								</div>
							</div>
							<div class="text-right">
								<p class="text-2xl font-bold">{formatTime(prayer.time)}</p>
								{#if !isTimeInPast(prayer.time)}
									<p class="text-sm text-primary-600 dark:text-primary-400">
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
			class="btn-primary"
		>
			<svg class="w-5 h-5 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2" />
			</svg>
			{$t.muteNow}
		</button>
		<button
			onclick={() => apiClient.unmuteAudioNow()}
			class="btn-outline"
		>
			<svg class="w-5 h-5 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
			</svg>
			{$t.unmute}
		</button>
	</div>
</div>
