<script lang="ts">
import { onMount } from "svelte";
import { dayRecords } from '$store/day-records';
import CalendarHeader from '$components/header/index.svelte'
import QuarterView from "$components/calendar/quarter-view.svelte";

onMount(() => {
	const listener = dayRecords.listen()

  return () => {
		listener.then(un => un())
  }
})
</script>

<CalendarHeader />
<QuarterView />
