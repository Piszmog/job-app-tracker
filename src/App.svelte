<script lang="ts">
	import { onMount } from 'svelte';
	import { getJobApplications } from './lib/utils/client';
	import type { JobApplication } from './lib/utils/types';
	import Drawer from './components/Drawer.svelte';
	import JobApplicationForm from './components/JobForm.svelte';
	import Jobs from './components/Jobs.svelte';
	import Header from './components/Header.svelte';
	import Stats from './components/Stats.svelte';

	let jobApplications: Promise<JobApplication[]>;
	let jobs: JobApplication[] = [];
	onMount(() => {
		jobApplications = getJobApplications().then((res: JobApplication[]) => (jobs = res));
	});

	let open = false;

	const handleAddJob = (e: CustomEvent<JobApplication>) => {
		jobs = [e.detail, ...jobs];
		open = false;
	};
</script>

<main>
	<Header title="Job Applications" bind:open />
	{#await jobApplications}
		<p class="text-sm font-semibold leading-6 text-gray-900">Loading...</p>
	{:then data}
		<Stats jobsApplications={jobs} />
		<Jobs bind:jobs />
	{:catch error}
		<p class="text-sm font-semibold leading-6 text-gray-900">Error: {error.message}</p>
	{/await}

	<Drawer title="Add Job Application" bind:open>
		<JobApplicationForm on:submit={handleAddJob} on:cancel={() => (open = false)} />
	</Drawer>
</main>

<style>
</style>
