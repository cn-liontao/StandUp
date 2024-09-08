<script lang="ts">
	import { calendarState } from '$store/calendar'
	import { standing } from '$store/day-records';
	import { invoke } from '@tauri-apps/api/tauri';
	import { useTooltip } from '$components/tooltip/index.svelte';
	import { formatDuration, intervalToDuration, startOfToday } from 'date-fns';

	const tooltipContent = useTooltip()
	const stand_or_sit = () => {
		invoke("stand_or_sit")
		tooltipContent.set('')
	}

	const showStandingStatus = () => {
		if (!$standing) {
			tooltipContent.set('点击可以开始站立')
			return
		}
		const duration = intervalToDuration({ start: $standing, end: Date.now() })
		tooltipContent.set(`已经站了 ${duration.hours || 0}:${duration.minutes || 0}:${duration.seconds || 0}`)
	}
</script>

<nav class="flex justify-end gap-1 mb-2">
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 i-tabler-chevron-left text-xl"
		on:click={() => calendarState.decrementAnchor()}
	/>
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 text-xl"
		class:i-tabler-user-screen={!$standing}
		class:i-tabler-armchair2={$standing}
		on:focus={showStandingStatus}
		on:mouseenter={showStandingStatus}
		on:mouseleave={() => tooltipContent.set('')}
		on:click={stand_or_sit}
	/>
	<button
		class="bg-gray-4 dark:bg-gray-500 hover:bg-gray-800 hover:dark:bg-gray-100 i-tabler-chevron-right text-xl"
		on:click={() => calendarState.incrementAnchor()}
	/>
</nav>