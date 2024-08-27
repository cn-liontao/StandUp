<script lang="ts">
	import { calendarState } from '$store/calendar'
</script>

<nav class="flex justify-end gap-1 mb-2">
	<button
		class="bg-gray-4 hover:bg-gray-8 i-tabler-chevron-left text-xl"
		on:click={() => calendarState.decrementAnchor()}
	></button>
	<button
		class="bg-gray-4 hover:bg-gray-8 i-tabler-chevron-right text-xl"
		on:click={() => calendarState.incrementAnchor()}
	></button>
</nav>