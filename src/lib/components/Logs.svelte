<script lang="ts">
	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api/client';
	import type { PrayerLog } from '$lib/api/schema';
	import { formatTime } from '$lib/utils/time';
	import { isLoadingLogs } from '$lib/stores';
	import { t, currentLanguage, translatePrayerName } from '$lib/i18n';

	let logs: PrayerLog[] = [];
	let displayLimit = 50;

	async function loadLogs() {
		try {
			$isLoadingLogs = true;
			logs = await apiClient.getRecentLogs(displayLimit);
		} catch (error) {
			console.error('Error loading logs:', error);
		} finally {
			$isLoadingLogs = false;
		}
	}

	function formatDateTime(dateTime: string): string {
		try {
			const date = new Date(dateTime);
			return date.toLocaleString('en-US', {
				month: 'short',
				day: 'numeric',
				hour: '2-digit',
				minute: '2-digit'
			});
		} catch {
			return dateTime;
		}
	}

	onMount(() => {
		loadLogs();
	});
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-4xl font-bold text-gray-800 dark:text-white mb-2">{$t.prayerLogs}</h1>
			<p class="text-gray-600 dark:text-gray-400">
				{$t.historyOfActions}
			</p>
		</div>
		<button
			onclick={loadLogs}
			class="btn-secondary"
			disabled={$isLoadingLogs}
		>
			<svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
			</svg>
			{$t.refresh}
		</button>
	</div>

	{#if $isLoadingLogs}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-4 border-primary-500 border-t-transparent"></div>
		</div>
	{:else if logs.length === 0}
		<div class="card text-center py-12">
			<svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
			</svg>
			<p class="text-gray-600 dark:text-gray-400 text-lg">{$t.noLogs}</p>
			<p class="text-sm text-gray-500 dark:text-gray-500 mt-2">
				{$t.historyOfActions}
			</p>
		</div>
	{:else}
		<div class="card">
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr class="border-b border-gray-200 dark:border-gray-700">
							<th class="text-left py-3 px-4 font-semibold text-gray-700 dark:text-gray-300">
								Prayer
							</th>
							<th class="text-left py-3 px-4 font-semibold text-gray-700 dark:text-gray-300">
								Scheduled Time
							</th>
							<th class="text-left py-3 px-4 font-semibold text-gray-700 dark:text-gray-300">
								Executed At
							</th>
							<th class="text-left py-3 px-4 font-semibold text-gray-700 dark:text-gray-300">
								Action
							</th>
							<th class="text-center py-3 px-4 font-semibold text-gray-700 dark:text-gray-300">
								Status
							</th>
						</tr>
					</thead>
					<tbody>
						{#each logs as log (log.id)}
							<tr class="border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
								<td class="py-3 px-4">
									<div class="flex items-center space-x-3">
										<div class="w-10 h-10 rounded-full bg-primary-500 text-white flex items-center justify-center font-bold">
											{log.prayer_name.charAt(0)}
										</div>
										<span class="font-medium text-gray-800 dark:text-gray-200">
											{log.prayer_name}
										</span>
									</div>
								</td>
								<td class="py-3 px-4 text-gray-600 dark:text-gray-400">
									{formatTime(log.scheduled_time)}
								</td>
								<td class="py-3 px-4 text-gray-600 dark:text-gray-400">
									{formatDateTime(log.executed_at)}
								</td>
								<td class="py-3 px-4">
									<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
										{log.action.replace('_', ' ')}
									</span>
								</td>
								<td class="py-3 px-4 text-center">
									{#if log.success}
										<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
											<svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
												<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
											</svg>
											Success
										</span>
									{:else}
										<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200">
											<svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
												<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
											</svg>
											Failed
										</span>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			{#if logs.length >= displayLimit}
				<div class="mt-4 text-center">
					<button
						onclick={() => {
							displayLimit += 50;
							loadLogs();
						}}
						class="btn-secondary text-sm"
					>
						Load More
					</button>
				</div>
			{/if}
		</div>

		<!-- Summary Stats -->
		<div class="grid grid-cols-3 gap-4 mt-6">
			<div class="card text-center">
				<p class="text-3xl font-bold text-primary-500">{logs.length}</p>
				<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Total Logs</p>
			</div>
			<div class="card text-center">
				<p class="text-3xl font-bold text-green-500">
					{logs.filter((l) => l.success).length}
				</p>
				<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Successful</p>
			</div>
			<div class="card text-center">
				<p class="text-3xl font-bold text-red-500">
					{logs.filter((l) => !l.success).length}
				</p>
				<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Failed</p>
			</div>
		</div>
	{/if}
</div>
