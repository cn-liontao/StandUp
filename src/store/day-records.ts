import { writable } from 'svelte/store'
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

if (!import.meta.env.TEST) {
    dayRecords.init()
}
