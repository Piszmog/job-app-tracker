<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { JobApplication } from '../lib/utils/types';
	import { createJobApplication } from '../lib/utils/client';

	const dispatch = createEventDispatcher<{
		submit: JobApplication;
		cancel: never;
	}>();

	const handleSubmit = async (event: Event) => {
		const form = event.target as HTMLFormElement;
		const data = new FormData(form);

		const jobApplication = await createJobApplication(
			data.get('company'),
			data.get('title'),
			data.get('url')
		);
		form.reset();
		dispatch('submit', jobApplication);
	};
</script>

<form on:submit|preventDefault={handleSubmit}>
	<div class="grid grid-cols-1 gap-x-6 gap-y-8">
		<div>
			<label for="company" class="block text-sm font-medium leading-6 text-gray-900">Company</label>
			<div class="mt-2">
				<input
					type="text"
					name="company"
					id="company"
					class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6"
					required
				/>
			</div>
		</div>
		<div>
			<label for="title" class="block text-sm font-medium leading-6 text-gray-900">Title</label>
			<div class="mt-2">
				<input
					type="text"
					name="title"
					id="title"
					class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6"
					required
				/>
			</div>
		</div>
		<div>
			<label for="url" class="block text-sm font-medium leading-6 text-gray-900">URL</label>
			<div class="mt-2">
				<div
					class="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-blue-600"
				>
					<span class="flex select-none items-center pl-3 text-gray-500 sm:text-sm">https://</span>
					<input
						type="text"
						name="url"
						id="url"
						class="block w-full border-0 bg-transparent py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
						placeholder="www.example.com"
						required
					/>
				</div>
			</div>
		</div>
	</div>
	<div class="mt-6 flex items-center justify-end gap-x-6">
		<button
			type="button"
			class="text-sm font-semibold leading-6 text-gray-900"
			on:click={() => dispatch('cancel')}>Cancel</button
		>
		<button
			type="submit"
			class="rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
		>
			Add
		</button>
	</div>
</form>
