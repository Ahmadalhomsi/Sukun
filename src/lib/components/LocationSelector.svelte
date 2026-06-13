<script lang="ts">
	import { countries, type City } from '$lib/data/locations';
	import { t } from '$lib/i18n';
	
	export let show = false;
	export let selectedCity = '';
	export let selectedCountry = '';
	export let onSelect: (city: string, country: string) => void;
	
	let selectedCountryData = countries.find(c => c.name === selectedCountry) || countries[0];
	let searchCity = '';
	
	$: filteredCities = selectedCountryData.cities.filter(city =>
		city.name.toLowerCase().includes(searchCity.toLowerCase())
	);
	
	function handleCountryChange(countryName: string) {
		selectedCountryData = countries.find(c => c.name === countryName) || countries[0];
		searchCity = '';
	}
	
	function selectLocation() {
		if (selectedCity && selectedCountryData.name) {
			onSelect(selectedCity, selectedCountryData.name);
			show = false;
		}
	}
	
	function handleCityClick(cityName: string) {
		selectedCity = cityName;
		selectLocation();
	}
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4"
		role="button"
		tabindex="0"
		onclick={() => (show = false)}
		onkeydown={(e) => {
			if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') {
				show = false;
			}
		}}
	>
		<div
			class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md max-h-[80vh] flex flex-col"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div class="p-6 border-b border-gray-200 dark:border-gray-700">
				<div class="flex items-center justify-between">
					<h2 class="text-2xl font-bold text-gray-800 dark:text-white">
						{$t.selectLocation}
					</h2>
					<button
						onclick={() => (show = false)}
						class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
						aria-label="Close dialog"
					>
						<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>
			</div>
			
			<!-- Body -->
			<div class="p-6 flex-1 overflow-y-auto">
				<!-- Country Selection -->
				<div class="mb-4">
					<label for="country-select" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						{$t.country}
					</label>
					<select
						id="country-select"
						bind:value={selectedCountryData.name}
						onchange={(e) => handleCountryChange(e.currentTarget.value)}
						class="input-field"
					>
						{#each countries as country}
							<option value={country.name}>{country.name}</option>
						{/each}
					</select>
				</div>
				
				<!-- City Search -->
				<div class="mb-4">
					<label for="city-search" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						{$t.city}
					</label>
					<input
						id="city-search"
						type="text"
						bind:value={searchCity}
						placeholder="Search cities..."
						class="input-field"
					/>
				</div>
				
				<!-- Cities List -->
				<div class="grid grid-cols-2 gap-2 max-h-60 overflow-y-auto">
					{#each filteredCities as city}
						<button
							onclick={() => handleCityClick(city.name)}
							class="p-3 text-left rounded-lg border transition-all duration-150 {selectedCity === city.name ? 'border-primary-500 bg-primary-50 dark:bg-gray-700 text-primary-700 dark:text-primary-300' : 'border-gray-200 dark:border-gray-700 hover:border-primary-400 hover:bg-primary-50 dark:hover:bg-gray-700'}"
						>
							<span class="text-sm font-medium text-gray-700 dark:text-gray-300">
								{city.name}
							</span>
						</button>
					{/each}
				</div>
			</div>
		</div>
	</div>
{/if}
