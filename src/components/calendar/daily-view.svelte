<script context="module" lang="ts">
	import { getContext, setContext } from 'svelte';
	import { type Writable, writable } from 'svelte/store';
	import { fade, fly } from 'svelte/transition';
	import type { DayRecord } from '$store/day-records';

	export const provideDailyView = () => {
		const dailyViewDate = writable<Date>()

		setContext('daily-view', dailyViewDate)

		return dailyViewDate;
	}

	export const useDailyView: () => Writable<Date> = () => {
		return getContext('daily-view');
	}
</script>

<script lang="ts">
	import { dayRecords, type StandingRecord } from '$store/day-records';
	import { getActiveHoursByRecord, getActivityScoreByHour, getRecordByDate } from '$lib/record-calc';
	import { useTooltip } from '$components/tooltip/index.svelte';
	import { format } from 'date-fns';
	import { zhCN } from 'date-fns/locale';
	import { lightFormat, startOfDay } from 'date-fns/fp';

	export let dailyViewDate: Date | undefined;
	const empty = { date: 0, records: [] }

	$: dayRecord = dailyViewDate ? getRecordByDate($dayRecords, dailyViewDate) ?? empty : empty

	const tooltipContent = useTooltip();
	const updateTooltip = (item: ProgressItem) => {
		const record = dayRecord.records[item.index]
		const formatTime = lightFormat('HH:mm:ss');
		tooltipContent.set(`${formatTime(record.start_time)}~${formatTime(record.end_time)}`)
	}
	const hideDailyView = () => {
		tooltipContent.set('')
		dailyViewDate = undefined
	}

	$: todayText = dayRecord ? format(dayRecord.date, "M月d日 eee", { locale: zhCN }) : ''

	/**
	 * 计算每段站立时间在当日的百分比区间
	 * @param record
	 */
	type ProgressItem = { start: number, end: number, index: number }
	const normalizeDayProgress = (record: DayRecord | undefined) => {
		const am: Array<ProgressItem> = []
		const pm: Array<ProgressItem> = []

		if (!record) return { am, pm }

		const todayStart = startOfDay(record.date).getTime()
		const todayNoon = startOfDay(record.date).setHours(12, 0, 0, 0)

		const halfDay = todayNoon - todayStart
		record.records.forEach((record: StandingRecord, index: number) => {
			if (record.end_time <= todayNoon) {
				const start = Math.max(0, (record.start_time - todayStart) / halfDay)
				const end = (record.end_time - todayStart) / halfDay
				am.push({ start, end, index })
			} else if (record.start_time >= todayNoon) {
				const start = (record.start_time - todayNoon) / halfDay
				const end = Math.min(1, (record.end_time - todayNoon) / halfDay)
				pm.push({ start, end, index })
			} else {
				const start = Math.max(0, (record.start_time - todayStart) / halfDay)
				const end = Math.min(1, (record.end_time - todayNoon) / halfDay)
				am.push({ start, end: 1, index })
				pm.push({ start: 0, end, index })
			}
		})

		return {
			am, pm
		}
	}

	$: recordsProgress = normalizeDayProgress(dayRecord)
	$: console.log(recordsProgress)
	$: dayScore = getActivityScoreByHour(getActiveHoursByRecord(dayRecord))

</script>

{#if dayRecord.records.length > 0}
	<div
		class="bg-[#00000080] fixed top-0 left-0 w-screen h-screen outline-none"
		role="button"
		tabindex="-1"
		transition:fade={{ duration: 100 }}
		on:keyup={() => {}}
		on:click={hideDailyView}
	>
		<div
			role="button"
			tabindex="-1"
			class="fixed bottom-0 bg-white w-full h-[50vh] rounded-t-md pt-3 px-4 box-border flex flex-col gap-2"
			transition:fly={{ y: 100, duration: 300 }}
			on:keyup={() => {}}
			on:click|stopPropagation
		>
			<div class="flex justify-between">
				<h3 class="text-sm m-0">{todayText}</h3>
				<button 
					class="hover:bg-gray-8 bg-gray-5 w-5 h-4 rounded-sm i-tabler:arrow-back-up"
					on:click={hideDailyView}
				/>
			</div>
			<div class="w-full h-6 shadow-md bg-gray-1 rounded-md relative">
				{#each recordsProgress.am as item}
					<div
						class="h-6 shadow-md absolute {`bg-green-${dayScore}00`}"
						class:rounded-l-md={item.start === 0}
						class:rounded-r-md={item.end === 1}
						style="left: {item.start * 100}%; width: {(item.end - item.start) * 100}%"
					/>
				{/each}
			</div>
			<div class="w-full h-6 shadow-md bg-gray-1 relative">
				{#each recordsProgress.pm as item}
					<div
						role="button"
						tabindex="-1"
						class="h-6 shadow-md absolute {`bg-green-${dayScore}00`}"
						class:rounded-l-md={item.start === 0}
						class:rounded-r-md={item.end === 1}
						style="left: {item.start * 100}%; width: {(item.end - item.start) * 100}%"
						on:focus={() => updateTooltip(item)}
						on:mouseover={() => updateTooltip(item)}
					/>
				{/each}
			</div>
		</div>
	</div>
{/if}