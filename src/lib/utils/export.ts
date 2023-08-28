import { save } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';
import { writeTextFile } from '@tauri-apps/api/fs';
import { getAllData } from './client';

const fileName = 'job-applications.json';

export const exportData = async () => {
	const jobApplications = await getAllData();
	const filePath = await save({
		defaultPath: (await downloadDir()) + '/' + fileName
	});
	console.log(filePath);

	await writeTextFile(filePath!, JSON.stringify(jobApplications));
};
