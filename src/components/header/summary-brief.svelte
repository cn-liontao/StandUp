<script lang="ts">
	import { dayRecords } from '$store/day-records';
	import { dayRecordsInView, calendarState } from '$store/calendar';
	import { sumHoursInRecords } from '$lib/record-calc';
	import SummaryItem from './summary-item.svelte';

	$: daysCount = $dayRecords.length;
	$: hoursCount = sumHoursInRecords($dayRecords)
	$: hoursInView = sumHoursInRecords($dayRecordsInView)
	$: viewName = $calendarState.appSettings.calendar_view === 'Month' ? '月度' : '季度'
</script>

<div class="flex items-center gap-4 text-sm">
	<SummaryItem
		num={daysCount}
		text="天"
	/>
	<SummaryItem
		num={hoursCount}
		text="站立"
	/>
	<SummaryItem
		num="{hoursInView}h"
		text="{viewName}总计"
	/>
</div>