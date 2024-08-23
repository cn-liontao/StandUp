<script lang="ts">
import { onMount } from "svelte";
import { type Event, listen } from '@tauri-apps/api/event';
import { type DayRecord, dayRecords } from '../store/day_records.js';
import QuarterView from "../components/calendar/quarter-view.svelte";

onMount(() => {
	const listener = listen('records-update', (event: Event<DayRecord[]>) => {
		console.log('backend update:', event);
		dayRecords.set(event.payload);
	})

  return () => {
		listener.then(un => un())
  }
})
</script>

<QuarterView />