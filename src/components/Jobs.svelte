<script lang="ts">
	import Job from './Job.svelte';
	import type { JobApplication } from '../lib/utils/types';
	import Drawer from './Drawer.svelte';
	import JobDetails from './JobDetails.svelte';

	export let jobs: JobApplication[] = [];

	let open = false;
	let selectedJob: JobApplication;
	const handleView = (job: JobApplication) => {
		selectedJob = job;
		open = true;
	};

	const handleSubmit = (e: CustomEvent<JobApplication>) => {
		const { detail } = e;
		const index = jobs.findIndex((job) => job.id === detail.id);
		jobs[index] = detail;
		open = false;
	};
</script>

{#if jobs}
	<ul role="list" class="divide-y divide-gray-100 px-4 py-5 sm:px-6">
		{#each jobs as job (job.id)}
			<li class="flex items-center justify-between gap-x-6 py-5">
				<Job
					company={job.company}
					title={job.title}
					status={job.status}
					updatedAt={job.updatedAt}
				/>
				<div class="flex flex-none items-center gap-x-4">
					<button
						class="hidden rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:block"
						on:click={() => handleView(job)}
					>
						View job
					</button>
				</div>
			</li>
		{/each}
	</ul>
{/if}

<Drawer title="Job Application" bind:open>
	<JobDetails on:submit={handleSubmit} on:cancel={() => (open = false)} {...selectedJob} />
</Drawer>
