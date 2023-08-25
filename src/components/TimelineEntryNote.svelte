<script lang="ts">
	export let note: string;
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
						class="icon icon-tabler icon-tabler-message"
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
						<path d="M8 9h8"></path>
						<path d="M8 13h6"></path>
						<path
							d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12z"
						></path>
					</svg>
				</div>
			</div>
		</div>
		<div class="min-w-0 flex-1">
			<div>
				<p class="mt-0.5 text-sm text-gray-500">
					Note add
					<span class="whitespace-nowrap">{new Date(createdAt).toDateString()}</span>
				</p>
			</div>
			<div class="mt-2 text-sm text-gray-700">
				<p>
					{note}
				</p>
			</div>
		</div>
	</div>
</div>
