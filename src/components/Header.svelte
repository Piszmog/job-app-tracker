<script lang="ts">
	import { exportData, importData } from '../lib/utils/fs';

	export let title: string;
	export let open = false;
</script>

<div class="border-b border-gray-200 bg-white px-4 py-5 sm:px-6">
	<div class="-ml-4 -mt-2 flex flex-wrap items-center justify-between sm:flex-nowrap">
		<div class="ml-4 mt-2">
			<h3 class="text-base font-semibold leading-6 text-gray-900">{title}</h3>
		</div>
		<div class="ml-4 mt-2 flex-shrink-0">
			<button
				type="button"
				class="relative inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-gray-300 hover:bg-gray-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-100"
				on:click={importData}
			>
				Import
			</button>
			<button
				type="button"
				class="relative inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-gray-300 hover:bg-gray-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-100"
				on:click={exportData}
			>
				Export
			</button>
			<button
				type="button"
				class="relative inline-flex items-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
				on:click={() => (open = true)}
			>
				Add
			</button>
		</div>
	</div>
</div>
