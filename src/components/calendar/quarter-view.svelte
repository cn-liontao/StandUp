<script lang="ts">
	import { calendarState, dayRecordsInView, daysInView } from '$store/calendar';
	import {
		getActiveHoursByRecord,
		getActivityScoreByHour,
	} from '$lib/record-calc';
	import { formatMonth, isFirstWeekOfMonth } from '$lib/calendar';

	$: dayScores = $dayRecordsInView.reduce<Map<number, number>>((acc, dayRecord) => {
		acc.set(dayRecord.date, getActivityScoreByHour(getActiveHoursByRecord(dayRecord)));
		return acc;
	}, new Map());
</script>

{#key $calendarState.anchor}
	<section
		class="flex flex-row-reverse gap-1"
	>
		{#each $daysInView as week, weekIndex}
			<div class="flex flex-col gap-1 relative">
				{#each week as day, dayIndex}
					<button
						tabindex={weekIndex * 7 + dayIndex}
						class="outline-none border-none p-0 w-4 h-4 bg-gray-1 rounded-sm cursor-pointer"
					>
						{#if dayScores.has(day.getTime())}
							<div class="w-full h-full rounded-sm {`bg-green-${dayScores.get(day.getTime())}00`}" />
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
