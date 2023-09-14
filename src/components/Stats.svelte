<script lang="ts">
	import type { JobApplication } from '../lib/utils/types';
	import { JobApplicationStatus } from '../lib/utils/types';
	import { calculateAgeInDays } from '../lib/utils/date';

	export let jobsApplications: JobApplication[];

	let averageTime = 0;
	let rejectionPercentage = 0;
	let interviewPercentage = 0;
	let companies: string[] = [];
	$: {
		let totalDays = 0;
		let totalHearedBack = 0;
		let totalApplications = 0;
		let totalRejections = 0;
		let totalInterviews = 0;
		jobsApplications.forEach((jobApplication) => {
			if (jobApplication.status !== JobApplicationStatus.Applied) {
				totalDays += calculateAgeInDays(new Date(jobApplication.appliedAt));
				totalHearedBack++;
			}
			if (jobApplication.status === JobApplicationStatus.Rejected) {
				totalRejections++;
			}
			if (jobApplication.status === JobApplicationStatus.Interviewing) {
				totalInterviews++;
			}
			if (!companies.includes(jobApplication.company)) {
				companies = [...companies, jobApplication.company];
			}
			totalApplications++;
		});
		if (totalApplications > 0) {
			averageTime = Math.round(totalDays / totalHearedBack);
			rejectionPercentage = Math.round((totalRejections / totalApplications) * 100);
			interviewPercentage = Math.round((totalInterviews / totalApplications) * 100);
		}
	}
</script>

<div class="m-3 border-b">
	<dl class="mx-auto grid grid-cols-1 gap-px bg-gray-900/5 sm:grid-cols-5 lg:grid-cols-5">
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
			<dt class="text-sm font-medium leading-6 text-gray-500">Total Companies</dt>
			<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
				{companies.length}
			</dd>
		</div>
		<div
			class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
		>
			<dt class="text-sm font-medium leading-6 text-gray-500">Average time to hear back</dt>
			<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
				{averageTime} days
			</dd>
		</div>
		<div
			class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2 bg-white px-4 py-3 sm:px-6 xl:px-8"
		>
			<dt class="text-sm font-medium leading-6 text-gray-500">Interview Rate</dt>
			<dd class="w-full flex-none text-2xl font-medium leading-10 tracking-tight text-gray-900">
				{interviewPercentage}%
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
