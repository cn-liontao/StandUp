import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/tauri'

export interface StandingRecord {
    start_time: number
    end_time: number
    duration: number
}

export interface DayRecord {
    date: number
    records: StandingRecord[]
}

export const dayRecords = writable<DayRecord[]>([])

invoke<DayRecord[]>('get_records').then((records) => {
    dayRecords.set(records)
})