<script lang="ts">
	import type { JobApplicationStatus } from '../lib/utils/types';
	import StatusBadge from './StatusBadge.svelte';

	export let company: string;
	export let title: string;
	export let status: JobApplicationStatus;
	export let updatedAt: string;
</script>

<div class="min-w-0">
	<div class="flex items-start gap-x-3">
		<p class="text-sm font-semibold leading-6 text-gray-900">{company}</p>
		<StatusBadge {status} />
	</div>
	<div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
		<p class="truncate">{title}</p>
		<svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
			<circle cx="1" cy="1" r="1" />
		</svg>
		<p class="whitespace-nowrap">
			Updated
			<time datetime="2023-03-17T00:00Z">
				{new Date(updatedAt).toDateString()}
			</time>
		</p>
	</div>
</div>
