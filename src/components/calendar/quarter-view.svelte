<script lang="ts">
	import { type DayRecord, dayRecords } from '../../store/day_records';
	import { allDaysOfWeeksBefore } from '$lib/calendar';

	let startDate = new Date();

	$: quarterWeeks = allDaysOfWeeksBefore(startDate, 4 * 3)

	const getRecordByDate = (date: Date) => $dayRecords.find(r => r.date === date.getTime())
	const getActiveHoursByRecord = (dayRecord?: DayRecord) => {
		if (!dayRecord) return 0
		const microseconds = dayRecord.records.reduce((acc, record) => {
			acc += record.duration
			return acc
		}, 0)
		console.log(dayRecord)
		return microseconds / 1000 / 60 / 24
	}
	const getActivityScoreByHour = (hours: number) => {
		if (hours < 0) return 0;
		else if (hours < 0.5) return 2;
		else if (hours < 1) return 3;
		else if (hours < 1.5) return 4;
		else if (hours < 2) return 5;
		else if (hours < 4) return 6;
		else if (hours < 7) return 7;
		else if (hours < 12) return 8;
		else if (hours < 16) return 9;
		// else if (hours < 20) return 9.5;
		else return 9;
	}

	$: dayScores = $dayRecords.reduce<Map<number, number>>((acc, dayRecord) => {
		acc.set(dayRecord.date, getActivityScoreByHour(getActiveHoursByRecord(dayRecord)))
		return acc
	}, new Map())

	const showActivitiesByDate = (date: Date) => () => {
		const dayRecord = getRecordByDate(date)
		if (!dayRecord) {
			console.log('No activities:', date)
			return;
		} else {
			console.log(date, getActiveHoursByRecord(getRecordByDate(date)))
		}
	}
</script>

<section class="flex flex-row-reverse gap-1">
	{#each quarterWeeks as week, weekIndex}
		<div class="flex flex-col gap-1">
			{#each week as day, dayIndex}
				<button
					tabindex={weekIndex * 7 + dayIndex}
					class="outline-none border-none p-0 w-4 h-4 bg-gray-1 rounded-sm cursor-pointer"
					on:click={showActivitiesByDate(day)}
				>
					{#if dayScores.has(day.getTime())}
						<div class="w-full h-full rounded-sm { `bg-green-${dayScores.get(day.getTime())}00` }" />
					{/if}
				</button>
			{/each}
		</div>
	{/each}
</section>