import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/tauri'
import { type Event, listen } from '@tauri-apps/api/event';

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
        init: () => {
            invoke<DayRecord[]>('get_records').then((records) => {
                set(records)
            })
        },
        listen: () => {
            return listen('records-update', (event: Event<DayRecord[]>) => {
                console.log('backend update:', event);
                set(event.payload);
            })
        }
    }
}

export const dayRecords = createDayRecords()

if (!import.meta.env.TEST) {
    dayRecords.init()
}
