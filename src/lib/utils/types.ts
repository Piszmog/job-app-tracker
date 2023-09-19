export interface Record {
	id: number;
	createdAt: string;
	updatedAt: string;
}

export interface JobApplication extends Record {
	company: string;
	title: string;
	url: string;
	status: JobApplicationStatus;
	appliedAt: string;
	notes?: JobApplicationNote[];
	statuses?: JobApplicationStatusHistory[];
}

export interface JobApplicationNote extends Record {
	jobApplicationId: number;
	note: string;
}

export interface JobApplicationStatusHistory extends Record {
	jobApplicationId: number;
	status: JobApplicationStatus;
}

export enum JobApplicationStatus {
	Accepted = 'accepted',
	Applied = 'applied',
	Cancelled = 'cancelled',
	Closed = 'closed',
	Declined = 'declined',
	Interviewing = 'interviewing',
	Offered = 'offered',
	Rejected = 'rejected',
	Watching = 'watching',
	Withdrawn = 'withdrawn'
}

export const statusOptions = Object.entries(JobApplicationStatus).map(([label, value]) => ({
	label,
	value
}));

export type Option = {
	label: string;
	value: string;
};
