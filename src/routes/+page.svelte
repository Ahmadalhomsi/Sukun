<script lang="ts">
	import '../app.css';
	import { currentView } from '$lib/stores';
	import Home from '$lib/components/Home.svelte';
	import Settings from '$lib/components/Settings.svelte';
	import Logs from '$lib/components/Logs.svelte';

	let view = $state<'home' | 'settings' | 'logs'>('home');

	// Subscribe to store
	$effect(() => {
		const unsubscribe = currentView.subscribe((v) => {
			view = v;
		});
		return unsubscribe;
	});

	function navigate(page: 'home' | 'settings' | 'logs') {
		view = page;
		currentView.set(page);
	}
</script>

<div class="flex h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Sidebar Navigation -->
	<aside class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
		<div class="p-6">
			<div class="flex items-center space-x-2 mb-8">
				<div class="w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-600 rounded-lg flex items-center justify-center">
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
				</div>
				<div>
					<h2 class="text-xl font-bold text-gray-800 dark:text-white">Sukun</h2>
					<p class="text-xs text-gray-500 dark:text-gray-400">Prayer Times</p>
				</div>
			</div>

			<nav class="space-y-2">
				<button
					onclick={() => navigate('home')}
					class="w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200"
					class:bg-primary-500={view === 'home'}
					class:text-white={view === 'home'}
					class:hover:bg-primary-600={view === 'home'}
					class:text-gray-700={view !== 'home'}
					class:dark:text-gray-300={view !== 'home'}
					class:hover:bg-gray-100={view !== 'home'}
					class:dark:hover:bg-gray-700={view !== 'home'}
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
					</svg>
					<span class="font-medium">Home</span>
				</button>

				<button
					onclick={() => navigate('settings')}
					class="w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200"
					class:bg-primary-500={view === 'settings'}
					class:text-white={view === 'settings'}
					class:hover:bg-primary-600={view === 'settings'}
					class:text-gray-700={view !== 'settings'}
					class:dark:text-gray-300={view !== 'settings'}
					class:hover:bg-gray-100={view !== 'settings'}
					class:dark:hover:bg-gray-700={view !== 'settings'}
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
					</svg>
					<span class="font-medium">Settings</span>
				</button>

				<button
					onclick={() => navigate('logs')}
					class="w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200"
					class:bg-primary-500={view === 'logs'}
					class:text-white={view === 'logs'}
					class:hover:bg-primary-600={view === 'logs'}
					class:text-gray-700={view !== 'logs'}
					class:dark:text-gray-300={view !== 'logs'}
					class:hover:bg-gray-100={view !== 'logs'}
					class:dark:hover:bg-gray-700={view !== 'logs'}
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
					</svg>
					<span class="font-medium">Logs</span>
				</button>
			</nav>
		</div>

		<!-- Footer -->
		<div class="mt-auto p-6 border-t border-gray-200 dark:border-gray-700">
			<p class="text-xs text-gray-500 dark:text-gray-400 text-center">
				v1.0.0
			</p>
		</div>
	</aside>

	<!-- Main Content -->
	<main class="flex-1 overflow-y-auto">
		{#if view === 'home'}
			<Home />
		{:else if view === 'settings'}
			<Settings />
		{:else if view === 'logs'}
			<Logs />
		{/if}
	</main>
</div>
