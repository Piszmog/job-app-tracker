<script lang="ts">
	import type { JobApplicationStatus } from '../lib/utils/types';
	import StatusBadge from './StatusBadge.svelte';

	export let status: JobApplicationStatus;
	export let createdAt: string;
	export let isLast = false;
</script>

<div class="relative pb-8">
	{#if !isLast}
		<span class="absolute left-5 top-5 -ml-px h-full w-0.5 bg-gray-200" aria-hidden="true"></span>
	{/if}
	<div class="relative flex items-start space-x-3">
		<div>
			<div class="relative px-1">
				<div
					class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 ring-8 ring-white"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="icon icon-tabler icon-tabler-tag"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						stroke-width="2"
						stroke="currentColor"
						fill="none"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
						<circle cx="8.5" cy="8.5" r="1" fill="currentColor"></circle>
						<path
							d="M4 7v3.859c0 .537 .213 1.052 .593 1.432l8.116 8.116a2.025 2.025 0 0 0 2.864 0l4.834 -4.834a2.025 2.025 0 0 0 0 -2.864l-8.117 -8.116a2.025 2.025 0 0 0 -1.431 -.593h-3.859a3 3 0 0 0 -3 3z"
						></path>
					</svg>
				</div>
			</div>
		</div>
		<div class="min-w-0 flex-1 py-0">
			<div class="text-sm leading-8 text-gray-500">
				<span class="mr-0.5"> Status Change </span>
				<span class="mr-0.5 inline-flex">
					<StatusBadge {status} />
				</span>
				<span class="whitespace-nowrap">{new Date(createdAt).toDateString()}</span>
			</div>
		</div>
	</div>
</div>
