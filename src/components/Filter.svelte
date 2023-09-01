<script>
	import Select from './Select.svelte';
	import { JobApplicationStatus } from '../lib/utils/types';

	export let company = '';
	export let status = '';

	const statusOptions = Object.entries(JobApplicationStatus).map(([label, value]) => ({
		label,
		value
	}));
	statusOptions.push({ value: '', label: 'All' });
</script>

<div class="ml-3 mr-3 mt-3 flex items-center space-x-2">
	<div class="w-full">
		<label for="company" class="block text-sm font-medium leading-6 text-gray-900">Company</label>
		<div class="mt-2">
			<input
				type="text"
				name="company"
				id="company"
				class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-gray-600 sm:text-sm sm:leading-6"
				placeholder="Awesome Company"
				bind:value={company}
			/>
		</div>
	</div>
	<Select name="status" label="Status" options={statusOptions} bind:value={status} />
</div>
