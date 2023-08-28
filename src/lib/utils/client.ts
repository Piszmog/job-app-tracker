import { invoke } from '@tauri-apps/api/tauri';
import type { JobApplication, JobApplicationNote, JobApplicationStatusHistory } from './types';

export const getJobApplications = async (): Promise<JobApplication[]> => {
	return (await invoke('get_job_applications')) as JobApplication[];
};

export const createJobApplication = async (
	company: string,
	title: string,
	url: string
): Promise<JobApplication> => {
	return (await invoke('create_job_application', { company, title, url })) as JobApplication;
};

export const updateJobApplicationStatus = async (
	id: number,
	status: string
): Promise<JobApplicationStatusHistory> => {
	return (await invoke('update_job_application_status', {
		id,
		status
	})) as JobApplicationStatusHistory;
};

export const getJobApplication = async (id: number): Promise<JobApplication> => {
	return (await invoke('get_job_application', { id })) as JobApplication;
};

export const addJobApplicationNote = async (
	id: number,
	note: string
): Promise<JobApplicationNote> => {
	return (await invoke('add_job_application_note', { id, note })) as JobApplicationNote;
};

export const getJobApplicationNotes = async (id: number): Promise<JobApplicationNote[]> => {
	return (await invoke('get_job_application_notes', { id })) as JobApplicationNote[];
};

export const getJobApplicationStatusHistories = async (id: number): Promise<JobApplication[]> => {
	return (await invoke('get_job_application_status_histories', { id })) as JobApplication[];
};

export const getAllData = async (): Promise<JobApplication[]> => {
	return (await invoke('get_all_data')) as JobApplication[];
};
