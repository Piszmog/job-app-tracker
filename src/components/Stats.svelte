<script lang="ts">
	import { getStats } from '../lib/utils/client';

	export let reload = false;

	let statsPromise = getStats();
	$: if (reload) {
		statsPromise = getStats();
	}
</script>

{#await statsPromise}
	<!-- Nothing -->
{:then stats}
	<div class="m-3 border-b">
		<dl class="mx-auto grid grid-cols-1 gap-px bg-gray-900/5 sm:grid-cols-5 lg:grid-cols-5">
			<div
				class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
			>
				<dt class="text-sm font-medium leading-6 text-gray-500">Total Applications</dt>
				<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
					{stats.totalApplications}
				</dd>
			</div>
			<div
				class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
			>
				<dt class="text-sm font-medium leading-6 text-gray-500">Total Companies</dt>
				<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
					{stats.totalCompanies}
				</dd>
			</div>
			<div
				class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
			>
				<dt class="text-sm font-medium leading-6 text-gray-500">Average time to hear back</dt>
				<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
					{stats.averageTimeToHearBack} days
				</dd>
			</div>
			<div
				class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
			>
				<dt class="text-sm font-medium leading-6 text-gray-500">Interview Rate</dt>
				<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
					{Math.floor((stats.totalInterviewing / stats.totalApplications) * 100)}%
				</dd>
			</div>
			<div
				class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
			>
				<dt class="text-sm font-medium leading-6 text-gray-500">Rejection Rate</dt>
				<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
					{Math.floor((stats.totalRejections / stats.totalApplications) * 100)}%
				</dd>
			</div>
		</dl>
	</div>
{/await}
