<script lang="ts">
	import { JobApplicationStatus } from '../lib/utils/types';
	import { fromLowercaseToCapitalized } from '../lib/utils/word.js';

	export let status: JobApplicationStatus;
</script>

<p
	class:accepted={status === 'accepted'}
	class:applied={status === 'applied'}
	class:cancelled={status === 'cancelled'}
	class:declined={status === 'declined'}
	class:interviewing={status === 'interviewing'}
	class:offered={status === 'offered'}
	class:rejected={status === 'rejected'}
	class:watching={status === 'watching'}
	class:withdrawn={status === 'withdrawn'}
	class="mt-0.5 whitespace-nowrap rounded-md px-1.5 py-0.5 text-xs font-medium ring-1 ring-inset"
>
	{fromLowercaseToCapitalized(status)}
</p>

<style lang="postcss">
	.accepted {
		@apply bg-green-50 text-green-700 ring-green-600/20;
	}

	.applied {
		@apply bg-blue-50 text-blue-700 ring-blue-600/20;
	}

	.cancelled {
		@apply bg-gray-50 text-gray-700 ring-gray-600/20;
	}

	.declined {
		@apply bg-red-50 text-red-700 ring-red-600/20;
	}

	.interviewing {
		@apply bg-yellow-50 text-yellow-700 ring-yellow-600/20;
	}

	.offered {
		@apply bg-purple-50 text-purple-700 ring-purple-600/20;
	}

	.rejected {
		@apply bg-red-50 text-red-700 ring-red-600/20;
	}

	.watching {
		@apply bg-gray-50 text-gray-700 ring-gray-600/20;
	}

	.withdrawn {
		@apply bg-gray-50 text-gray-700 ring-gray-600/20;
	}
</style>
