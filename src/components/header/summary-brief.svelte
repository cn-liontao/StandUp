<script lang="ts">
	import { dayRecords } from '$store/day-records';
	import { dayRecordsInCalendarView, calendarState } from '$store/calendar';
	import { sumHoursInRecords } from '$lib/record-calc';

	$: daysCount = $dayRecords.length;
	$: hoursCount = sumHoursInRecords($dayRecords)
	$: hoursInView = sumHoursInRecords($dayRecordsInCalendarView)
	$: viewName = $calendarState.calendarView === 'month' ? '月度' : '季度'
</script>

<div class="flex items-center gap-4">
	<div class="flex flex-col">
		<div>{daysCount}</div>
		<div>天</div>
	</div>
	<div class="flex flex-col">
		<div>{hoursCount}h</div>
		<div>站立</div>
	</div>
	<div class="flex flex-col">
		<div>{hoursInView}h</div>
		<div>{viewName}总计</div>
	</div>
</div>