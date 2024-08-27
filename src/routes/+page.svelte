<script lang="ts">
import { onMount } from "svelte";
import { dayRecords } from '$store/day-records';
import QuarterView from "$components/calendar/quarter-view.svelte";

onMount(() => {
	const listener = dayRecords.listen()

  return () => {
		listener.then(un => un())
  }
})
</script>

<QuarterView />
