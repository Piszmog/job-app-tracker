<script lang="ts">
	import type { JobApplication } from '../lib/utils/types';
	import { JobApplicationStatus } from '../lib/utils/types';
	import { calculateAgeInDays } from '../lib/utils/date';

	export let jobsApplications: JobApplication[];

	let averageTime = 0;
	let rejectionPercentage = 0;
	$: {
		let totalDays = 0;
		let totalApplications = 0;
		let totalRejections = 0;
		jobsApplications.forEach((jobApplication) => {
			totalDays += calculateAgeInDays(new Date(jobApplication.appliedAt));
			if (jobApplication.status === JobApplicationStatus.Rejected) {
				totalRejections++;
			}
			totalApplications++;
		});
		if (totalApplications > 0) {
			averageTime = Math.round(totalDays / totalApplications);
			rejectionPercentage = Math.round((totalRejections / totalApplications) * 100);
		}
	}
</script>

<div class="m-3 border-b">
	<dl class="mx-auto grid grid-cols-1 gap-px bg-gray-900/5 sm:grid-cols-3 lg:grid-cols-3">
		<div
			class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
		>
			<dt class="text-sm font-medium leading-6 text-gray-500">Total Applications</dt>
			<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
				{jobsApplications.length}
			</dd>
		</div>
		<div
			class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
		>
			<dt class="text-sm font-medium leading-6 text-gray-500">Average time of Application</dt>
			<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
				{averageTime} days
			</dd>
		</div>
		<div
			class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
		>
			<dt class="text-sm font-medium leading-6 text-gray-500">Rejection Rate</dt>
			<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
				{rejectionPercentage}%
			</dd>
		</div>
	</dl>
</div>
