<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import { appConfig, themeMode } from '$lib/stores';
	import { getTodayDate } from '$lib/utils/time';

	let city = 'Istanbul';
	let country = 'Turkey';
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
			await apiClient.setSetting('city', city);
			await apiClient.setSetting('country', country);
			await apiClient.setSetting('auto_mute', autoMute.toString());
			await apiClient.setSetting('notifications_enabled', notificationsEnabled.toString());
			await apiClient.setSetting('theme_mode', selectedTheme);

			// Update local stores
			$appConfig = {
				city,
				country,
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

			if (!city || !country) {
				saveMessage = 'Lütfen önce şehir ve ülke bilgilerini girin';
				return;
			}

			const today = getTodayDate();
			await apiClient.fetchAndStorePrayerTimes(city, country, today);
			
saveMessage = 'Namaz vakitleri başarıyla alındı!';
			setTimeout(() => (saveMessage = ''), 3000);
		} catch (error) {
			saveMessage = 'Namaz vakitleri alınamadı';
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
	<h1 class="text-4xl font-bold text-gray-800 dark:text-white mb-8">Ayarlar</h1>

	{#if saveMessage}
		<div
			class="mb-6 px-4 py-3 rounded-lg"
			class:bg-green-100={saveMessage.includes('başarıyla')}
			class:text-green-700={saveMessage.includes('başarıyla')}
			class:dark:bg-green-900={saveMessage.includes('başarıyla')}
			class:dark:text-green-200={saveMessage.includes('başarıyla')}
			class:bg-red-100={!saveMessage.includes('başarıyla')}
			class:text-red-700={!saveMessage.includes('başarıyla')}
			class:dark:bg-red-900={!saveMessage.includes('başarıyla')}
			class:dark:text-red-200={!saveMessage.includes('başarıyla')}
		>
			{saveMessage}
		</div>
	{/if}

	<div class="space-y-6">
		<!-- Konum Ayarları -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Konum
			</h2>
			<div class="grid grid-cols-2 gap-4">
				<div>
					<label for="city" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						Şehir
					</label>
					<input
						id="city"
						type="text"
						bind:value={city}
						placeholder="Örn: Istanbul"
						class="input-field"
					/>
				</div>
				<div>
					<label for="country" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						Ülke
					</label>
					<input
						id="country"
						type="text"
						bind:value={country}
						placeholder="Örn: Turkey"
						class="input-field"
					/>
				</div>
			</div>
			<p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
				API anahtarı gerekmez
			</p>
		</div>

		<!-- Ses Davranışı -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Ses Davranışı
			</h2>
			<div class="space-y-4">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={autoMute}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						Namaz vaktinde sistem sesini otomatik kapat
					</span>
				</label>
			</div>
		</div>

		<!-- Bildirimler -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Bildirimler
			</h2>
			<div class="space-y-4">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={notificationsEnabled}
						class="w-5 h-5 text-primary-500 border-gray-300 rounded focus:ring-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">
						Namaz vakitleri için bildirim göster
					</span>
				</label>
			</div>
		</div>

		<!-- Tema -->
		<div class="card">
			<h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
				Tema
			</h2>
			<div class="space-y-2">
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="light"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">Açık</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="dark"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">Koyu</span>
				</label>
				<label class="flex items-center space-x-3 cursor-pointer">
					<input
						type="radio"
						bind:group={selectedTheme}
						value="system"
						class="w-4 h-4 text-primary-500"
					/>
					<span class="text-gray-700 dark:text-gray-300">Sistem</span>
				</label>
			</div>
		</div>

		<!-- İşlem Düğmeleri -->
		<div class="flex space-x-4">
			<button
				onclick={saveSettings}
				disabled={isSaving}
				class="btn-primary flex-1"
			>
				{isSaving ? 'Kaydediliyor...' : 'Ayarları Kaydet'}
			</button>
			<button
				onclick={fetchPrayersNow}
				disabled={isSaving}
				class="btn-outline flex-1"
			>
				Namaz Vakitlerini Şimdi Al
			</button>
		</div>
	</div>
</div>
