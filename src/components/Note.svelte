<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { addJobApplicationNote } from '../lib/utils/client';
	import type { JobApplicationNote } from '../lib/utils/types';

	export let id: number;

	const dispatch = createEventDispatcher<{
		submit: JobApplicationNote;
	}>();

	const handleSubmit = async (event: Event) => {
		const form = event.target as HTMLFormElement;
		const data = new FormData(form);

		const note = await addJobApplicationNote(id, data.get('note'));
		form.reset();
		dispatch('submit', note);
	};
</script>

<form on:submit|preventDefault={handleSubmit} class="relative flex-auto">
	<div
		class="overflow-hidden rounded-lg pb-12 shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-blue-600"
	>
		<label for="note" class="sr-only">Add a note</label>
		<textarea
			rows="2"
			name="note"
			id="note"
			class="block w-full resize-none border-0 bg-transparent py-1.5 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
			placeholder="Add a note..."
		></textarea>
	</div>
	<div class="absolute inset-x-0 bottom-0 flex justify-between py-2 pl-3 pr-2">
		<button
			type="submit"
			class="rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
		>
			Add
		</button>
	</div>
</form>
