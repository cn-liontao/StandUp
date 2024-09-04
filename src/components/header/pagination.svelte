<script lang="ts">
	import { calendarState } from '$store/calendar'
</script>

<nav class="flex justify-end gap-1 mb-2">
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 i-tabler-chevron-left text-xl"		on:click={() => calendarState.decrementAnchor()}
	></button>
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 i-tabler-chevron-left text-xl"		on:click={() => calendarState.incrementAnchor()}
	></button>
</nav>