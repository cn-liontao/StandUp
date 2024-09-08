<script lang="ts">
	import { calendarState } from '$store/calendar'
	import { standing } from '$store/day-records';
	import { invoke } from '@tauri-apps/api/tauri';

	const stand_or_sit = () => {
		invoke("stand_or_sit")
	}
</script>

<nav class="flex justify-end gap-1 mb-2">
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 i-tabler-chevron-left text-xl"
		on:click={() => calendarState.decrementAnchor()}
	/>
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 text-xl"
		class:i-tabler-user-screen={$standing}
		class:i-tabler-armchair2={!$standing}
		on:click={() => stand_or_sit()}
	/>
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 i-tabler-chevron-right text-xl"
		on:click={() => calendarState.incrementAnchor()}
	/>
</nav>