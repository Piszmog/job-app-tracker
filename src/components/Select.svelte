<script lang="ts">
	import type { Option } from '../lib/utils/types';
	import { random } from '../lib/utils/random';

	export let label: string;
	export let options: Option[];
	export let placeholder = '';
	export let value = '';
	export let required = false;
	export let name = '';
	export let error = '';

	let className = '';
	export { className as class };

	const randomId = random();
</script>

<div class="w-full">
	<label for={randomId} class="block text-sm font-medium leading-6 text-gray-900">{label}</label>
	<div class="relative mt-2 rounded-md shadow-sm">
		<select
			id={randomId}
			{name}
			autocomplete="country-name"
			class="{className} block w-full rounded-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-300 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-gray-600 sm:text-sm sm:leading-6"
			class:select-error={error}
			bind:value
			on:change
			{required}
			aria-invalid={error ? 'true' : 'false'}
			aria-describedby="{randomId}-error"
		>
			{#if placeholder}
				<option disabled selected value="">{placeholder}</option>
			{/if}
			{#if options}
				{#each options as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			{/if}
		</select>
		{#if error}
			<div class="pointer-events-none absolute inset-y-0 right-5 flex items-center pr-3">
				<svg
					class="h-5 w-5 text-red-500"
					viewBox="0 0 20 20"
					fill="currentColor"
					aria-hidden="true"
				>
					<path
						fill-rule="evenodd"
						d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z"
						clip-rule="evenodd"
					/>
				</svg>
			</div>
		{/if}
	</div>
	{#if error}
		<p class="mt-2 text-sm text-red-600" id="{randomId}-error">{error}</p>
	{/if}
</div>

<style lang="postcss">
	.select-error {
		@apply text-red-900 ring-red-300 placeholder:text-red-300 focus:ring-red-500;
	}
</style>
