<script lang="ts">
	import { calendarState, dayRecordsInView, daysInView } from '$store/calendar';
	import {
		formatDayRecord,
		getActiveHoursByRecord,
		getActivityScoreByHour,
		getRecordByDate
	} from '$lib/record-calc';
	import { formatMonth, isFirstWeekOfMonth } from '$lib/calendar';
	import { useTooltip } from '$components/tooltip/index.svelte';
	import { useDailyView } from './daily-view.svelte';

	$: dayScores = $dayRecordsInView.reduce<Map<number, number>>((acc, dayRecord) => {
		acc.set(dayRecord.date, getActivityScoreByHour(getActiveHoursByRecord(dayRecord)));
		return acc;
	}, new Map());

	const tooltipContent = useTooltip();

	const updateTooltip = (e: Event) => {
		const index = ((e.target as HTMLElement).dataset as { index: string }).index;
		const day = $daysInView[Math.floor(Number(index) / 7)][Number(index) % 7];
		if (dayScores.has(day.getTime())) {
			const dayRecord = getRecordByDate($dayRecordsInView, day)
			if (dayRecord) tooltipContent.set(formatDayRecord(dayRecord))
		} else {
			tooltipContent.set('')
		}
	}

	const dailyViewDate = useDailyView();
	const toggleDailyView = (day: Date) => {
		dailyViewDate.set(day)
		tooltipContent.set('')
	}
</script>

{#key $calendarState.anchor}
	<section
		class="flex flex-row-reverse gap-1 select-none"
	>
		{#each $daysInView as week, weekIndex}
			<div class="flex flex-col gap-1 relative">
				{#each week as day, dayIndex}
					<button
						on:click={() => toogleDailyView(day)}
						on:focus={updateTooltip}
						on:mouseover={updateTooltip}
						tabindex={weekIndex * 7 + dayIndex}
						data-index={weekIndex * 7 + dayIndex}
						class="outline-none border-none p-0 w-4 h-4 bg-gray-1 rounded-sm cursor-pointer"
					>
						{#if dayScores.has(day.getTime())}
							<div
								data-index={weekIndex * 7 + dayIndex}
								class="w-full h-full rounded-sm {`bg-green-${dayScores.get(day.getTime())}00`}"
							/>
						{/if}
					</button>
				{/each}
				{#if isFirstWeekOfMonth(week[0])}
					<div
						class="pointer-events-none text-stone-600 text-xs font-medium whitespace-nowrap absolute bottom-[-1.2rem]"
						class:left-[-0.25rem]={ 0 < weekIndex && weekIndex < 11 }
					>
						{formatMonth(week[0])}
					</div>
				{/if}
			</div>
		{/each}
	</section>
{/key}
