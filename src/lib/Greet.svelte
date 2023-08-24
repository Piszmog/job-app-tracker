<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke('greet', { name });
	}

	async function getJobApplications() {
		const jobApplications = await invoke('get_job_applications');
		console.log(jobApplications);
	}
</script>

<div>
	<form class="row" on:submit|preventDefault={greet}>
		<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
		<button type="submit">Greet</button>
		<button type="button" on:click={getJobApplications}>Get Job Applications</button>
	</form>
	<p>{greetMsg}</p>
</div>

<p class="text-red-600">Foo</p>
