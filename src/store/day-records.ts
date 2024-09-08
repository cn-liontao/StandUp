import { derived, writable } from 'svelte/store';
import { last } from 'lodash-es'
import { startOfToday } from 'date-fns';
import { getRecordByDate } from '$lib/record-calc';
import { createSyncStore } from './sync'

export interface StandingRecord {
    start_time: number
    end_time: number
    duration: number
}

export interface DayRecord {
    date: number
    records: StandingRecord[]
}

function createDayRecords() {
    const { set, subscribe } = writable<DayRecord[]>([])

    return {
        subscribe,
        ...createSyncStore<DayRecord[]>({
            initCmd: 'get_records',
            updateEvent: 'records-update',
            update: set
        }),
    }
}

export const dayRecords = createDayRecords()

export const standing = derived([dayRecords], ([$dayRecords]) => {
    const todayRecord = getRecordByDate($dayRecords, startOfToday())
    if (todayRecord) {
        const latest = last(todayRecord.records.sort((a, b) => a.start_time - b.start_time))
        if (latest?.end_time === 0) {
            return latest.start_time
        }
        return 0
    }
    return 0
});

if (!import.meta.env.TEST) {
    dayRecords.init()
}
