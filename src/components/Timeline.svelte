<script lang="ts">
	import type { JobApplicationNote, JobApplicationStatusHistory } from '../lib/utils/types';
	import TimelineEntryStatus from './TimelineEntryStatus.svelte';
	import TimelineEntryNote from './TimelineEntryNote.svelte';
	import Note from './Note.svelte';

	export let id: number;
	export let histories: JobApplicationStatusHistory[] = [];
	export let notes: JobApplicationNote[] = [];

	let data = [...histories, ...notes];
	$: {
		data = [...histories, ...notes];
		data.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
	}

	const handleSubmit = (e: CustomEvent<JobApplicationNote>) => {
		data = [e.detail, ...data];
	};
</script>

<div class="flow-root">
	<ul role="list" class="-mb-8">
		{#each data as d, index (d.id)}
			{#if d.status !== undefined}
				<TimelineEntryStatus
					status={d.status}
					createdAt={d.createdAt}
					isLast={index === data.length - 1}
				/>
			{:else if d.note !== undefined}
				<TimelineEntryNote
					note={d.note}
					createdAt={d.createdAt}
					isLast={index === data.length - 1}
				/>
			{/if}
		{/each}
	</ul>
</div>
<div class="mt-6 flex gap-x-3">
	<Note on:submit={handleSubmit} {id} />
</div>
