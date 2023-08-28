import { save, open } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { getAllData, importJobs } from './client';

const fileName = 'job-applications.json';

export const exportData = async () => {
	const jobApplications = await getAllData();
	const filePath = await save({
		defaultPath: (await downloadDir()) + '/' + fileName
	});

	await writeTextFile(filePath!, JSON.stringify(jobApplications));
};

export const importData = async () => {
	const selectedFilePath = await open({
		directory: false,
		multiple: false,
		filters: [
			{
				name: 'JSON',
				extensions: ['json']
			}
		],
		defaultPath: await downloadDir()
	});

	const content = await readTextFile(selectedFilePath! as string);
	const parsedJSON = JSON.parse(content);
	await importJobs(parsedJSON);
};
