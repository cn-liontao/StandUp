<script lang="ts">
import { onMount } from "svelte";
import { dayRecords } from '$store/day-records';
import { calendarState } from '$store/calendar';
import CalendarHeader from '$components/header/index.svelte'
import QuarterView from "$components/calendar/quarter-view.svelte";
import DailyView, { provideDailyView } from '$components/calendar/daily-view.svelte';
import Tooltip, { provideTooltip } from "$components/tooltip/index.svelte";

onMount(() => {
	const listener = Promise.all([dayRecords.listen(), calendarState.listen()])

  return () => {
		listener.then(un => un.map(i => i()))
  }
})

const tooltipContent = provideTooltip()
const dailyViewDate = provideDailyView()
</script>

<CalendarHeader />
<QuarterView />
<Tooltip content={$tooltipContent} />
<DailyView bind:dailyViewDate={$dailyViewDate} />