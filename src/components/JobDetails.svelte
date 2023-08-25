<script lang="ts">
	import type { JobApplication } from '../lib/utils/types';
	import { JobApplicationStatus } from '../lib/utils/types';
	import { createEventDispatcher } from 'svelte';
	import Select from './Select.svelte';
	import { statusOptions } from '../lib/utils/types';
	import { updateJobApplicationStatus } from '../lib/utils/client';

	export let id: number;
	export let updatedAt: string;
	export let appliedAt: string;
	export let company: string;
	export let title: string;
	export let url: string;
	export let status: JobApplicationStatus;

	const dispatch = createEventDispatcher<{
		cancel: never;
		submit: JobApplication;
	}>();

	const handleSubmit = async (event: Event) => {
		const form = event.target as HTMLFormElement;
		const data = new FormData(form);

		const jobApplication = await updateJobApplicationStatus(
			id,
			data.get('status') as JobApplicationStatus
		);
		dispatch('submit', jobApplication);
	};
</script>

<form on:submit|preventDefault={handleSubmit}>
	<div class="mb-5 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
		<p class="whitespace-nowrap">
			Applied
			<time datetime="2023-03-17T00:00Z">
				{new Date(appliedAt).toDateString()}
			</time>
		</p>
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
	<input type="hidden" name="id" value={id} />
	<div class="grid grid-cols-2 gap-x-6 gap-y-8">
		<div>
			<label for="company" class="block text-sm font-medium leading-6 text-gray-900">Company</label>
			<div class="mt-2">
				<input
					type="text"
					name="company"
					id="company"
					class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6"
					readonly
					value={company}
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
					readonly
					value={title}
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
						readonly
						value={url}
					/>
				</div>
			</div>
		</div>
		<div>
			<Select name="status" label="Status" options={statusOptions} required value={status} />
		</div>
	</div>
	<div class="mt-6 flex items-center justify-end gap-x-6">
		<button
			type="button"
			class="text-sm font-semibold leading-6 text-gray-900"
			on:click={() => dispatch('cancel')}
		>
			Cancel
		</button>
		<button
			type="submit"
			class="rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
		>
			Update
		</button>
	</div>
</form>
